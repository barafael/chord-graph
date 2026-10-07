//! `chord-graph seed`: generates the voicing trie and writes it into the store, idempotently.

use crate::chords::{generate, symbols, Node, Symbol};
use crate::db::{open, Db};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use surrealdb::Surreal;

const BATCH: usize = 500;

fn ints(p: &[i64]) -> String {
    p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
}

fn strs(s: &[String]) -> String {
    s.iter().map(|x| format!("'{}'", x.replace('\'', "\\'"))).collect::<Vec<_>>().join(", ")
}

fn vid(p: &[i64]) -> String {
    format!("voicing:[{}]", ints(p))
}

fn sid(symbol: &str) -> String {
    format!("chord_symbol:⟨{}⟩", symbol)
}

fn symbol_row(s: &Symbol, run: i64) -> String {
    format!(
        "{{ id: {}, symbol: '{}', name: '{}', root: {}, pcs: [{}], gen: {} }}",
        sid(&s.symbol), s.symbol, s.name, s.root, ints(&s.pcs), run
    )
}

fn voicing_row(n: &Node, run: i64) -> String {
    let last = n.pitches.len() - 1;
    let parent = if last == 0 { "NONE".to_string() } else { vid(&n.pitches[..last]) };
    let mask: i64 = n.pitches.iter().map(|&p| 1i64 << p).sum();
    format!(
        "{{ id: {}, names: [{}], pitches: [{}], top: {}, size: {}, mask: {}, parent: {}, emotions: [{}], gen: {} }}",
        vid(&n.pitches), strs(&n.names), ints(&n.pitches), n.pitches[last], n.pitches.len(), mask, parent, strs(&n.emotions), run
    )
}

fn relation_rows(n: &Node, run: i64) -> Vec<String> {
    n.fits
        .iter()
        .enumerate()
        .map(|(rank, f)| {
            format!(
                "{{ id: voicing_symbol:[[{}], '{}'], in: {}, out: {}, fit: {}, root: {}, rank: {}, label: '{}', gen: {} }}",
                ints(&n.pitches), f.symbol, vid(&n.pitches), sid(&f.symbol), f.fit, f.root, rank, f.label, run
            )
        })
        .collect()
}

async fn insert(db: &Surreal<Db>, table: &str, rows: &[String], update: &str) -> surrealdb::Result<()> {
    let relation = if table == "voicing_symbol" { "RELATION " } else { "" };
    for chunk in rows.chunks(BATCH) {
        let q = format!("INSERT {}INTO {} [\n{}\n] ON DUPLICATE KEY UPDATE {};", relation, table, chunk.join(",\n"), update);
        db.query(q).await?.check()?;
    }
    Ok(())
}

pub async fn run(path: &str) -> surrealdb::Result<()> {
    let started = Instant::now();
    let db = open(path).await?;

    // Every row written by this run carries `gen`; rows left over from earlier runs are pruned afterwards.
    let run = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
    let syms = symbols();
    let nodes = generate();
    println!("generated {} symbols, {} voicings in {:.1?}", syms.len(), nodes.len(), started.elapsed());

    let symbol_rows: Vec<String> = syms.iter().map(|s| symbol_row(s, run)).collect();
    let voicing_rows: Vec<String> = nodes.iter().map(|n| voicing_row(n, run)).collect();
    let relation_rows: Vec<String> = nodes.iter().flat_map(|n| relation_rows(n, run)).collect();

    insert(&db, "chord_symbol", &symbol_rows, "symbol = $input.symbol, name = $input.name, root = $input.root, pcs = $input.pcs, gen = $input.gen").await?;
    insert(&db, "voicing", &voicing_rows, "names = $input.names, pitches = $input.pitches, top = $input.top, size = $input.size, mask = $input.mask, parent = $input.parent, emotions = $input.emotions, gen = $input.gen").await?;
    insert(&db, "voicing_symbol", &relation_rows, "fit = $input.fit, root = $input.root, rank = $input.rank, label = $input.label, gen = $input.gen").await?;

    let mut pruned = db
        .query("DELETE voicing_symbol WHERE gen != $gen RETURN BEFORE; DELETE voicing WHERE gen != $gen RETURN BEFORE; DELETE chord_symbol WHERE gen != $gen RETURN BEFORE;")
        .bind(("gen", run))
        .await?
        .check()?;
    let stale: usize = (0..3).map(|i| pruned.take::<Vec<serde_json::Value>>(i).map(|v| v.len()).unwrap_or(0)).sum();
    println!("seeded in {:.1?}, pruned {} stale rows", started.elapsed(), stale);

    let mut res = db
        .query(
            "SELECT size, count() AS voicings FROM voicing GROUP BY size ORDER BY size; \
             SELECT count() AS relations FROM voicing_symbol GROUP ALL; \
             SELECT pitches, names, emotions, ->voicing_symbol.{symbol: out.symbol, root, fit} AS fits FROM voicing WHERE pitches IN [[0, 4, 7], [0, 3, 8], [0, 4, 10]];",
        )
        .await?;
    let rings: Vec<serde_json::Value> = res.take(0)?;
    let rels: Vec<serde_json::Value> = res.take(1)?;
    let samples: Vec<serde_json::Value> = res.take(2)?;
    println!("rings: {}", serde_json::to_string(&rings).unwrap_or_default());
    println!("relations: {}", serde_json::to_string(&rels).unwrap_or_default());
    for s in &samples {
        println!("sample: {}", serde_json::to_string(s).unwrap_or_default());
    }
    Ok(())
}

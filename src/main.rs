mod chords;

use chords::{generate, CHORD_TYPES};
use surrealdb::engine::local::Db;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;

const SCHEMA: &str = include_str!("../schema.surql");

fn vid(p: &[i64]) -> String {
    format!("voicing:[{}]", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
}

async fn run_batches(db: &Surreal<Db>, stmts: &[String], chunk: usize) -> surrealdb::Result<()> {
    for c in stmts.chunks(chunk) {
        db.query(c.join(";\n")).await?.check()?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>("surreal.db").await?;
    db.use_ns("chord_graph").use_db("chord_graph").await?;
    db.query(SCHEMA).await?.check()?;

    let nodes = generate();

    let symbol_stmts: Vec<String> = CHORD_TYPES
        .iter()
        .map(|t| format!("UPSERT chord_symbol:⟨{}⟩ SET symbol = '{}';", t.symbol, t.symbol))
        .collect();

    let mut voicing_stmts: Vec<String> = Vec::new();
    for n in &nodes {
        let id = vid(&n.pitches);
        let last = n.pitches.len() - 1;
        let parent = if last == 0 { "NONE".to_string() } else { vid(&n.pitches[..last]) };
        let mask: i64 = n.pitches.iter().map(|&p| 1i64 << p).sum();
        let pitches = n.pitches.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
        let names = n.names.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ");
        let emotions = n.emotions.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ");
        voicing_stmts.push(format!(
            "UPSERT {} SET names = [{}], pitches = [{}], root = {}, top = {}, size = {}, mask = {}, parent = {}, emotions = [{}];",
            id, names, pitches, n.pitches[0], n.pitches[last], n.pitches.len(), mask, parent, emotions
        ));
    }

    let mut rel_stmts: Vec<String> = Vec::new();
    for n in &nodes {
        let id = vid(&n.pitches);
        for (sym, fit) in &n.fits {
            rel_stmts.push(format!("DELETE voicing_symbol WHERE in = {} AND out = chord_symbol:⟨{}⟩;", id, sym));
            rel_stmts.push(format!("RELATE {}->voicing_symbol->chord_symbol:⟨{}⟩ SET fit = {};", id, sym, fit));
        }
    }

    run_batches(&db, &symbol_stmts, 100).await?;
    run_batches(&db, &voicing_stmts, 100).await?;
    run_batches(&db, &rel_stmts, 100).await?;

    let mut res = db
        .query(
            "SELECT size, count() AS voicings FROM voicing GROUP BY size ORDER BY size; \
             SELECT count() AS relations FROM voicing_symbol GROUP ALL; \
             SELECT pitches, names, emotions, (SELECT VALUE { symbol: out.symbol, fit: fit } FROM voicing_symbol WHERE in = $parent.id) AS fits FROM voicing WHERE pitches = [0, 4, 7]; \
             SELECT pitches, names FROM voicing WHERE size = 7 LIMIT 3;",
        )
        .await?;
    let rings: Vec<serde_json::Value> = res.take(0)?;
    let rels: Vec<serde_json::Value> = res.take(1)?;
    let triad: Vec<serde_json::Value> = res.take(2)?;
    let deep: Vec<serde_json::Value> = res.take(3)?;
    println!("symbols: {}", CHORD_TYPES.len());
    println!("rings: {}", serde_json::to_string(&rings).unwrap_or_default());
    println!("relations: {}", serde_json::to_string(&rels).unwrap_or_default());
    println!("C major triad: {}", serde_json::to_string(&triad).unwrap_or_default());
    println!("ring 7 samples: {}", serde_json::to_string(&deep).unwrap_or_default());
    Ok(())
}

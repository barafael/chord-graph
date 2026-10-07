//! `chord-graph export`: writes the trie from the store into static JSON files for the app
//! (see [`api::shard_key`]): `index.json` plus one file per three-note prefix.

use crate::db::open;
use api::{join, shard_key, Row};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct ReadingRow {
    symbol: String,
    label: String,
    root: i64,
    fit: f64,
    rank: i64,
}

#[derive(Deserialize)]
struct VoicingRow {
    pitches: Vec<i64>,
    names: Vec<String>,
    emotions: Vec<String>,
    readings: Vec<ReadingRow>,
}

pub async fn run(path: &str, out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db = open(path).await?;
    let mut res = db
        .query("SELECT pitches, names, emotions, ->voicing_symbol.{symbol: out.symbol, label, root, fit, rank} AS readings FROM voicing;")
        .await?;
    let rows: Vec<VoicingRow> = res.take(0)?;
    let mut shards: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for mut v in rows {
        v.readings.sort_by_key(|r| r.rank);
        let row = Row {
            p: v.pitches.clone(),
            n: v.names,
            e: v.emotions,
            r: v.readings.into_iter().map(|r| (r.symbol, r.label, r.root, r.fit)).collect(),
        };
        if v.pitches.len() <= 3 {
            shards.entry("index".into()).or_default().push(row.clone());
        }
        if v.pitches.len() >= 3 {
            shards.entry(join(&v.pitches[..3])).or_default().push(row);
        }
    }
    std::fs::create_dir_all(out)?;
    let (mut files, mut bytes) = (0usize, 0usize);
    for (key, mut rows) in shards {
        rows.sort_by(|a, b| a.p.cmp(&b.p));
        debug_assert!(rows.iter().all(|r| shard_key(&r.p) == key || r.p.len() == 3));
        let json = serde_json::to_vec(&rows)?;
        bytes += json.len();
        files += 1;
        std::fs::write(format!("{out}/{key}.json"), json)?;
    }
    println!("wrote {files} files, {:.1} MB, to {out}", bytes as f64 / 1e6);
    Ok(())
}

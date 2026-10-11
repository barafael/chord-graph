//! chord-graph: a trie of chord voicings in an embedded SurrealDB store.
//!
//!   chord-graph seed   [--db surreal.db]
//!   chord-graph export [--db surreal.db] [--out crates/app/public/data]

mod chords;
mod db;
mod export;
mod seed;

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].clone())
        .unwrap_or_else(|| default.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let db = arg(&args, "--db", "surreal.db");
    match args.get(1).map(String::as_str) {
        Some("seed") => seed::run(&db).await?,
        Some("export") => export::run(&db, &arg(&args, "--out", "crates/app/public/data")).await?,
        _ => {
            eprintln!(
                "usage: chord-graph seed [--db PATH] | chord-graph export [--db PATH] [--out DIR]"
            );
            std::process::exit(2);
        }
    }
    Ok(())
}

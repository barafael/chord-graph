//! chord-graph: a trie of chord voicings in an embedded SurrealDB store.
//!
//!   chord-graph seed  [--db surreal.db]
//!   chord-graph serve [--db surreal.db] [--dist crates/app/dist] [--port 8090]

mod chords;
mod db;
mod seed;
mod serve;

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone()).unwrap_or_else(|| default.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let db = arg(&args, "--db", "surreal.db");
    match args.get(1).map(String::as_str) {
        Some("seed") => seed::run(&db).await?,
        Some("serve") => {
            let port: u16 = arg(&args, "--port", "8090").parse()?;
            serve::run(&db, &arg(&args, "--dist", "crates/app/dist"), port).await?;
        }
        _ => {
            eprintln!("usage: chord-graph seed [--db PATH] | chord-graph serve [--db PATH] [--dist DIR] [--port N]");
            std::process::exit(2);
        }
    }
    Ok(())
}

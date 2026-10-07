use surrealdb::engine::local::Db;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;

const SCHEMA: &str = include_str!("../schema.surql");
const SEED: &str = include_str!("../seed.surql");

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>("surreal.db").await?;
    db.use_ns("chord_graph").use_db("chord_graph").await?;

    db.query(SCHEMA).await?.check()?;
    db.query(SEED).await?.check()?;

    let mut res = db
        .query("SELECT <string> id AS id, pitches, size, top, mask, IF parent IS NONE THEN NONE ELSE <string> parent END AS parent, quality, strength FROM voicing ORDER BY size")
        .await?;
    let rows: Vec<serde_json::Value> = res.take(0)?;
    println!(
        "SurrealDB {} — schema ready, {} voicings:",
        db.version().await?,
        rows.len()
    );
    for row in rows {
        println!("{}", serde_json::to_string(&row).unwrap_or_default());
    }
    Ok(())
}

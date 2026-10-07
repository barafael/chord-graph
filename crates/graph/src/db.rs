//! Opening the embedded SurrealKV store with the schema applied.

use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;

pub use surrealdb::engine::local::Db;

const SCHEMA: &str = include_str!("../../../schema.surql");

pub async fn open(path: &str) -> surrealdb::Result<Surreal<Db>> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>(path).await?;
    db.use_ns("chord_graph").use_db("chord_graph").await?;
    db.query(SCHEMA).await?.check()?;
    Ok(db)
}

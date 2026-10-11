//! Opening the embedded SurrealKV store with the schema applied.

use surrealdb::Surreal;
use surrealdb::engine::local::SurrealKv;

pub use surrealdb::engine::local::Db;

const SCHEMA: &str = include_str!("../../../schema.surql");

#[allow(clippy::result_large_err)]
pub async fn open(path: &str) -> surrealdb::Result<Surreal<Db>> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>(path).await?;
    db.use_ns("chord_graph").use_db("chord_graph").await?;
    db.query(SCHEMA).await?.check()?;
    Ok(db)
}

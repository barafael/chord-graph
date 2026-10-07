//! `chord-graph serve`: a read-only JSON API over the store plus the built app as static files.
//!
//! `GET /api/voicing/0,4,7` answers with an [`api::Voicing`]: the voicing, its ranked readings, its parent
//! and its children (one per note that can be added).

use crate::db::{open, Db};
use api::{note_name, Child, Reading, Voicing};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::Surreal;
use tower_http::services::{ServeDir, ServeFile};

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

fn readings(mut rows: Vec<ReadingRow>) -> Vec<Reading> {
    rows.sort_by_key(|r| r.rank);
    rows.into_iter().map(|r| Reading { symbol: r.symbol, label: r.label, root: r.root, fit: r.fit }).collect()
}

fn parse_pitches(s: &str) -> Option<Vec<i64>> {
    let p: Vec<i64> = s.split(',').map(|x| x.trim().parse().ok()).collect::<Option<_>>()?;
    let ok = p.first() == Some(&0) && p.windows(2).all(|w| w[0] < w[1]) && p.iter().all(|&x| (0..=36).contains(&x));
    ok.then_some(p)
}

fn record_id(p: &[i64]) -> String {
    format!("voicing:[{}]", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
}

async fn voicing(State(db): State<Arc<Surreal<Db>>>, Path(pitches): Path<String>) -> Result<Json<Voicing>, StatusCode> {
    let pitches = parse_pitches(&pitches).ok_or(StatusCode::BAD_REQUEST)?;
    let id = record_id(&pitches);
    let fields = "pitches, names, emotions, ->voicing_symbol.{symbol: out.symbol, label, root, fit, rank} AS readings";
    let mut res = db
        .query(format!("SELECT {fields} FROM {id}; SELECT {fields} FROM voicing WHERE parent = {id} ORDER BY pitches;"))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let node: Option<VoicingRow> = res.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let node = node.ok_or(StatusCode::NOT_FOUND)?;
    let kids: Vec<VoicingRow> = res.take(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut children: Vec<Child> = kids
        .into_iter()
        .map(|k| {
            let pitch = *k.pitches.last().unwrap_or(&0);
            Child { pitch, note: note_name(pitch), names: k.names, emotions: k.emotions, readings: readings(k.readings) }
        })
        .collect();
    children.sort_by_key(|c| c.pitch);
    let parent = (pitches.len() > 1).then(|| pitches[..pitches.len() - 1].to_vec());
    Ok(Json(Voicing {
        notes: pitches.iter().map(|&p| note_name(p)).collect(),
        pitches: node.pitches,
        names: node.names,
        emotions: node.emotions,
        readings: readings(node.readings),
        parent,
        children,
    }))
}

pub async fn run(path: &str, dist: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(open(path).await?);
    let index = format!("{dist}/index.html");
    let app = Router::new()
        .route("/api/voicing/{pitches}", get(voicing))
        .fallback_service(ServeDir::new(dist).fallback(ServeFile::new(index)))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
    println!("serving {path} and {dist} on http://127.0.0.1:{port}");
    axum::serve(listener, app).await?;
    Ok(())
}

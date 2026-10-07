//! The JSON shapes served at `/api/voicing/{pitches}` and consumed by the app.

use serde::{Deserialize, Serialize};

/// One ranked reading of a voicing as a chord symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    pub symbol: String,
    /// The reading spelled out, with omissions and doublings: "C major 6 (no A, doubled C)".
    pub label: String,
    /// Semitones from the bass up to the chord root.
    pub root: i64,
    pub fit: f64,
}

/// A voicing one note larger than the current one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Child {
    /// The note that is added, in semitones above the bass.
    pub pitch: i64,
    /// Note name with octave marks: "E", "E′", "C″".
    pub note: String,
    pub names: Vec<String>,
    pub emotions: Vec<String>,
    pub readings: Vec<Reading>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Voicing {
    pub pitches: Vec<i64>,
    pub notes: Vec<String>,
    pub names: Vec<String>,
    pub emotions: Vec<String>,
    pub readings: Vec<Reading>,
    pub parent: Option<Vec<i64>>,
    pub children: Vec<Child>,
}

pub const TONES: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

/// "E", "E′", "C″", "C‴" for 4, 16, 24, 36.
pub fn note_name(pitch: i64) -> String {
    let marks = match pitch / 12 { 0 => "", 1 => "′", 2 => "″", _ => "‴" };
    format!("{}{}", TONES[(pitch % 12) as usize], marks)
}

/// One voicing as stored in the static data files: pitches, names, emotions and ranked readings as
/// `[symbol, label, root, fit]` tuples. Field names are short because there are 60k of these.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    pub p: Vec<i64>,
    pub n: Vec<String>,
    pub e: Vec<String>,
    pub r: Vec<(String, String, i64, f64)>,
}

/// The data file that holds a voicing and its children. Voicings of up to three notes live in `index`;
/// deeper ones are sharded by their first three notes ("0,4,7"), and the three-note voicing itself is
/// repeated in its shard so one file answers any request.
pub fn shard_key(pitches: &[i64]) -> String {
    if pitches.len() <= 2 { "index".to_string() } else { join(&pitches[..3]) }
}

pub fn join(pitches: &[i64]) -> String {
    pitches.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}

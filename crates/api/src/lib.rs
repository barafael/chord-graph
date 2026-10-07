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

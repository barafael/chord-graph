use std::collections::{BTreeMap, BTreeSet};

pub const TONES: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

const INTERVALS: [&str; 12] = [
    "unison", "minor 2nd", "major 2nd", "minor 3rd", "major 3rd", "perfect 4th", "tritone",
    "perfect 5th", "minor 6th", "major 6th", "minor 7th", "major 7th",
];

/// Highest pitch (semitones above the bass) a voicing may contain.
pub const RANGE: i64 = 24;
/// Largest voicing (number of notes) that is generated.
pub const MAX_NOTES: usize = 7;
/// Readings below this fit are not attributed to a chord symbol at all.
pub const MIN_FIT: f64 = 0.5;
/// Two-note voicings are intervals first; only a missing fifth may separate them from a chord symbol.
pub const MIN_FIT_DYAD: f64 = 0.8;
/// How many ranked readings are kept per voicing.
pub const MAX_FITS: usize = 4;

pub struct ChordType {
    pub symbol: &'static str,
    pub name: &'static str,
    /// Pitch classes relative to the chord root.
    pub pcs: &'static [i64],
    pub emotions: &'static [&'static str],
}

/// A chord symbol as stored: absolute, with C as the bass note.
pub struct Symbol {
    pub symbol: String,
    pub name: String,
    /// Semitones from the bass (C) up to the chord root.
    pub root: i64,
    /// Pitch classes relative to the bass, sorted.
    pub pcs: Vec<i64>,
}

pub struct Fit {
    pub symbol: String,
    pub root: i64,
    pub fit: f64,
}

pub struct Node {
    pub pitches: Vec<i64>,
    pub names: Vec<String>,
    pub fits: Vec<Fit>,
    pub emotions: Vec<String>,
}

/// Chord vocabulary, spelled from C. Every chord tone may also serve as the bass (inversions).
pub const CHORD_TYPES: &[ChordType] = &[
    // triads and dyads
    ChordType { symbol: "C", name: "C major", pcs: &[0, 4, 7], emotions: &["bright", "stable"] },
    ChordType { symbol: "C5", name: "C power chord", pcs: &[0, 7], emotions: &["raw"] },
    ChordType { symbol: "Cm", name: "C minor", pcs: &[0, 3, 7], emotions: &["somber"] },
    ChordType { symbol: "Cdim", name: "C diminished", pcs: &[0, 3, 6], emotions: &["unstable"] },
    ChordType { symbol: "Caug", name: "C augmented", pcs: &[0, 4, 8], emotions: &["eerie"] },
    ChordType { symbol: "Csus2", name: "C suspended 2nd", pcs: &[0, 2, 7], emotions: &["floating"] },
    ChordType { symbol: "Csus4", name: "C suspended 4th", pcs: &[0, 5, 7], emotions: &["yearning"] },
    // added-tone chords
    ChordType { symbol: "C6", name: "C major 6", pcs: &[0, 4, 7, 9], emotions: &["warm"] },
    ChordType { symbol: "Cm6", name: "C minor 6", pcs: &[0, 3, 7, 9], emotions: &["wistful"] },
    ChordType { symbol: "Cm(b6)", name: "C minor flat 6", pcs: &[0, 3, 7, 8], emotions: &["dark"] },
    ChordType { symbol: "Cadd9", name: "C added 9th", pcs: &[0, 2, 4, 7], emotions: &["open"] },
    ChordType { symbol: "Cmadd9", name: "C minor added 9th", pcs: &[0, 2, 3, 7], emotions: &["pensive"] },
    ChordType { symbol: "Cadd11", name: "C added 11th", pcs: &[0, 4, 5, 7], emotions: &[] },
    ChordType { symbol: "Cmadd11", name: "C minor added 11th", pcs: &[0, 3, 5, 7], emotions: &[] },
    ChordType { symbol: "C6/9", name: "C six nine", pcs: &[0, 2, 4, 7, 9], emotions: &["sunny"] },
    ChordType { symbol: "Cm6/9", name: "C minor six nine", pcs: &[0, 2, 3, 7, 9], emotions: &[] },
    // sevenths
    ChordType { symbol: "C7", name: "C dominant 7", pcs: &[0, 4, 7, 10], emotions: &["tense"] },
    ChordType { symbol: "Cmaj7", name: "C major 7", pcs: &[0, 4, 7, 11], emotions: &["lush"] },
    ChordType { symbol: "Cm7", name: "C minor 7", pcs: &[0, 3, 7, 10], emotions: &["mellow"] },
    ChordType { symbol: "CmMaj7", name: "C minor major 7", pcs: &[0, 3, 7, 11], emotions: &["noir"] },
    ChordType { symbol: "Cm7b5", name: "C half-diminished", pcs: &[0, 3, 6, 10], emotions: &["anxious"] },
    ChordType { symbol: "Cdim7", name: "C diminished 7", pcs: &[0, 3, 6, 9], emotions: &["ominous"] },
    ChordType { symbol: "C7b5", name: "C dominant 7 flat 5", pcs: &[0, 4, 6, 10], emotions: &[] },
    ChordType { symbol: "C7#5", name: "C dominant 7 sharp 5", pcs: &[0, 4, 8, 10], emotions: &[] },
    ChordType { symbol: "Cmaj7#5", name: "C major 7 sharp 5", pcs: &[0, 4, 8, 11], emotions: &["luminous"] },
    ChordType { symbol: "C7sus4", name: "C dominant 7 suspended 4th", pcs: &[0, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "C7sus2", name: "C dominant 7 suspended 2nd", pcs: &[0, 2, 7, 10], emotions: &[] },
    ChordType { symbol: "Cmaj7sus4", name: "C major 7 suspended 4th", pcs: &[0, 5, 7, 11], emotions: &[] },
    ChordType { symbol: "Cmaj7sus2", name: "C major 7 suspended 2nd", pcs: &[0, 2, 7, 11], emotions: &[] },
    // ninths
    ChordType { symbol: "Cmaj9", name: "C major 9", pcs: &[0, 2, 4, 7, 11], emotions: &["dreamy"] },
    ChordType { symbol: "C9", name: "C dominant 9", pcs: &[0, 2, 4, 7, 10], emotions: &["funky"] },
    ChordType { symbol: "Cm9", name: "C minor 9", pcs: &[0, 2, 3, 7, 10], emotions: &["smooth"] },
    ChordType { symbol: "Cm9maj7", name: "C minor 9 major 7", pcs: &[0, 2, 3, 7, 11], emotions: &[] },
    ChordType { symbol: "C9sus4", name: "C dominant 9 suspended 4th", pcs: &[0, 2, 5, 7, 10], emotions: &["suspended"] },
    ChordType { symbol: "C7b9", name: "C dominant 7 flat 9", pcs: &[0, 1, 4, 7, 10], emotions: &["restless"] },
    ChordType { symbol: "C7#9", name: "C dominant 7 sharp 9", pcs: &[0, 3, 4, 7, 10], emotions: &["gritty"] },
    ChordType { symbol: "C7sus4b9", name: "C dominant 7 suspended 4th flat 9", pcs: &[0, 1, 5, 7, 10], emotions: &["exotic"] },
    ChordType { symbol: "C9b5", name: "C dominant 9 flat 5", pcs: &[0, 2, 4, 6, 10], emotions: &[] },
    ChordType { symbol: "C9#5", name: "C dominant 9 sharp 5", pcs: &[0, 2, 4, 8, 10], emotions: &[] },
    // altered dominants
    ChordType { symbol: "C7b9b5", name: "C dominant 7 flat 9 flat 5", pcs: &[0, 1, 4, 6, 10], emotions: &["altered"] },
    ChordType { symbol: "C7b9#5", name: "C dominant 7 flat 9 sharp 5", pcs: &[0, 1, 4, 8, 10], emotions: &["altered"] },
    ChordType { symbol: "C7#9b5", name: "C dominant 7 sharp 9 flat 5", pcs: &[0, 3, 4, 6, 10], emotions: &["altered"] },
    ChordType { symbol: "C7#9#5", name: "C dominant 7 sharp 9 sharp 5", pcs: &[0, 3, 4, 8, 10], emotions: &["altered"] },
    // elevenths
    ChordType { symbol: "Cmaj7#11", name: "C major 7 sharp 11", pcs: &[0, 4, 6, 7, 11], emotions: &["lydian"] },
    ChordType { symbol: "C7#11", name: "C dominant 7 sharp 11", pcs: &[0, 4, 6, 7, 10], emotions: &[] },
    ChordType { symbol: "Cm11", name: "C minor 11", pcs: &[0, 2, 3, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "C11", name: "C dominant 11", pcs: &[0, 2, 4, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "Cmaj9#11", name: "C major 9 sharp 11", pcs: &[0, 2, 4, 6, 7, 11], emotions: &[] },
    ChordType { symbol: "C9#11", name: "C dominant 9 sharp 11", pcs: &[0, 2, 4, 6, 7, 10], emotions: &[] },
    // thirteenths
    ChordType { symbol: "Cmaj13", name: "C major 13", pcs: &[0, 2, 4, 7, 9, 11], emotions: &[] },
    ChordType { symbol: "C13", name: "C dominant 13", pcs: &[0, 2, 4, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cm13", name: "C minor 13", pcs: &[0, 2, 3, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "C13b9", name: "C dominant 13 flat 9", pcs: &[0, 1, 4, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "C13sus4", name: "C dominant 13 suspended 4th", pcs: &[0, 2, 5, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "C13#11", name: "C dominant 13 sharp 11", pcs: &[0, 2, 4, 6, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cmaj13(11)", name: "C major 13 complete", pcs: &[0, 2, 4, 5, 7, 9, 11], emotions: &[] },
    ChordType { symbol: "C13(9,11)", name: "C dominant 13 complete", pcs: &[0, 2, 4, 5, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cm13(9,11)", name: "C minor 13 complete", pcs: &[0, 2, 3, 5, 7, 9, 10], emotions: &[] },
];

/// Cost of leaving out a chord tone (interval relative to the chord root), or `None` when the
/// tone defines the symbol and a reading without it would be a different chord.
fn omission_cost(t: &ChordType, iv: i64) -> Option<f64> {
    let has = |x: i64| t.pcs.contains(&x);
    let third = has(3) || has(4);
    let seventh = has(10) || has(11);
    match iv {
        3 | 4 if has(3) && has(4) => None,                         // #9 chords need both the 3rd and the #9
        3 | 4 => Some(0.3),                                        // the third (shell and rootless voicings drop it)
        7 => Some(0.1),                                            // perfect fifth, routinely omitted
        2 if (has(5) && third) || (has(9) && seventh) => Some(0.15), // 9th inside an 11th or 13th chord
        5 if third && has(9) && seventh => Some(0.15),             // 11th inside a complete 13th chord
        _ => None,                                                  // root, 7th, 6th, 9th, 11th, 13th, sus, altered and added tones
    }
}

/// One way of reading a chord type: chord tone `bass` (relative to the root) placed in the bass.
struct Reading {
    t: &'static ChordType,
    /// Offset from bass up to root.
    root: i64,
    /// Pitch classes relative to the bass, sorted.
    pcs: Vec<i64>,
}

fn readings() -> Vec<Reading> {
    let mut out = Vec::new();
    for t in CHORD_TYPES {
        for &bass in t.pcs {
            let root = (12 - bass) % 12;
            let mut pcs: Vec<i64> = t.pcs.iter().map(|p| (p - bass).rem_euclid(12)).collect();
            pcs.sort();
            out.push(Reading { t, root, pcs });
        }
    }
    out
}

fn reading_symbol(r: &Reading) -> String {
    if r.root == 0 { r.t.symbol.to_string() } else { format!("{}{}/C", TONES[r.root as usize], &r.t.symbol[1..]) }
}

fn reading_name(r: &Reading) -> String {
    if r.root == 0 { r.t.name.to_string() } else { format!("{} {} over C", TONES[r.root as usize], &r.t.name[2..]) }
}

pub fn symbols() -> Vec<Symbol> {
    readings()
        .iter()
        .map(|r| Symbol { symbol: reading_symbol(r), name: reading_name(r), root: r.root, pcs: r.pcs.clone() })
        .collect()
}

fn nonempty_subsets(slots: &[i64]) -> Vec<Vec<i64>> {
    (1..(1 << slots.len()))
        .map(|mask| slots.iter().enumerate().filter(|(i, _)| mask & (1 << i) != 0).map(|(_, &p)| p).collect())
        .collect()
}

/// All ways of realising a pitch-class set (bass included) across the range, each class at least once.
fn expand(pcs: &[i64]) -> Vec<Vec<i64>> {
    let mut acc: Vec<Vec<i64>> = vec![Vec::new()];
    for &pc in pcs {
        let slots: Vec<i64> = (0..).map(|o| pc + 12 * o).take_while(|&p| p <= RANGE).collect();
        let subs: Vec<Vec<i64>> = nonempty_subsets(&slots).into_iter().filter(|s| pc != 0 || s.contains(&0)).collect();
        let mut next = Vec::new();
        for base in &acc {
            for s in &subs {
                if base.len() + s.len() <= MAX_NOTES {
                    let mut v = base.clone();
                    v.extend_from_slice(s);
                    next.push(v);
                }
            }
        }
        acc = next;
    }
    acc.into_iter().map(|mut v| { v.sort(); v }).collect()
}

fn classify(pitches: &[i64], readings: &[Reading]) -> Node {
    let vpcs: BTreeSet<i64> = pitches.iter().map(|&p| p % 12).collect();
    let doubling: Vec<String> = (0..12)
        .filter(|&pc| pitches.iter().filter(|&&p| p % 12 == pc).count() > 1)
        .map(|pc| format!("doubled {}", TONES[pc as usize]))
        .collect();
    let with_notes = |base: String, mut parts: Vec<String>| {
        parts.extend(doubling.iter().cloned());
        if parts.is_empty() { base } else { format!("{} ({})", base, parts.join(", ")) }
    };

    if vpcs.len() == 1 {
        let name = if pitches.len() == 1 { "C" } else { "C octaves" };
        return Node { pitches: pitches.to_vec(), names: vec![name.into()], fits: vec![], emotions: vec![] };
    }

    // Score every reading whose root is present in the voicing and whose tones cover it.
    let mut scored: Vec<(&Reading, f64, Vec<String>)> = Vec::new();
    for r in readings {
        if !vpcs.iter().all(|pc| r.pcs.contains(pc)) {
            continue;
        }
        if let Some(fit) = subset_fit(r, &vpcs) {
            let missing = r.pcs.iter().filter(|pc| !vpcs.contains(pc)).map(|&pc| format!("no {}", TONES[pc as usize])).collect();
            scored.push((r, fit, missing));
        }
    }
    // Exact readings first (root position before inversions); among incomplete readings, root
    // position wins before fit does, so "C7 (no E)" outranks "Gm(add11)/C (no D)".
    scored.sort_by(|a, b| {
        (a.1 < 1.0).cmp(&(b.1 < 1.0))
            .then((a.0.root != 0).cmp(&(b.0.root != 0)))
            .then(b.1.partial_cmp(&a.1).unwrap())
            .then(a.0.t.symbol.cmp(b.0.t.symbol))
    });
    scored.truncate(MAX_FITS);

    let mut names: Vec<String> = Vec::new();
    if vpcs.len() == 2 {
        let upper = *vpcs.iter().find(|&&p| p != 0).unwrap();
        names.push(with_notes(format!("{} (C, {})", INTERVALS[upper as usize], TONES[upper as usize]), vec![]));
    }
    if scored.is_empty() && vpcs.len() > 2 {
        let notes: Vec<String> = vpcs.iter().filter(|&&p| p != 0).map(|&p| TONES[p as usize].to_string()).collect();
        names.push(with_notes("C".into(), notes));
    }
    for (r, _, missing) in &scored {
        names.push(with_notes(reading_name(r), missing.clone()));
    }

    let mut emotions: Vec<String> = Vec::new();
    for (r, fit, _) in &scored {
        if *fit >= 1.0 {
            for e in r.t.emotions {
                if !emotions.iter().any(|x| x == e) {
                    emotions.push(e.to_string());
                }
            }
        }
    }
    let fits = scored.iter().map(|(r, fit, _)| Fit { symbol: reading_symbol(r), root: r.root, fit: *fit }).collect();
    Node { pitches: pitches.to_vec(), names, fits, emotions }
}

/// Fit of a reading realised by exactly the pitch classes in `present`, or `None` if a defining
/// tone is missing or the fit falls under the threshold.
fn subset_fit(r: &Reading, present: &BTreeSet<i64>) -> Option<f64> {
    if !present.contains(&0) || !present.contains(&r.root) {
        return None;
    }
    let mut fit = 1.0;
    for &iv in r.t.pcs {
        let abs = (iv - (12 - r.root)).rem_euclid(12);
        if !present.contains(&abs) {
            fit -= omission_cost(r.t, iv)?;
        }
    }
    let fit = (fit * 100.0).round() / 100.0;
    let min = if present.len() == 2 { MIN_FIT_DYAD } else { MIN_FIT };
    (fit >= min).then_some(fit)
}

/// Every voicing that has at least one chord reading (complete or with permitted omissions),
/// plus all of its trie prefixes, classified.
pub fn generate() -> Vec<Node> {
    let readings = readings();
    let mut set: BTreeMap<Vec<i64>, ()> = BTreeMap::new();
    for r in &readings {
        for sub in nonempty_subsets(&r.pcs) {
            let present: BTreeSet<i64> = sub.iter().copied().collect();
            if subset_fit(r, &present).is_none() {
                continue;
            }
            for v in expand(&sub) {
                for k in 1..=v.len() {
                    set.insert(v[..k].to_vec(), ());
                }
            }
        }
    }
    set.keys().map(|p| classify(p, &readings)).collect()
}

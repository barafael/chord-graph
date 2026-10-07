const TONES: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

pub struct ChordType {
    pub symbol: &'static str,
    pub name: &'static str,
    pub pcs: &'static [i64],
    pub emotions: &'static [&'static str],
}

pub struct Node {
    pub pitches: Vec<i64>,
    pub names: Vec<String>,
    pub fits: Vec<(String, f64)>,
    pub emotions: Vec<String>,
}

pub const CHORD_TYPES: &[ChordType] = &[
    ChordType { symbol: "C", name: "C major", pcs: &[0, 4, 7], emotions: &["bright", "stable"] },
    ChordType { symbol: "C5", name: "C power chord", pcs: &[0, 7], emotions: &["raw"] },
    ChordType { symbol: "Cm", name: "C minor", pcs: &[0, 3, 7], emotions: &["somber"] },
    ChordType { symbol: "Cdim", name: "C diminished", pcs: &[0, 3, 6], emotions: &["unstable"] },
    ChordType { symbol: "Caug", name: "C augmented", pcs: &[0, 4, 8], emotions: &["eerie"] },
    ChordType { symbol: "Csus2", name: "C suspended 2nd", pcs: &[0, 2, 7], emotions: &["floating"] },
    ChordType { symbol: "Csus4", name: "C suspended 4th", pcs: &[0, 5, 7], emotions: &["yearning"] },
    ChordType { symbol: "C6", name: "C major 6", pcs: &[0, 4, 7, 9], emotions: &["warm"] },
    ChordType { symbol: "Cm6", name: "C minor 6", pcs: &[0, 3, 7, 9], emotions: &[] },
    ChordType { symbol: "C7", name: "C dominant 7", pcs: &[0, 4, 7, 10], emotions: &["tense"] },
    ChordType { symbol: "Cmaj7", name: "C major 7", pcs: &[0, 4, 7, 11], emotions: &["lush"] },
    ChordType { symbol: "Cm7", name: "C minor 7", pcs: &[0, 3, 7, 10], emotions: &["mellow"] },
    ChordType { symbol: "CmMaj7", name: "C minor major 7", pcs: &[0, 3, 7, 11], emotions: &["noir"] },
    ChordType { symbol: "Cm7b5", name: "C half-diminished", pcs: &[0, 3, 6, 10], emotions: &[] },
    ChordType { symbol: "Cdim7", name: "C diminished 7", pcs: &[0, 3, 6, 9], emotions: &["ominous"] },
    ChordType { symbol: "C7sus4", name: "C dominant 7 suspended 4th", pcs: &[0, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "Cadd9", name: "C added 9th", pcs: &[0, 2, 4, 7], emotions: &[] },
    ChordType { symbol: "Cmadd9", name: "C minor added 9th", pcs: &[0, 2, 3, 7], emotions: &[] },
    ChordType { symbol: "Cadd11", name: "C added 11th", pcs: &[0, 4, 5, 7], emotions: &[] },
    ChordType { symbol: "C6/9", name: "C six nine", pcs: &[0, 2, 4, 7, 9], emotions: &[] },
    ChordType { symbol: "Cmaj9", name: "C major 9", pcs: &[0, 2, 4, 7, 11], emotions: &["dreamy"] },
    ChordType { symbol: "C9", name: "C dominant 9", pcs: &[0, 2, 4, 7, 10], emotions: &[] },
    ChordType { symbol: "Cm9", name: "C minor 9", pcs: &[0, 2, 3, 7, 10], emotions: &[] },
    ChordType { symbol: "C7b9", name: "C dominant 7 flat 9", pcs: &[0, 1, 4, 7, 10], emotions: &["restless"] },
    ChordType { symbol: "C7#9", name: "C dominant 7 sharp 9", pcs: &[0, 3, 4, 7, 10], emotions: &[] },
    ChordType { symbol: "C7b5", name: "C dominant 7 flat 5", pcs: &[0, 4, 6, 10], emotions: &[] },
    ChordType { symbol: "C7#5", name: "C dominant 7 sharp 5", pcs: &[0, 4, 8, 10], emotions: &[] },
    ChordType { symbol: "Cmaj7#11", name: "C major 7 sharp 11", pcs: &[0, 4, 6, 7, 11], emotions: &[] },
    ChordType { symbol: "Cm11", name: "C minor 11", pcs: &[0, 3, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "C11", name: "C dominant 11", pcs: &[0, 2, 5, 7, 10], emotions: &[] },
    ChordType { symbol: "Cmaj13", name: "C major 13", pcs: &[0, 4, 7, 9, 11], emotions: &[] },
    ChordType { symbol: "C13", name: "C dominant 13", pcs: &[0, 4, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cm13", name: "C minor 13", pcs: &[0, 3, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cmaj9#11", name: "C major 9 sharp 11", pcs: &[0, 2, 4, 6, 7, 11], emotions: &[] },
    ChordType { symbol: "C13#11", name: "C dominant 13 sharp 11", pcs: &[0, 4, 6, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "C9#11", name: "C dominant 9 sharp 11", pcs: &[0, 2, 4, 6, 7, 10], emotions: &[] },
    ChordType { symbol: "Cm9maj7", name: "C minor 9 major 7", pcs: &[0, 2, 3, 7, 11], emotions: &[] },
    ChordType { symbol: "Cmaj13(11)", name: "C major 13 complete", pcs: &[0, 2, 4, 5, 7, 9, 11], emotions: &[] },
    ChordType { symbol: "C13(9,11)", name: "C dominant 13 complete", pcs: &[0, 2, 4, 5, 7, 9, 10], emotions: &[] },
    ChordType { symbol: "Cm13(9,11)", name: "C minor 13 complete", pcs: &[0, 2, 3, 5, 7, 9, 10], emotions: &[] },
];

fn nonempty_subsets(slots: &[i64]) -> Vec<Vec<i64>> {
    let mut out = Vec::new();
    for mask in 1..(1 << slots.len()) {
        let mut s = Vec::new();
        for (i, &p) in slots.iter().enumerate() {
            if mask & (1 << i) != 0 {
                s.push(p);
            }
        }
        out.push(s);
    }
    out
}

fn expand(pcs: &[i64]) -> Vec<Vec<i64>> {
    let mut acc: Vec<Vec<i64>> = vec![Vec::new()];
    for &pc in pcs {
        let slots: Vec<i64> = [pc, pc + 12, pc + 24].into_iter().filter(|&p| p <= 24).collect();
        let subs: Vec<Vec<i64>> = nonempty_subsets(&slots)
            .into_iter()
            .filter(|s| pc != 0 || s.contains(&0))
            .collect();
        let mut next = Vec::new();
        for base in &acc {
            for s in &subs {
                let mut v = base.clone();
                v.extend_from_slice(s);
                if v.len() <= 7 {
                    next.push(v);
                }
            }
        }
        acc = next;
    }
    acc.into_iter().map(|mut v| { v.sort(); v }).collect()
}

fn classify(pitches: &[i64]) -> Node {
    let vpcs: std::collections::BTreeSet<i64> = pitches.iter().map(|&p| p % 12).collect();
    let mut doubling: Vec<String> = Vec::new();
    for (i, tone) in TONES.iter().enumerate() {
        let c = pitches.iter().filter(|&&p| (p % 12) as usize == i).count();
        if c > 1 {
            doubling.push(format!("doubled {}", tone));
        }
    }
    if vpcs.len() == 1 {
        return Node {
            pitches: pitches.to_vec(),
            names: vec![if pitches.len() == 1 { "C".into() } else { "C octaves".into() }],
            fits: vec![],
            emotions: vec![],
        };
    }
    let mut fits: Vec<(&ChordType, f64)> = Vec::new();
    let mut exact: Option<&ChordType> = None;
    for t in CHORD_TYPES {
        let tset: std::collections::BTreeSet<i64> = t.pcs.iter().copied().collect();
        if vpcs == tset {
            exact = Some(t);
            fits.push((t, 1.0));
        } else if vpcs.is_subset(&tset) {
            fits.push((t, 1.0 - 0.15 * (tset.len() - vpcs.len()) as f64));
        }
    }
    fits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.symbol.cmp(b.0.symbol)));
    fits.truncate(3);

    if fits.is_empty() {
        let mut parts: Vec<String> = vpcs.iter().filter(|&&p| p != 0).map(|&p| TONES[p as usize].to_string()).collect();
        parts.extend(doubling);
        let name = if parts.is_empty() { "C".to_string() } else { format!("C ({})", parts.join(", ")) };
        return Node {
            pitches: pitches.to_vec(),
            names: vec![name],
            fits: vec![],
            emotions: vec![],
        };
    }

    let mut names: Vec<String> = Vec::new();
    for (t, _) in &fits {
        let tset: std::collections::BTreeSet<i64> = t.pcs.iter().copied().collect();
        let mut parts: Vec<String> = Vec::new();
        for m in tset.difference(&vpcs) {
            parts.push(format!("no {}", TONES[*m as usize]));
        }
        parts.extend(doubling.iter().cloned());
        names.push(if parts.is_empty() { t.name.to_string() } else { format!("{} ({})", t.name, parts.join(", ")) });
    }
    let emotions: Vec<String> = exact.map(|t| t.emotions.iter().map(|s| s.to_string()).collect()).unwrap_or_default();
    let fits_out = fits.iter().map(|(t, f)| (t.symbol.to_string(), *f)).collect();
    Node { pitches: pitches.to_vec(), names, fits: fits_out, emotions }
}

pub fn generate() -> Vec<Node> {
    let mut set = std::collections::BTreeMap::new();
    for t in CHORD_TYPES {
        for v in expand(t.pcs) {
            for k in 1..=v.len() {
                set.insert(v[..k].to_vec(), ());
            }
        }
    }
    set.keys().map(|p| classify(p)).collect()
}

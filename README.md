# chord-graph

A trie of chord voicings in an embedded SurrealDB store, and a browser app for walking it one note at a time.

```
schema.surql          the SurrealDB schema (chord_symbol, voicing, voicing_symbol)
crates/graph          `chord-graph seed` generates the trie; `chord-graph serve` exposes it as JSON + serves the app
crates/api            the JSON types shared by server and app
crates/app            the Leptos (client-side) app, built with Trunk
surreal.db/           the SurrealKV store (gitignored, derived: rebuild it with `seed`)
```

## Build and run

```sh
cargo run --release -p chord-graph -- seed            # a few minutes; idempotent
(cd crates/app && trunk build --release)              # → crates/app/dist
cargo run --release -p chord-graph -- serve           # http://127.0.0.1:8090
```

For development, `chord-graph serve` in one terminal and `trunk serve` (port 8091, proxies `/api`) in
`crates/app` in another gives live reloads of the app.

## The data

Every voicing is a sorted list of semitones above the bass note C, within two octaves (0–24), at most seven
notes; a few curated named voicings reach a third octave. The trie's edges add one higher note. A voicing
carries its ranked `names`, the `emotions` of its exact readings, and `voicing_symbol` relations to the chord
symbols it can be read as, with `fit`, the chord `root` above the bass, a `rank` and a spelled-out `label`.
Chord symbols are absolute with C in the bass: `C`, `Cm7`, `Ab/C`, `Fm7/C`.

Fit is 1.0 for a complete chord. Omitting the 3rd costs 0.3, the 5th 0.1, the 9th inside an 11th or 13th
chord and the 11th inside a complete 13th 0.15; any other tone defines the symbol and cannot be left out.
A rootless reading (the jazz "rootless voicing": 3rd, 7th and a colour tone present, root absent) costs 0.35.

Sources for the vocabulary and the named voicings: Wikipedia "List of chords", "Jazz chord", "Upper
structure", "Quartal and quintal harmony", "Petrushka chord"; pianogroove.com (Kenny Barron voicing);
pianowithjonny.com (rootless A/B voicings). Emotions follow conventional descriptions in songwriting
guides and Lahdelma & Eerola's listener study; chords without a sourced description have none.

The vocabulary, the fit model and the named voicings live in `crates/graph/src/chords.rs`.

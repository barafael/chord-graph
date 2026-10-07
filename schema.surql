-- Chord symbols: absolute spellings with C as the bass note (C, Cm7, Ab/C, Fm7/C, ...).
DEFINE TABLE IF NOT EXISTS chord_symbol SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS symbol ON TABLE chord_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS name ON TABLE chord_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS root ON TABLE chord_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD IF NOT EXISTS pcs ON TABLE chord_symbol TYPE array ASSERT array::len($value) >= 2 AND $value == array::sort($value);
DEFINE FIELD IF NOT EXISTS pcs.* ON TABLE chord_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD IF NOT EXISTS gen ON TABLE chord_symbol TYPE int;
-- Record ids are deterministic (chord_symbol:⟨symbol⟩, voicing:[pitches], voicing_symbol:[[pitches], symbol]), so
-- symbol, mask and (in, out) are unique by construction; the indexes below are for lookups only. Bulk
-- INSERT ... ON DUPLICATE KEY UPDATE fails on re-runs when a UNIQUE index exists on another field.
DEFINE INDEX IF NOT EXISTS idx_symbol ON TABLE chord_symbol FIELDS symbol;

-- Voicings: sorted semitone offsets above the bass note (0 = C), forming a trie via `parent`.
DEFINE TABLE IF NOT EXISTS voicing SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS names ON TABLE voicing TYPE array ASSERT array::len($value) >= 1;
DEFINE FIELD IF NOT EXISTS names.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS pitches ON TABLE voicing TYPE array
    ASSERT $value[0] == 0 AND $value == array::sort($value) AND array::len($value) == array::len(array::distinct($value));
DEFINE FIELD IF NOT EXISTS pitches.* ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS top ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS size ON TABLE voicing TYPE int ASSERT $value >= 1 AND $value <= 25;
DEFINE FIELD IF NOT EXISTS mask ON TABLE voicing TYPE int ASSERT $value > 0;
DEFINE FIELD IF NOT EXISTS parent ON TABLE voicing TYPE option<record<voicing>>;
DEFINE FIELD IF NOT EXISTS emotions ON TABLE voicing TYPE array;
DEFINE FIELD IF NOT EXISTS emotions.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS gen ON TABLE voicing TYPE int;
DEFINE INDEX IF NOT EXISTS idx_mask ON TABLE voicing FIELDS mask;
DEFINE INDEX IF NOT EXISTS idx_parent ON TABLE voicing FIELDS parent;
DEFINE INDEX IF NOT EXISTS idx_size_top ON TABLE voicing FIELDS size, top;

-- One reading of a voicing as a chord symbol. `root` is the semitone offset of the chord root above the bass.
DEFINE TABLE IF NOT EXISTS voicing_symbol SCHEMAFULL TYPE RELATION IN voicing OUT chord_symbol;
DEFINE FIELD IF NOT EXISTS fit ON TABLE voicing_symbol TYPE float ASSERT $value >= 0.0 AND $value <= 1.0;
DEFINE FIELD IF NOT EXISTS root ON TABLE voicing_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD IF NOT EXISTS gen ON TABLE voicing_symbol TYPE int;
DEFINE INDEX IF NOT EXISTS idx_in_out ON TABLE voicing_symbol FIELDS in, out;
DEFINE INDEX IF NOT EXISTS idx_symbol_fit ON TABLE voicing_symbol FIELDS out, fit;

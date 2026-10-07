-- Fields are defined with OVERWRITE so that changed assertions apply to an existing store on reseed;
-- tables and indexes keep IF NOT EXISTS (overwriting an index would rebuild it on every run).
-- Chord symbols: absolute spellings with C as the bass note (C, Cm7, Ab/C, Fm7/C, ...).
DEFINE TABLE IF NOT EXISTS chord_symbol SCHEMAFULL;
DEFINE FIELD OVERWRITE symbol ON TABLE chord_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD OVERWRITE name ON TABLE chord_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD OVERWRITE root ON TABLE chord_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD OVERWRITE pcs ON TABLE chord_symbol TYPE array ASSERT array::len($value) >= 2 AND $value == array::sort($value);
DEFINE FIELD OVERWRITE pcs.* ON TABLE chord_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD OVERWRITE gen ON TABLE chord_symbol TYPE int;
-- Record ids are deterministic (chord_symbol:⟨symbol⟩, voicing:[pitches], voicing_symbol:[[pitches], symbol]), so
-- symbol, mask and (in, out) are unique by construction; the indexes below are for lookups only. Bulk
-- INSERT ... ON DUPLICATE KEY UPDATE fails on re-runs when a UNIQUE index exists on another field.
DEFINE INDEX IF NOT EXISTS idx_symbol ON TABLE chord_symbol FIELDS symbol;

-- Voicings: sorted semitone offsets above the bass note (0 = C), forming a trie via `parent`. Generated
-- voicings stay within two octaves; a few curated named voicings reach a third.
DEFINE TABLE IF NOT EXISTS voicing SCHEMAFULL;
DEFINE FIELD OVERWRITE names ON TABLE voicing TYPE array ASSERT array::len($value) >= 1;
DEFINE FIELD OVERWRITE names.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD OVERWRITE pitches ON TABLE voicing TYPE array
    ASSERT $value[0] == 0 AND $value == array::sort($value) AND array::len($value) == array::len(array::distinct($value));
DEFINE FIELD OVERWRITE pitches.* ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 36;
DEFINE FIELD OVERWRITE top ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 36;
DEFINE FIELD OVERWRITE size ON TABLE voicing TYPE int ASSERT $value >= 1 AND $value <= 37;
DEFINE FIELD OVERWRITE mask ON TABLE voicing TYPE int ASSERT $value > 0;
DEFINE FIELD OVERWRITE parent ON TABLE voicing TYPE option<record<voicing>>;
DEFINE FIELD OVERWRITE emotions ON TABLE voicing TYPE array;
DEFINE FIELD OVERWRITE emotions.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD OVERWRITE gen ON TABLE voicing TYPE int;
DEFINE INDEX IF NOT EXISTS idx_mask ON TABLE voicing FIELDS mask;
DEFINE INDEX IF NOT EXISTS idx_parent ON TABLE voicing FIELDS parent;
DEFINE INDEX IF NOT EXISTS idx_size_top ON TABLE voicing FIELDS size, top;

-- One reading of a voicing as a chord symbol. `root` is the semitone offset of the chord root above the bass.
DEFINE TABLE IF NOT EXISTS voicing_symbol SCHEMAFULL TYPE RELATION IN voicing OUT chord_symbol;
DEFINE FIELD OVERWRITE fit ON TABLE voicing_symbol TYPE float ASSERT $value >= 0.0 AND $value <= 1.0;
DEFINE FIELD OVERWRITE root ON TABLE voicing_symbol TYPE int ASSERT $value >= 0 AND $value <= 11;
DEFINE FIELD OVERWRITE rank ON TABLE voicing_symbol TYPE int ASSERT $value >= 0;
DEFINE FIELD OVERWRITE label ON TABLE voicing_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD OVERWRITE gen ON TABLE voicing_symbol TYPE int;
DEFINE INDEX IF NOT EXISTS idx_in_out ON TABLE voicing_symbol FIELDS in, out;
DEFINE INDEX IF NOT EXISTS idx_symbol_fit ON TABLE voicing_symbol FIELDS out, fit;

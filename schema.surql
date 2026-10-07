REMOVE TABLE IF EXISTS quality;

DEFINE TABLE IF NOT EXISTS chord_symbol SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS symbol ON TABLE chord_symbol TYPE string ASSERT string::len($value) > 0;
DEFINE INDEX IF NOT EXISTS uniq_symbol ON TABLE chord_symbol FIELDS symbol UNIQUE;

DEFINE TABLE IF NOT EXISTS voicing SCHEMAFULL;
REMOVE INDEX IF EXISTS idx_quality ON TABLE voicing;
REMOVE FIELD IF EXISTS quality ON TABLE voicing;
REMOVE FIELD IF EXISTS strength ON TABLE voicing;
REMOVE FIELD IF EXISTS name ON TABLE voicing;
DEFINE FIELD IF NOT EXISTS names ON TABLE voicing TYPE array ASSERT array::len($value) >= 1;
DEFINE FIELD IF NOT EXISTS names.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS pitches ON TABLE voicing TYPE array
    ASSERT $value == array::sort($value) AND array::len($value) == array::len(array::distinct($value));
DEFINE FIELD IF NOT EXISTS pitches.* ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS root ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS top ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS size ON TABLE voicing TYPE int ASSERT $value >= 1 AND $value <= 25;
DEFINE FIELD IF NOT EXISTS mask ON TABLE voicing TYPE int ASSERT $value > 0;
DEFINE FIELD IF NOT EXISTS parent ON TABLE voicing TYPE option<record<voicing>>;
DEFINE FIELD IF NOT EXISTS emotions ON TABLE voicing TYPE array;
DEFINE FIELD IF NOT EXISTS emotions.* ON TABLE voicing TYPE string ASSERT string::len($value) > 0;

DEFINE INDEX IF NOT EXISTS uniq_mask ON TABLE voicing FIELDS mask UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_parent ON TABLE voicing FIELDS parent;
DEFINE INDEX IF NOT EXISTS idx_size ON TABLE voicing FIELDS size;
DEFINE INDEX IF NOT EXISTS idx_size_top ON TABLE voicing FIELDS size, top;

DEFINE TABLE IF NOT EXISTS voicing_symbol SCHEMAFULL TYPE RELATION IN voicing OUT chord_symbol;
DEFINE FIELD IF NOT EXISTS fit ON TABLE voicing_symbol TYPE float ASSERT $value >= 0.0 AND $value <= 1.0;
DEFINE INDEX IF NOT EXISTS uniq_voicing_symbol ON TABLE voicing_symbol FIELDS in, out UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_symbol_fit ON TABLE voicing_symbol FIELDS out, fit;

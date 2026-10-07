DEFINE TABLE IF NOT EXISTS quality SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON TABLE quality TYPE string ASSERT string::len($value) > 0;
DEFINE INDEX IF NOT EXISTS uniq_quality_name ON TABLE quality FIELDS name UNIQUE;

DEFINE TABLE IF NOT EXISTS voicing SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS pitches ON TABLE voicing TYPE array
    ASSERT $value == array::sort($value) AND array::len($value) == array::len(array::distinct($value));
DEFINE FIELD IF NOT EXISTS pitches.* ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS root ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS top ON TABLE voicing TYPE int ASSERT $value >= 0 AND $value <= 24;
DEFINE FIELD IF NOT EXISTS size ON TABLE voicing TYPE int ASSERT $value >= 1 AND $value <= 25;
DEFINE FIELD IF NOT EXISTS mask ON TABLE voicing TYPE int ASSERT $value > 0;
DEFINE FIELD IF NOT EXISTS parent ON TABLE voicing TYPE option<record<voicing>>;
DEFINE FIELD IF NOT EXISTS quality ON TABLE voicing TYPE option<record<quality>>;
DEFINE FIELD IF NOT EXISTS strength ON TABLE voicing TYPE option<float> ASSERT $value >= 0.0 AND $value <= 1.0;

DEFINE INDEX IF NOT EXISTS uniq_mask ON TABLE voicing FIELDS mask UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_parent ON TABLE voicing FIELDS parent;
DEFINE INDEX IF NOT EXISTS idx_size ON TABLE voicing FIELDS size;
DEFINE INDEX IF NOT EXISTS idx_size_top ON TABLE voicing FIELDS size, top;
DEFINE INDEX IF NOT EXISTS idx_quality ON TABLE voicing FIELDS quality, strength;

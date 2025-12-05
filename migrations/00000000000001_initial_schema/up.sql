-- Initial schema for Losselot
-- Contains: schema_versions, analysis_results

CREATE TABLE schema_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    version TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    features TEXT NOT NULL,
    introduced_at TEXT NOT NULL
);

CREATE TABLE analysis_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    analyzed_at TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    verdict TEXT NOT NULL,
    combined_score INTEGER NOT NULL,
    spectral_score INTEGER NOT NULL,
    binary_score INTEGER NOT NULL,
    bitrate INTEGER NOT NULL,
    sample_rate INTEGER NOT NULL,
    duration_secs REAL,
    encoder TEXT,
    lowpass INTEGER,
    rms_full REAL,
    rms_mid_high REAL,
    rms_high REAL,
    rms_upper REAL,
    rms_19_20k REAL,
    rms_ultrasonic REAL,
    high_drop REAL,
    upper_drop REAL,
    ultrasonic_drop REAL,
    ultrasonic_flatness REAL,
    cutoff_variance REAL,
    avg_cutoff_freq REAL,
    rolloff_slope REAL,
    transition_width REAL,
    natural_rolloff INTEGER,
    binary_details_json TEXT,
    flags TEXT,
    error TEXT,
    file_hash TEXT,
    UNIQUE(file_path, analyzed_at)
);

CREATE INDEX idx_file_path ON analysis_results(file_path);
CREATE INDEX idx_verdict ON analysis_results(verdict);
CREATE INDEX idx_analyzed_at ON analysis_results(analyzed_at);
CREATE INDEX idx_schema_version ON analysis_results(schema_version);

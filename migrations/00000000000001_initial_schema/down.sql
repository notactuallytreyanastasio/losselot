-- Revert initial schema
DROP INDEX IF EXISTS idx_schema_version;
DROP INDEX IF EXISTS idx_analyzed_at;
DROP INDEX IF EXISTS idx_verdict;
DROP INDEX IF EXISTS idx_file_path;
DROP TABLE IF EXISTS analysis_results;
DROP TABLE IF EXISTS schema_versions;

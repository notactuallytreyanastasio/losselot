-- Revert command log
DROP INDEX IF EXISTS idx_command_decision_node;
DROP INDEX IF EXISTS idx_command_started_at;
DROP TABLE IF EXISTS command_log;

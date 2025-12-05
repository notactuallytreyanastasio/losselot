-- Command log for tracking shell/bash commands executed
-- Provides full audit trail of actions taken

CREATE TABLE command_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    command TEXT NOT NULL,
    description TEXT,
    working_dir TEXT,
    exit_code INTEGER,
    stdout TEXT,
    stderr TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_ms INTEGER,
    decision_node_id INTEGER,
    FOREIGN KEY (decision_node_id) REFERENCES decision_nodes(id)
);

CREATE INDEX idx_command_started_at ON command_log(started_at);
CREATE INDEX idx_command_decision_node ON command_log(decision_node_id);

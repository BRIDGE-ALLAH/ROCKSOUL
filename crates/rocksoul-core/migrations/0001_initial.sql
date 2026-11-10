PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 80),
    root_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_projects_updated_at
    ON projects(updated_at DESC);

CREATE TABLE IF NOT EXISTS missions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 120),
    objective TEXT NOT NULL CHECK (length(objective) BETWEEN 1 AND 2000),
    status TEXT NOT NULL CHECK (status IN ('draft', 'in_progress', 'review', 'completed', 'blocked', 'cancelled')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_missions_project_updated
    ON missions(project_id, updated_at DESC);

CREATE TABLE IF NOT EXISTS artifacts (
    id TEXT PRIMARY KEY NOT NULL,
    mission_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY(mission_id) REFERENCES missions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_artifacts_mission_created
    ON artifacts(mission_id, created_at DESC);

CREATE TABLE IF NOT EXISTS evidence (
    id TEXT PRIMARY KEY NOT NULL,
    mission_id TEXT NOT NULL,
    artifact_id TEXT,
    kind TEXT NOT NULL,
    summary TEXT NOT NULL,
    source_path TEXT,
    details_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY(mission_id) REFERENCES missions(id) ON DELETE CASCADE,
    FOREIGN KEY(artifact_id) REFERENCES artifacts(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_evidence_mission_created
    ON evidence(mission_id, created_at DESC);

-- Typed domain-event ledger. Events are metadata-only and immutable after append.
CREATE TABLE IF NOT EXISTS event_ledger (
    id TEXT PRIMARY KEY NOT NULL,
    event_type TEXT NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    subject_type TEXT NOT NULL CHECK (subject_type IN ('project', 'mission', 'backup')),
    subject_id TEXT NOT NULL,
    actor_type TEXT CHECK (actor_type IS NULL OR actor_type IN ('user', 'system')),
    actor_id TEXT,
    occurred_at TEXT NOT NULL,
    correlation_id TEXT,
    payload_json TEXT NOT NULL,
    payload_bytes INTEGER NOT NULL CHECK (payload_bytes BETWEEN 2 AND 16384),
    CHECK ((actor_type IS NULL AND actor_id IS NULL) OR (actor_type IS NOT NULL AND actor_id IS NOT NULL))
);

CREATE INDEX IF NOT EXISTS idx_event_ledger_subject_time
    ON event_ledger (subject_type, subject_id, occurred_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS idx_event_ledger_correlation
    ON event_ledger (correlation_id, occurred_at DESC)
    WHERE correlation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_event_ledger_type_time
    ON event_ledger (event_type, occurred_at DESC);

CREATE TRIGGER IF NOT EXISTS event_ledger_reject_update
BEFORE UPDATE ON event_ledger
BEGIN
    SELECT RAISE(ABORT, 'event_ledger is append-only');
END;

CREATE TRIGGER IF NOT EXISTS event_ledger_reject_delete
BEFORE DELETE ON event_ledger
BEGIN
    SELECT RAISE(ABORT, 'event_ledger is append-only');
END;

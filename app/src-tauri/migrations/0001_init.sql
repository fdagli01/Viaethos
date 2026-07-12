-- Via Ethos core schema (v1 Life OS). See ARCHITECTURE.md for rationale.

CREATE TABLE IF NOT EXISTS pillars (
  id           TEXT PRIMARY KEY,
  name         TEXT NOT NULL,
  color_token  TEXT NOT NULL,
  icon         TEXT,
  sort_order   INTEGER NOT NULL DEFAULT 0,
  archived_at  INTEGER
);

CREATE TABLE IF NOT EXISTS actions (
  id              TEXT PRIMARY KEY,
  pillar_id       TEXT NOT NULL REFERENCES pillars(id),
  name            TEXT NOT NULL,
  kind            TEXT NOT NULL CHECK (kind IN ('focus', 'tick')),
  default_minutes INTEGER,
  schedule        TEXT NOT NULL,
  target_per_day  INTEGER NOT NULL DEFAULT 1,
  sort_order      INTEGER NOT NULL DEFAULT 0,
  created_at      INTEGER NOT NULL,
  archived_at     INTEGER
);

CREATE TABLE IF NOT EXISTS entries (
  id              TEXT PRIMARY KEY,
  action_id       TEXT NOT NULL REFERENCES actions(id),
  kind            TEXT NOT NULL CHECK (kind IN ('focus', 'tick')),
  occurred_on     TEXT NOT NULL,
  intention       TEXT,
  planned_minutes INTEGER,
  started_at      INTEGER,
  ended_at        INTEGER,
  outcome         TEXT CHECK (outcome IN ('completed', 'abandoned', 'interrupted')),
  reflection      TEXT,
  created_at      INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_entries_action_day ON entries(action_id, occurred_on);
CREATE INDEX IF NOT EXISTS idx_entries_active ON entries(kind, ended_at);

CREATE TABLE IF NOT EXISTS tasks (
  id           TEXT PRIMARY KEY,
  pillar_id    TEXT REFERENCES pillars(id),
  action_id    TEXT REFERENCES actions(id),
  title        TEXT NOT NULL,
  due_on       TEXT,
  completed_at INTEGER,
  created_at   INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS ethos_ledger (
  id         TEXT PRIMARY KEY,
  pillar_id  TEXT NOT NULL REFERENCES pillars(id),
  action_id  TEXT REFERENCES actions(id),      -- NULL for non-action sources (tasks, meals, sleep...)
  entry_id   TEXT REFERENCES entries(id),
  points     INTEGER NOT NULL,
  reason     TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ledger_pillar ON ethos_ledger(pillar_id, created_at);

CREATE TABLE IF NOT EXISTS settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

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
  lesson_id       TEXT REFERENCES lessons(id),  -- set when this Focus Session studies a lesson
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

-- Real weather (Open-Meteo). One cached snapshot; offline falls back to the
-- last row when a fresh fetch fails. Independent of Inner Weather, which is
-- a manual mood layer — real weather is what the sky actually shows outside.
CREATE TABLE IF NOT EXISTS weather_cache (
  fetched_at   INTEGER PRIMARY KEY,
  payload_json TEXT NOT NULL
);

-- Sofra: meal/calorie tracking. food_items is a small packaged list plus
-- anything the user defines; meals are the actual log entries.
CREATE TABLE IF NOT EXISTS food_items (
  id               TEXT PRIMARY KEY,
  name             TEXT NOT NULL,
  kcal_per_100g    REAL NOT NULL,
  protein_per_100g REAL NOT NULL DEFAULT 0,
  carb_per_100g    REAL NOT NULL DEFAULT 0,
  fat_per_100g     REAL NOT NULL DEFAULT 0,
  user_defined     INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS meals (
  id          TEXT PRIMARY KEY,
  occurred_on TEXT NOT NULL,
  time_slot   TEXT NOT NULL CHECK (time_slot IN ('breakfast', 'lunch', 'dinner', 'snack')),
  name        TEXT NOT NULL,
  kcal        REAL NOT NULL,
  protein_g   REAL NOT NULL DEFAULT 0,
  carb_g      REAL NOT NULL DEFAULT 0,
  fat_g       REAL NOT NULL DEFAULT 0,
  note        TEXT,
  created_at  INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_meals_day ON meals(occurred_on);

-- Müfredat: a course is a plowed field row; each lesson studied is a
-- planted strip. A course carries its own linked Focus action so studying
-- flows through the exact same Ritual/Mosaic/streak machinery as any other
-- Focus Session — no parallel UI paradigm needed.
CREATE TABLE IF NOT EXISTS courses (
  id                TEXT PRIMARY KEY,
  name              TEXT NOT NULL,
  pillar_id         TEXT NOT NULL REFERENCES pillars(id),
  action_id         TEXT NOT NULL REFERENCES actions(id),
  color_token       TEXT NOT NULL,
  target_hours_week REAL NOT NULL DEFAULT 3,
  created_at        INTEGER NOT NULL,
  archived_at       INTEGER
);

CREATE TABLE IF NOT EXISTS lessons (
  id         TEXT PRIMARY KEY,
  course_id  TEXT NOT NULL REFERENCES courses(id),
  title      TEXT NOT NULL,
  planned_on TEXT NOT NULL,
  status     TEXT NOT NULL DEFAULT 'planned' CHECK (status IN ('planned', 'done', 'skipped')),
  review_of  TEXT REFERENCES lessons(id),  -- set on auto-generated spaced-repetition reviews
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lessons_course ON lessons(course_id, planned_on);

-- Uyku: one log per night. `date` is the wake-up morning's date, so "last
-- night" is always a simple lookup by today's date.
CREATE TABLE IF NOT EXISTS sleep_logs (
  id          TEXT PRIMARY KEY,
  date        TEXT NOT NULL UNIQUE,
  bed_at      TEXT NOT NULL,   -- 'HH:MM', the evening before `date`
  woke_at     TEXT NOT NULL,   -- 'HH:MM', on `date`
  quality_1_5 INTEGER NOT NULL CHECK (quality_1_5 BETWEEN 1 AND 5),
  created_at  INTEGER NOT NULL
);

-- Marks rows created by the "generate a synthetic month" preview tool, so
-- they can be told apart from real user data and wiped independently of a
-- full factory reset. 0 (default) means real data.
ALTER TABLE entries ADD COLUMN IF NOT EXISTS synthetic INTEGER NOT NULL DEFAULT 0;
ALTER TABLE ethos_ledger ADD COLUMN IF NOT EXISTS synthetic INTEGER NOT NULL DEFAULT 0;
ALTER TABLE meals ADD COLUMN IF NOT EXISTS synthetic INTEGER NOT NULL DEFAULT 0;
ALTER TABLE sleep_logs ADD COLUMN IF NOT EXISTS synthetic INTEGER NOT NULL DEFAULT 0;

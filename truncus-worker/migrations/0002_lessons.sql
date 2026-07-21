CREATE TABLE IF NOT EXISTS lessons (
  id TEXT PRIMARY KEY,
  project TEXT NOT NULL,
  category TEXT NOT NULL DEFAULT 'insight',
  title TEXT NOT NULL,
  insight TEXT NOT NULL,
  evidence TEXT NOT NULL DEFAULT '',
  confidence REAL NOT NULL DEFAULT 0.5,
  times_seen INTEGER NOT NULL DEFAULT 1,
  created_at INTEGER NOT NULL DEFAULT 0,
  updated_at INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_lessons_project ON lessons(project, confidence DESC, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_lessons_updated ON lessons(updated_at DESC);

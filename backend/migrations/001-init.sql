-- Scripts to be executed by notes
CREATE TABLE codes (
  name TEXT PRIMARY KEY,      -- e.g. "weekly_check", "project_bootstrap"
  script TEXT NOT NULL        -- Lua code as text
);

-- Hierarchical notes referencing codes
CREATE TABLE notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_id INTEGER,          -- for hierarchy; nullable root
  title TEXT NOT NULL,
  description TEXT,
  code_name TEXT,             -- optional logic attached

  FOREIGN KEY (parent_id) REFERENCES notes(id) ON DELETE CASCADE,
  FOREIGN KEY (code_name) REFERENCES codes(name) ON DELETE SET NULL
);

-- Key-value store of metadata for each note
CREATE TABLE attributes (
  note_id INTEGER NOT NULL,
  key TEXT NOT NULL,
  value TEXT NOT NULL,

  PRIMARY KEY (note_id, key),
  FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
);

-- Logs associated with notes
CREATE TABLE logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  note_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  kind TEXT NOT NULL,         -- e.g., "info", "error", "user", "script"
  message TEXT NOT NULL,      -- log body
  blob_data BLOB,             -- optional binary payload

  FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
);

INSERT OR REPLACE INTO meta (key, value) VALUES ('schema_version', '1');

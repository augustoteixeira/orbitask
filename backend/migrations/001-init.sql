-- Boards: each is an independent kanban workspace
CREATE TABLE boards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    is_template BOOLEAN NOT NULL DEFAULT 0,
    UNIQUE (name)
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE board_tags (
  board_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,

  PRIMARY KEY (board_id, tag_id),

  FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Factories: define how tasks are generated on a recurring basis
CREATE TABLE yearly_factories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    -- say we are displaying the note with reference year 2023,
    -- this will have a start date of 2023 + year_displacement
    -- + month + day.
    year_displacement INTEGER NOT NULL CHECK (day BETWEEN -4 AND 4),
    month INTEGER NOT NULL CHECK (month BETWEEN 1 AND 12),
    day INTEGER NOT NULL CHECK (day BETWEEN 1 AND 31),
    duration INTEGER NOT NULL,
    -- the interval of reference years that will be produced
    start_year INTEGER NOT NULL CHECK (start_year BETWEEN 1900 AND 3000),
    end_year INTEGER NOT NULL CHECK (end_year BETWEEN 1900 AND 3000),
    -- end_year is inclusive

    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
);

CREATE TABLE monthly_factories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    month_displacement INTEGER NOT NULL CHECK (day BETWEEN -11 AND 11),
    day INTEGER NOT NULL CHECK (month_displacement BETWEEN 1 AND 31),
    duration INTEGER NOT NULL,
    start_year INTEGER NOT NULL CHECK (start_year BETWEEN 1900 AND 3000),
    start_month INTEGER NOT NULL CHECK (start_month BETWEEN 1 AND 12),
    end_year INTEGER NOT NULL CHECK (end_year BETWEEN 1900 AND 3000),
    end_month INTEGER NOT NULL CHECK (end_month BETWEEN 1 AND 12),
    -- end_month is inclusive

    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
);

CREATE TABLE weekly_factories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    week_displacement INTEGER NOT NULL CHECK (day BETWEEN -4 AND 4),
    day INTEGER NOT NULL CHECK (week_displacement BETWEEN 1 AND 7),
    duration INTEGER NOT NULL,

    start_year INTEGER NOT NULL CHECK (start_year BETWEEN 1900 AND 3000),
    start_week INTEGER NOT NULL CHECK (start_week BETWEEN 1 AND 53),
    end_year INTEGER NOT NULL CHECK (end_year BETWEEN 1900 AND 3000),
    end_week INTEGER NOT NULL CHECK (end_week BETWEEN 1 AND 53),
    -- end_week is inclusive

    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
);


-- States: each board has its own custom workflow states
CREATE TABLE states (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    is_finished BOOLEAN NOT NULL DEFAULT 0,
    position INTEGER NOT NULL, -- For ordering states in UI

    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE,
    UNIQUE (board_id, name)
);

-- Notes: created manually or from a factory
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_id INTEGER NOT NULL,
    --factory_id INTEGER, -- nullable if manually created

    state_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL, -- ISO, ordinal day
    due_date TEXT NOT NULL,

    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE,
    --FOREIGN KEY (factory_id) REFERENCES factories(id) ON DELETE SET NULL,
    FOREIGN KEY (state_id) REFERENCES states(id) ON DELETE RESTRICT
);

-- Annotations: comments or events attached to notes
CREATE TABLE annotations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    note_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('comment', 'system')),

    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
);

INSERT OR REPLACE INTO meta (key, value) VALUES ('schema_version', '1');

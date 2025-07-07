CREATE TABLE IF NOT EXISTS meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO meta (key, value) VALUES
  ('password_hash', '$2b$12$d7qM7ZrPHqmxxOln2aztFu4GQFV/oeFYoJrw63EAPYTcmwwQHEY7e');

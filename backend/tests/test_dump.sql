-- -- Clear old data
-- DELETE FROM notes;
-- DELETE FROM codes;
-- DELETE FROM attributes;
-- DELETE FROM logs;
-- DELETE FROM sqlite_sequence WHERE name IN ('notes', 'codes', 'attributes', 'logs');

-- Sample codes
INSERT INTO codes (name, capabilities, script) VALUES
  ('simple_done', '["SysLog", { "GetAttribute": "Own" } , { "SetAttribute": "Own" } ]', '-- placeholder'),
  ('archive_old', '[]', '-- Lua: for each note older than X, archive it');

-- Root notes
INSERT INTO notes (title, description, code_name) VALUES
  ('Main Project', 'Top-level project note', 'simple_done'),
  ('Inbox', 'Temporary tasks and notes', NULL);

-- Sub-notes
INSERT INTO notes (parent_id, title, description, code_name) VALUES
  (1, 'Design Phase', 'UI and UX work', NULL),
  (1, 'Implementation Phase', 'Coding and testing', 'archive_old'),
  (2, 'Buy groceries', 'Milk, eggs, etc.', NULL);

-- Attributes
INSERT INTO attributes (note_id, key, value) VALUES
  (1, 'owner', 'Alice'),
  (3, 'status', 'pending'),
  (4, 'deadline', '2025-07-01'),
  (5, 'tag', 'personal');

-- Logs (some with dummy binary blobs)
INSERT INTO logs (note_id, kind, message, blob_data) VALUES
  (1, 'info', 'Project created.', NULL),
  (3, 'user', 'Design started by Bob.', NULL),
  (4, 'script', 'Archived due to inactivity.', X'FFD8FFE0'), -- Fake JPEG header
  (5, 'user', 'Marked as important.', NULL);

-- Sample codes
INSERT INTO codes (name, capabilities, script) VALUES
  ('simple_done', '["SysLog", { "GetAttribute": "Own" } , { "SetAttribute": "Own" } ]', '-- placeholder');

-- Root notes
INSERT INTO notes (title, description, code_name) VALUES
  ('First Note', 'First note description', 'simple_done');

-- Sub-notes
INSERT INTO notes (parent_id, title, description, code_name) VALUES
  (1, 'Sub note of one', 'This is a subnote', NULL);

-- Attributes
INSERT INTO attributes (note_id, key, value) VALUES
  (1, 'tag1', 'Value of tag 1');

-- Logs (some with dummy binary blobs)
INSERT INTO logs (note_id, kind, message, blob_data) VALUES
  (1, 'info', 'Project created.', NULL);

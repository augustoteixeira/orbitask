-- Clear old data
DELETE FROM board_tags;
DELETE FROM tags;
DELETE FROM notes;
DELETE FROM states;
DELETE FROM boards;
DELETE FROM sqlite_sequence WHERE name IN ('boards', 'states', 'notes', 'tags');


-- Insert fake boards into the `boards` table
INSERT INTO boards (name, is_template) VALUES
  ('Marketing Plan', 0),
  ('Engineering Roadmap', 0),
  ('Weekly Tasks', 0),
  ('Product Launch Template', 1),
  ('Research Pipeline', 0),
  ('Conference Planning', 0),
  ('Blog Editorial Calendar', 0),
  ('Onboarding Template', 1),
  ('DevOps Checklist', 0),
  ('Personal Goals', 0);

INSERT INTO tags (name) VALUES
  ('urgent'),
  ('template'),
  ('weekly'),
  ('long-term'),
  ('design'),
  ('meeting'),
  ('writing'),
  ('launch'),
  ('internal'),
  ('external');

-- Marketing Plan
INSERT INTO board_tags (board_id, tag_id) VALUES
  (1, 1),  -- urgent
  (1, 4),  -- long-term
  (1, 5);  -- design

-- Engineering Roadmap
INSERT INTO board_tags (board_id, tag_id) VALUES
  (2, 4),  -- long-term
  (2, 9);  -- internal

-- Weekly Tasks
INSERT INTO board_tags (board_id, tag_id) VALUES
  (3, 3);  -- weekly

-- Product Launch Template
INSERT INTO board_tags (board_id, tag_id) VALUES
  (4, 2),  -- template
  (4, 8);  -- launch

-- Research Pipeline
INSERT INTO board_tags (board_id, tag_id) VALUES
  (5, 4),  -- long-term
  (5, 10); -- external

-- Conference Planning
INSERT INTO board_tags (board_id, tag_id) VALUES
  (6, 6),  -- meeting
  (6, 10); -- external

-- Blog Editorial Calendar
INSERT INTO board_tags (board_id, tag_id) VALUES
  (7, 3),  -- weekly
  (7, 7);  -- writing

-- Onboarding Template
INSERT INTO board_tags (board_id, tag_id) VALUES
  (8, 2),  -- template
  (8, 9);  -- internal

-- DevOps Checklist
INSERT INTO board_tags (board_id, tag_id) VALUES
  (9, 1),  -- urgent
  (9, 9);  -- internal

-- Personal Goals
INSERT INTO board_tags (board_id, tag_id) VALUES
  (10, 4); -- long-term

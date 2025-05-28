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

-- States for 'Marketing Plan' (board_id = 1)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (1, 'Ideas', 0, 0),
  (1, 'Drafting', 0, 1),
  (1, 'Review', 0, 2),
  (1, 'Approved', 1, 3);

-- States for 'Engineering Roadmap' (board_id = 2)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (2, 'Planned', 0, 0),
  (2, 'In Progress', 0, 1),
  (2, 'Testing', 0, 2),
  (2, 'Released', 1, 3);

-- States for 'Weekly Tasks' (board_id = 3)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (3, 'Backlog', 0, 0),
  (3, 'Doing', 0, 1),
  (3, 'Done', 1, 2);

-- States for 'Product Launch Template' (board_id = 4)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (4, 'Pre-launch', 0, 0),
  (4, 'Launch Prep', 0, 1),
  (4, 'Launched', 1, 2),
  (4, 'Post-launch Review', 1, 3);

-- States for 'Research Pipeline' (board_id = 5)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (5, 'Idea', 0, 0),
  (5, 'Feasibility Study', 0, 1),
  (5, 'Experimentation', 0, 2),
  (5, 'Published', 1, 3);

-- States for 'Conference Planning' (board_id = 6)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (6, 'To Contact', 0, 0),
  (6, 'Waiting Response', 0, 1),
  (6, 'Confirmed', 1, 2),
  (6, 'Attended', 1, 3);

-- States for 'Blog Editorial Calendar' (board_id = 7)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (7, 'Ideas', 0, 0),
  (7, 'Writing', 0, 1),
  (7, 'Editing', 0, 2),
  (7, 'Published', 1, 3);

-- States for 'Onboarding Template' (board_id = 8)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (8, 'Preparation', 0, 0),
  (8, 'First Week', 0, 1),
  (8, 'First Month', 0, 2),
  (8, 'Fully Onboarded', 1, 3);

-- States for 'DevOps Checklist' (board_id = 9)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (9, 'To Configure', 0, 0),
  (9, 'Deploying', 0, 1),
  (9, 'Monitoring', 0, 2),
  (9, 'Stable', 1, 3);

-- States for 'Personal Goals' (board_id = 10)
INSERT INTO states (board_id, name, is_finished, position) VALUES
  (10, 'Planned', 0, 0),
  (10, 'In Progress', 0, 1),
  (10, 'Achieved', 1, 2);

-- 'Marketing Plan' states: 1 (Ideas), 2 (Drafting), 3 (Review), 4 (Approved)
INSERT INTO notes (board_id, state_id, name, start_date, due_date) VALUES
  -- State 1 (Ideas): 0 notes
  -- State 2 (Drafting): 2 notes
  (1, 2, 'Write homepage copy', '2025-05-01', '2025-05-02'),
  (1, 2, 'Design CTA layout', '2025-05-02', '2025-05-03'),

  -- State 3 (Review): 1 note
  (1, 3, 'Submit to team for review', '2025-05-04', '2025-05-05'),

  -- State 4 (Approved): 3 notes
  (1, 4, 'Approved: homepage', '2025-05-06', '2025-05-07'),
  (1, 4, 'Approved: press kit', '2025-05-08', '2025-05-09'),
  (1, 4, 'Approved: social plan', '2025-05-09', '2025-05-10');

-- States: 5–8
INSERT INTO notes (board_id, state_id, name, start_date, due_date) VALUES
  -- State 5 (Planned): 1 note
  (2, 5, 'Plan Q3 sprint', '2025-05-01', '2025-05-01'),

  -- State 6 (In Progress): 3 notes
  (2, 6, 'Feature toggle refactor', '2025-05-02', '2025-05-04'),
  (2, 6, 'Implement dark mode', '2025-05-03', '2025-05-06'),
  (2, 6, 'Database migration draft', '2025-05-04', '2025-05-07'),

  -- State 7 (Testing): 0 notes

  -- State 8 (Released): 1 note
  (2, 8, 'Release v2.3', '2025-05-08', '2025-05-09');

-- States: 9–11
INSERT INTO notes (board_id, state_id, name, start_date, due_date) VALUES
  -- State 9 (Backlog): 2 notes
  (3, 9, 'Update dependencies', '2025-05-01', '2025-05-01'),
  (3, 9, 'Check backups', '2025-05-01', '2025-05-02'),

  -- State 10 (Doing): 1 note
  (3, 10, 'Weekly team sync', '2025-05-02', '2025-05-02');

  -- State 11 (Done): 0 notes

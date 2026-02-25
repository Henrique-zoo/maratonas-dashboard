-- =========================
-- ENUMS
-- =========================

CREATE TYPE gender_category AS ENUM ('Open', 'FemaleOnly');
CREATE TYPE gender AS ENUM ('Male', 'Female', 'Other', 'RatherNotAnswer');
CREATE TYPE status AS ENUM (
  'Accepted',
  'WrongAnswer',
  'TimeLimitExceeded',
  'MemoryLimitExceeded',
  'PresentationError',
  'CompilationError',
  'RuntimeError'
);
CREATE TYPE role AS ENUM ('Contestant', 'Coach', 'Reserve');
CREATE TYPE location_type AS ENUM (
  'continent',
  'country',
  'region',
  'province',
  'prefecture',
  'city',
  'campus'
);

-- =========================
-- TABLES
-- =========================

CREATE TABLE organizer (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  website_url VARCHAR
);

CREATE TABLE location (
  id SERIAL PRIMARY KEY,
  parent_id INT REFERENCES location(id),
  type location_type NOT NULL,
  name TEXT NOT NULL,
  UNIQUE(parent_id, name)
);

CREATE TABLE competition (
  id SERIAL PRIMARY KEY,
  organizer_id INT NOT NULL REFERENCES organizer(id),
  name VARCHAR NOT NULL,
  gender_category gender_category NOT NULL,
  website_url VARCHAR,
  UNIQUE (name, organizer_id)
);

CREATE INDEX idx_competition_organizer_id ON competition(organizer_id);

CREATE TABLE event (
  id SERIAL PRIMARY KEY,
  competition_id INT NOT NULL REFERENCES competition(id),
  location_id INT NOT NULL REFERENCES location(id),
  name VARCHAR NOT NULL,
  date DATE NOT NULL,
  UNIQUE (competition_id, name, date)
);

CREATE INDEX idx_event_competition_id ON event(competition_id);
CREATE INDEX idx_event_location_id ON event(location_id);

CREATE TABLE institution (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  short_name TEXT,
  site TEXT
);

CREATE TABLE institution_location (
  institution_id INT REFERENCES institution(id),
  location_id INT REFERENCES location(id),
  PRIMARY KEY (institution_id, location_id)
);

CREATE TABLE team (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  institution_id INT NOT NULL REFERENCES institution(id),
  UNIQUE (name, institution_id)
);

CREATE INDEX idx_team_institution_id ON team(institution_id);

CREATE TABLE problem (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL REFERENCES event(id),
  item VARCHAR(10) NOT NULL,
  title TEXT NOT NULL,
  statement TEXT NOT NULL,
  UNIQUE (event_id, item)
);

CREATE INDEX idx_problem_event_id ON problem(event_id);

CREATE TABLE input_output (
  id SERIAL PRIMARY KEY,
  problem_id INT NOT NULL REFERENCES problem(id),
  input TEXT NOT NULL,
  output TEXT NOT NULL
);

CREATE INDEX idx_io_problem_id ON input_output(problem_id);

CREATE TABLE author (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  nationality TEXT
);

CREATE TABLE authorship (
  author_id INT REFERENCES author(id),
  problem_id INT REFERENCES problem(id),
  PRIMARY KEY (author_id, problem_id)
);

CREATE TABLE team_event (
  id SERIAL PRIMARY KEY,
  team_id INT NOT NULL REFERENCES team(id),
  event_id INT NOT NULL REFERENCES event(id),
  rank INT,
  UNIQUE (team_id, event_id)
);

CREATE INDEX idx_team_event_team_id ON team_event(team_id);
CREATE INDEX idx_team_event_event_id ON team_event(event_id);

CREATE TABLE member (
  id SERIAL PRIMARY KEY,
  gender gender NOT NULL
);

CREATE TABLE team_event_member (
  member_id INT REFERENCES member(id),
  team_event_id INT REFERENCES team_event(id),
  role role NOT NULL,
  PRIMARY KEY (member_id, team_event_id)
);

CREATE TABLE submission (
  id SERIAL PRIMARY KEY,
  status status NOT NULL,
  language VARCHAR NOT NULL,
  code TEXT NOT NULL,
  submission_time TIMESTAMP NOT NULL,
  team_event_id INT NOT NULL REFERENCES team_event(id),
  problem_id INT NOT NULL REFERENCES problem(id)
);

CREATE INDEX idx_submission_team_event_id ON submission(team_event_id);
CREATE INDEX idx_submission_problem_id ON submission(problem_id);
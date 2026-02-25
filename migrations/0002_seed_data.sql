-- =========================
-- ORGANIZER
-- =========================

INSERT INTO organizer (name, website_url)
VALUES ('ICPC', 'https://icpc.global');

-- =========================
-- LOCATION
-- =========================

INSERT INTO location (type, name) VALUES
('continent', 'South America'),
('country', 'Brazil'),
('city', 'São Paulo'),
('campus', 'USP');

UPDATE location SET parent_id = 1 WHERE id = 2;
UPDATE location SET parent_id = 2 WHERE id = 3;
UPDATE location SET parent_id = 3 WHERE id = 4;

-- =========================
-- COMPETITION
-- =========================

INSERT INTO competition (organizer_id, name, gender_category, website_url)
VALUES (1, 'ICPC Latin America', 'Open', 'https://latam.icpc.global');

-- =========================
-- EVENT
-- =========================

INSERT INTO event (competition_id, location_id, name, date)
VALUES (1, 4, 'ICPC Brazil Finals', '2024-10-10');

-- =========================
-- INSTITUTION
-- =========================

INSERT INTO institution (name, short_name, site)
VALUES ('Universidade de São Paulo', 'USP', 'https://usp.br');

INSERT INTO institution_location VALUES (1, 4);

-- =========================
-- TEAM
-- =========================

INSERT INTO team (name, institution_id)
VALUES ('USP Coders', 1);

-- =========================
-- TEAM_EVENT
-- =========================

INSERT INTO team_event (team_id, event_id, rank)
VALUES (1, 1, 1);

-- =========================
-- MEMBER
-- =========================

INSERT INTO member (gender) VALUES
('Male'), ('Female'), ('Male');

-- =========================
-- TEAM_EVENT_MEMBER
-- =========================

INSERT INTO team_event_member (member_id, team_event_id, role)
VALUES
(1, 1, 'Contestant'),
(2, 1, 'Contestant'),
(3, 1, 'Coach');

-- =========================
-- PROBLEM
-- =========================

INSERT INTO problem (event_id, item, title, statement)
VALUES
(1, 'A', 'Sum of Numbers', 'Given two integers, output their sum.'),
(1, 'B', 'Maximum Subarray', 'Find the maximum subarray sum.');

-- =========================
-- INPUT_OUTPUT
-- =========================

INSERT INTO input_output (problem_id, input, output)
VALUES
(1, '1 2', '3'),
(1, '10 20', '30'),
(2, '5\n1 2 3 4 5', '15');

-- =========================
-- AUTHOR
-- =========================

INSERT INTO author (name, nationality)
VALUES ('John Doe', 'USA');

-- =========================
-- AUTHORSHIP
-- =========================

INSERT INTO authorship (author_id, problem_id)
VALUES (1, 1), (1, 2);

-- =========================
-- SUBMISSION
-- =========================

INSERT INTO submission (status, language, code, submission_time, team_event_id, problem_id)
VALUES
('Accepted', 'C++', 'int main(){...}', now(), 1, 1),
('WrongAnswer', 'Python', 'print(1+1)', now(), 1, 2);
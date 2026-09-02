-- Dedicated, deterministic data set for the HTTP BDD suite.
-- It deliberately replaces the demonstration seed installed by migrations.

TRUNCATE TABLE
    submission,
    authorship,
    input_output,
    problem,
    team_event_member,
    team_event,
    team_contestant,
    member,
    team,
    institution_location,
    institution,
    event_instance,
    event,
    competition,
    location,
    organizer,
    author
RESTART IDENTITY CASCADE;

INSERT INTO organizer (id, name, website_url)
VALUES
    (1, 'Algorithm League', 'https://algorithm.example'),
    (2, 'Women Code Foundation', 'https://women-code.example'),
    (3, 'Inactive Organizer', NULL);

INSERT INTO location (id, parent_id, type, name)
VALUES
    (1, NULL, 'Continent', 'South America'),
    (2, 1, 'Country', 'Brazil'),
    (3, 1, 'Country', 'Argentina'),
    (4, 2, 'Province', 'State of Sao Paulo'),
    (5, 4, 'City', 'Campinas'),
    (6, 5, 'Campus', 'Alpha Campus'),
    (7, 2, 'Province', 'State of Rio de Janeiro'),
    (8, 7, 'City', 'Rio de Janeiro'),
    (9, 8, 'Campus', 'Beta Campus'),
    (10, 3, 'City', 'Buenos Aires'),
    (11, 10, 'Campus', 'Gamma Campus'),
    (12, NULL, 'Continent', 'Europe'),
    (13, 12, 'Country', 'Portugal'),
    (14, 13, 'City', 'Porto'),
    (15, 14, 'Campus', 'Ada Campus');

INSERT INTO competition (id, organizer_id, name, gender_category, website_url)
VALUES
    (10, 1, 'Global Algorithm Cup', 'Open', 'https://global-cup.example'),
    (20, 1, 'Regional Code League', 'Open', 'https://regional-league.example'),
    (30, 2, 'Women Challenge', 'FemaleOnly', 'https://women-challenge.example'),
    (40, 3, 'Unplayed Cup', 'Open', NULL);

INSERT INTO event (id, competition_id, name, level, scope)
VALUES
    (100, 10, 'Algorithm Final', 1, 'Global'),
    (101, 10, 'Algorithm Regional', 2, 'Regional'),
    (200, 20, 'Regional Final', 1, 'National'),
    (300, 30, 'Women Final', 1, 'International'),
    (400, 40, 'Unplayed Event', 1, 'National');

INSERT INTO event_instance (id, event_id, location_id, date)
VALUES
    (1000, 100, 10, '2023-09-10'),
    (1001, 100, 5, '2024-09-10'),
    (1002, 100, 8, '2025-09-10'),
    (1003, 100, 10, '2025-09-11'),
    (1010, 101, 10, '2024-06-01'),
    (1011, 101, 5, '2025-06-01'),
    (2000, 200, 5, '2024-11-01'),
    (2001, 200, 10, '2026-11-01'),
    (3000, 300, 14, '2025-07-01'),
    (4000, 400, 5, '2025-08-01');

INSERT INTO institution (id, name, short_name, site, main_location_id)
VALUES
    (1, 'Alpha University', 'AU', 'https://alpha.example', 6),
    (2, 'Beta Institute', 'BI', 'https://beta.example', 9),
    (3, 'Gamma University', 'GU', 'https://gamma.example', 11),
    (4, 'Ada College', 'AC', 'https://ada.example', 15),
    (5, 'Idle University', 'IU', NULL, 6);

INSERT INTO institution_location (institution_id, location_id)
VALUES
    (1, 6),
    (2, 9),
    (3, 11),
    (4, 15),
    (5, 6);

INSERT INTO team (id, name, institution_id)
VALUES
    (1, 'Alpha Coders', 1),
    (2, 'Alpha Bytes', 1),
    (3, 'Beta Stack', 2),
    (4, 'Gamma Graph', 3),
    (5, 'Ada Lovelace', 4),
    (6, 'Idle Team', 5);

INSERT INTO member (id, gender)
VALUES
    (1, 'Male'), (2, 'Female'), (3, 'Male'),
    (4, 'Female'), (5, 'Female'), (6, 'Male'),
    (7, 'Male'), (8, 'Other'), (9, 'Female'),
    (10, 'Female'), (11, 'Male'), (12, 'Male'),
    (13, 'Female'), (14, 'Female'), (15, 'Female'),
    (16, 'RatherNotAnswer');

INSERT INTO team_contestant (team_id, member_id)
VALUES
    (1, 1), (1, 2),
    (2, 4), (2, 5),
    (3, 7), (3, 8),
    (4, 10), (4, 11),
    (5, 13), (5, 14),
    (6, 15), (6, 16);

INSERT INTO team_event (id, team_id, event_instance_id, campus_location_id, rank)
VALUES
    (1, 1, 1000, 6, 2),
    (2, 3, 1000, 9, 1),
    (3, 1, 1001, 6, 3),
    (4, 2, 1001, 6, 1),
    (5, 3, 1001, 9, 2),
    (6, 4, 1001, 11, 4),
    (7, 1, 1002, 6, 1),
    (8, 2, 1002, 6, 3),
    (9, 3, 1002, 9, 2),
    (10, 1, 1010, NULL, 2),
    (11, 3, 1010, NULL, 1),
    (12, 2, 1011, 6, 1),
    (13, 4, 1011, 11, 2),
    (14, 1, 2000, 6, 4),
    (15, 4, 2000, 11, 1),
    (16, 1, 2001, 6, 2),
    (17, 4, 2001, 11, 1),
    (18, 5, 3000, 15, 1);

INSERT INTO team_event_member (member_id, team_event_id, role)
SELECT membership.member_id, te.id, membership.member_role
FROM team_event te
JOIN (
    VALUES
        (1, 3, 'Coach'::role),
        (2, 6, 'Coach'::role), (2, 16, 'Reserve'::role),
        (3, 9, 'Coach'::role),
        (4, 12, 'Coach'::role),
        (5, 15, 'Coach'::role)
) AS membership(team_id, member_id, member_role)
    ON membership.team_id = te.team_id;

SELECT setval(pg_get_serial_sequence('organizer', 'id'), (SELECT MAX(id) FROM organizer));
SELECT setval(pg_get_serial_sequence('location', 'id'), (SELECT MAX(id) FROM location));
SELECT setval(pg_get_serial_sequence('competition', 'id'), (SELECT MAX(id) FROM competition));
SELECT setval(pg_get_serial_sequence('event', 'id'), (SELECT MAX(id) FROM event));
SELECT setval(pg_get_serial_sequence('event_instance', 'id'), (SELECT MAX(id) FROM event_instance));
SELECT setval(pg_get_serial_sequence('institution', 'id'), (SELECT MAX(id) FROM institution));
SELECT setval(pg_get_serial_sequence('team', 'id'), (SELECT MAX(id) FROM team));
SELECT setval(pg_get_serial_sequence('team_event', 'id'), (SELECT MAX(id) FROM team_event));
SELECT setval(pg_get_serial_sequence('member', 'id'), (SELECT MAX(id) FROM member));

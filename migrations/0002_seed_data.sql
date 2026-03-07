-- =========================
-- ORGANIZER
-- =========================

INSERT INTO organizer (id, name, website_url)
VALUES
	(1, 'ICPC', 'https://icpc.global'),
	(2, 'SBC', 'https://www.sbc.org.br'),
	(3, 'OBI Committee', 'https://olimpiada.ic.unicamp.br'),
	(4, 'Codeforces', 'https://codeforces.com');

-- =========================
-- LOCATION
-- =========================

INSERT INTO location (id, parent_id, type, name)
VALUES
	(1, NULL, 'continent', 'South America'),
	(2, NULL, 'continent', 'North America'),
	(3, NULL, 'continent', 'Europe'),
	(4, 1, 'country', 'Brazil'),
	(5, 1, 'country', 'Argentina'),
	(6, 1, 'country', 'Chile'),
	(7, 1, 'country', 'Colombia'),
	(8, 2, 'country', 'United States'),
	(9, 2, 'country', 'Canada'),
	(10, 3, 'country', 'Portugal'),
	(11, 3, 'country', 'Spain'),
	(12, 4, 'city', 'Sao Paulo'),
	(13, 4, 'city', 'Campinas'),
	(14, 4, 'city', 'Rio de Janeiro'),
	(15, 5, 'city', 'Buenos Aires'),
	(16, 6, 'city', 'Santiago'),
	(17, 7, 'city', 'Bogota'),
	(18, 8, 'city', 'New York'),
	(19, 9, 'city', 'Toronto'),
	(20, 10, 'city', 'Porto'),
	(21, 11, 'city', 'Madrid'),
	(22, 4, 'city', 'Belo Horizonte'),
	(23, 12, 'campus', 'USP'),
	(24, 13, 'campus', 'UNICAMP'),
	(25, 14, 'campus', 'UFRJ'),
	(26, 15, 'campus', 'UBA'),
	(27, 16, 'campus', 'PUC Chile'),
	(28, 17, 'campus', 'Uniandes'),
	(29, 18, 'campus', 'MIT'),
	(30, 19, 'campus', 'UofT'),
	(31, 20, 'campus', 'UPorto'),
	(32, 21, 'campus', 'UPM'),
	(33, 22, 'campus', 'UFMG');

-- =========================
-- COMPETITION
-- =========================

INSERT INTO competition (id, organizer_id, name, gender_category, website_url)
VALUES
	(1, 1, 'ICPC World Championship', 'Open', 'https://icpc.global/worldfinals/'),
	(2, 1, 'ICPC Latin America Championship', 'Open', 'https://icpc.global/regionals/latin-america/'),
	(3, 1, 'ICPC South America Regional', 'Open', 'https://icpc.global/regionals/'),
	(4, 2, 'Maratona SBC', 'Open', 'https://maratona.sbc.org.br'),
	(5, 3, 'OBI Feminina', 'FemaleOnly', 'https://olimpiada.ic.unicamp.br'),
	(6, 3, 'OBI Programacao', 'Open', 'https://olimpiada.ic.unicamp.br'),
	(7, 4, 'Codeforces Gym South America', 'Open', 'https://codeforces.com/gyms'),
	(8, 4, 'Codeforces Gym Women Cup', 'FemaleOnly', 'https://codeforces.com/gyms');

-- =========================
-- EVENT
-- =========================

INSERT INTO event (id, competition_id, location_id, name, level, date)
VALUES
	(1, 1, 21, 'ICPC World Finals 2024', 1, '2024-09-15'),
	(2, 1, 18, 'ICPC World Finals 2025', 1, '2025-09-20'),
	(3, 2, 12, 'ICPC Latin America Finals 2024', 1, '2024-06-17'),
	(4, 2, 15, 'ICPC Latin America Finals 2025', 1, '2025-06-16'),
	(5, 3, 14, 'ICPC South America Regional Brazil 2024', 2, '2024-10-12'),
	(6, 3, 16, 'ICPC South America Regional Chile 2025', 2, '2025-10-11'),
	(7, 4, 13, 'Maratona SBC Finals 2024', 1, '2024-11-09'),
	(8, 4, 22, 'Maratona SBC Finals 2025', 1, '2025-11-08'),
	(9, 5, 12, 'OBI Feminina Nacional 2024', 2, '2024-08-24'),
	(10, 5, 14, 'OBI Feminina Nacional 2025', 2, '2025-08-23'),
	(11, 6, 12, 'OBI Nacional 2024', 2, '2024-09-28'),
	(12, 6, 13, 'OBI Nacional 2025', 2, '2025-09-27'),
	(13, 7, 15, 'Codeforces Gym South America Round 1', 3, '2024-04-06'),
	(14, 7, 17, 'Codeforces Gym South America Round 2', 3, '2025-04-05'),
	(15, 7, 12, 'Codeforces Gym South America Finals', 2, '2025-12-13'),
	(16, 8, 20, 'Codeforces Gym Women Cup Round 1', 3, '2024-05-04'),
	(17, 8, 21, 'Codeforces Gym Women Cup Round 2', 3, '2025-05-03'),
	(18, 8, 19, 'Codeforces Gym Women Cup Finals', 2, '2026-01-17');

-- =========================
-- INSTITUTION
-- =========================

INSERT INTO institution (id, name, short_name, site)
VALUES
	(1, 'Universidade de Sao Paulo', 'USP', 'https://www.usp.br'),
	(2, 'Universidade Estadual de Campinas', 'UNICAMP', 'https://www.unicamp.br'),
	(3, 'Universidade Federal do Rio de Janeiro', 'UFRJ', 'https://ufrj.br'),
	(4, 'Universidad de Buenos Aires', 'UBA', 'https://www.uba.ar'),
	(5, 'Pontificia Universidad Catolica de Chile', 'PUC Chile', 'https://www.uc.cl'),
	(6, 'Universidad de los Andes', 'Uniandes', 'https://uniandes.edu.co'),
	(7, 'Massachusetts Institute of Technology', 'MIT', 'https://web.mit.edu'),
	(8, 'University of Toronto', 'UofT', 'https://www.utoronto.ca'),
	(9, 'Universidade do Porto', 'UPorto', 'https://www.up.pt'),
	(10, 'Universidad Politecnica de Madrid', 'UPM', 'https://www.upm.es'),
	(11, 'Instituto Tecnologico de Buenos Aires', 'ITBA', 'https://www.itba.edu.ar'),
	(12, 'Universidade Federal de Minas Gerais', 'UFMG', 'https://www.ufmg.br');

INSERT INTO institution_location (institution_id, location_id)
VALUES
	(1, 23),
	(2, 24),
	(3, 25),
	(4, 26),
	(5, 27),
	(6, 28),
	(7, 29),
	(8, 30),
	(9, 31),
	(10, 32),
	(11, 15),
	(12, 33);

-- =========================
-- TEAM
-- =========================

INSERT INTO team (id, name, institution_id)
VALUES
	(1, 'USP Coders', 1),
	(2, 'USP Bitwise', 1),
	(3, 'UNICAMP Dynamic', 2),
	(4, 'UNICAMP Overflow', 2),
	(5, 'UFRJ Lambda', 3),
	(6, 'UFRJ Cariocas', 3),
	(7, 'UBA Recursion', 4),
	(8, 'UBA Pointers', 4),
	(9, 'PUC Chile Andes', 5),
	(10, 'PUC Chile Runtime', 5),
	(11, 'Uniandes Graphers', 6),
	(12, 'Uniandes DP Squad', 6),
	(13, 'MIT Blackboard', 7),
	(14, 'MIT Infinite Loop', 7),
	(15, 'UofT Maple', 8),
	(16, 'UofT Northern Lights', 8),
	(17, 'UPorto Atlantic', 9),
	(18, 'UPorto Stack', 9),
	(19, 'UPM Iberian Bits', 10),
	(20, 'UPM Madrid Hackers', 10),
	(21, 'ITBA Compilers', 11),
	(22, 'ITBA Fibonaccis', 11),
	(23, 'UFMG Ouro Preto', 12),
	(24, 'UFMG Inconfidentes', 12);

-- =========================
-- TEAM_EVENT
-- =========================

INSERT INTO team_event (id, team_id, event_id, rank)
VALUES
	(1, 1, 3, 1),
	(2, 3, 3, 2),
	(3, 5, 3, 3),
	(4, 7, 3, 4),
	(5, 9, 3, 5),
	(6, 11, 3, 6),
	(7, 2, 4, 1),
	(8, 4, 4, 2),
	(9, 6, 4, 3),
	(10, 8, 4, 4),
	(11, 10, 4, 5),
	(12, 12, 4, 6),
	(13, 1, 5, 1),
	(14, 2, 5, 2),
	(15, 3, 5, 3),
	(16, 4, 5, 4),
	(17, 5, 5, 5),
	(18, 6, 5, 6),
	(19, 23, 5, 7),
	(20, 24, 5, 8),
	(21, 7, 6, 1),
	(22, 8, 6, 2),
	(23, 9, 6, 3),
	(24, 10, 6, 4),
	(25, 11, 6, 5),
	(26, 12, 6, 6),
	(27, 17, 6, 7),
	(28, 18, 6, 8),
	(29, 1, 7, 1),
	(30, 3, 7, 2),
	(31, 5, 7, 3),
	(32, 23, 7, 4),
	(33, 24, 7, 5),
	(34, 2, 7, 6),
	(35, 2, 8, 1),
	(36, 4, 8, 2),
	(37, 6, 8, 3),
	(38, 23, 8, 4),
	(39, 24, 8, 5),
	(40, 1, 8, 6),
	(41, 2, 9, 1),
	(42, 4, 9, 2),
	(43, 6, 9, 3),
	(44, 8, 9, 4),
	(45, 10, 9, 5),
	(46, 12, 9, 6),
	(47, 15, 9, 7),
	(48, 16, 9, 8),
	(49, 1, 10, 1),
	(50, 3, 10, 2),
	(51, 5, 10, 3),
	(52, 7, 10, 4),
	(53, 9, 10, 5),
	(54, 11, 10, 6),
	(55, 17, 10, 7),
	(56, 19, 10, 8),
	(57, 7, 13, 1),
	(58, 11, 13, 2),
	(59, 17, 13, 3),
	(60, 21, 13, 4),
	(61, 23, 13, 5),
	(62, 8, 14, 1),
	(63, 12, 14, 2),
	(64, 18, 14, 3),
	(65, 22, 14, 4),
	(66, 24, 14, 5),
	(67, 1, 15, 1),
	(68, 9, 15, 2),
	(69, 13, 15, 3),
	(70, 15, 15, 4),
	(71, 19, 15, 5),
	(72, 10, 16, 1),
	(73, 12, 16, 2),
	(74, 16, 16, 3),
	(75, 18, 16, 4),
	(76, 20, 16, 5),
	(77, 9, 17, 1),
	(78, 11, 17, 2),
	(79, 15, 17, 3),
	(80, 17, 17, 4),
	(81, 19, 17, 5),
	(82, 14, 18, 1),
	(83, 16, 18, 2),
	(84, 18, 18, 3),
	(85, 20, 18, 4),
	(86, 22, 18, 5);

-- =========================
-- MEMBER
-- =========================

INSERT INTO member (id, gender)
SELECT
	gs,
	CASE gs % 4
		WHEN 0 THEN 'Female'::gender
		WHEN 1 THEN 'Male'::gender
		WHEN 2 THEN 'Other'::gender
		ELSE 'RatherNotAnswer'::gender
	END
FROM generate_series(1, 258) AS gs;

-- =========================
-- TEAM_EVENT_MEMBER
-- =========================

INSERT INTO team_event_member (member_id, team_event_id, role)
SELECT
	((te_id - 1) * 3) + role_map.member_offset AS member_id,
	te_id AS team_event_id,
	role_map.member_role AS role
FROM generate_series(1, 86) AS te_id
CROSS JOIN (
	VALUES
		(1, 'Contestant'::role),
		(2, 'Contestant'::role),
		(3, 'Coach'::role)
) AS role_map(member_offset, member_role);

-- =========================
-- PROBLEM
-- =========================

INSERT INTO problem (id, event_id, item, title, statement)
VALUES
	(1, 3, 'A', 'Warmup Sum', 'Given two integers, output their sum.'),
	(2, 3, 'B', 'Maximum Prefix', 'Find the maximum prefix sum of an array.'),
	(3, 3, 'C', 'Campus Routes', 'Count shortest routes between two buildings.'),
	(4, 4, 'A', 'Power Pair', 'Compute a^b modulo m for multiple queries.'),
	(5, 4, 'B', 'Scheduling Labs', 'Assign labs minimizing overall lateness.'),
	(6, 4, 'C', 'Regional Network', 'Find articulation points in a graph.'),
	(7, 5, 'A', 'Brazilian Coin Change', 'Minimize number of coins for a target value.'),
	(8, 5, 'B', 'River Crossing', 'Determine if all teams can cross a river safely.'),
	(9, 5, 'C', 'Contest Replay', 'Reconstruct accepted runs timeline.'),
	(10, 6, 'A', 'Andes Skyline', 'Compute visible mountain pairs.'),
	(11, 6, 'B', 'Regional Ranking', 'Sort teams with tie-break rules.'),
	(12, 6, 'C', 'Packet Delay', 'Find the minimum delay path in weighted graph.'),
	(13, 7, 'A', 'Matrix Walk', 'Count paths in a matrix with blocked cells.'),
	(14, 7, 'B', 'String Rotation', 'Find lexicographically smallest rotation.'),
	(15, 7, 'C', 'Judge Queue', 'Simulate priority queue for submissions.'),
	(16, 8, 'A', 'Mining Logs', 'Aggregate and query event logs.'),
	(17, 8, 'B', 'Sorted Buckets', 'Maintain buckets under updates and queries.'),
	(18, 8, 'C', 'Travel Budget', 'Minimize travel cost with discount edges.'),
	(19, 13, 'A', 'Gym Sprint', 'Simple implementation challenge.'),
	(20, 13, 'B', 'Sparse Grid', 'Answer range queries on sparse coordinates.'),
	(21, 13, 'C', 'Bitwise Parade', 'Maximize score using bitwise operations.'),
	(22, 14, 'A', 'Array Craft', 'Build array satisfying constraints.'),
	(23, 14, 'B', 'Two Trees', 'Compare diameters of two trees.'),
	(24, 14, 'C', 'Modulo Paths', 'Count paths with modulo condition.'),
	(25, 15, 'A', 'Final Warmup', 'Compute sums for many test cases.'),
	(26, 15, 'B', 'Final DP', 'Optimize score with dynamic programming.'),
	(27, 15, 'C', 'Final Graph', 'Process dynamic connectivity queries.'),
	(28, 16, 'A', 'Women Cup Intro', 'Implementation and parsing task.'),
	(29, 16, 'B', 'Women Cup Geometry', 'Compute polygon perimeter after edits.'),
	(30, 16, 'C', 'Women Cup Trees', 'Support subtree aggregation queries.'),
	(31, 17, 'A', 'Round Two Intro', 'Greedy arrangement problem.'),
	(32, 17, 'B', 'Round Two DS', 'Process online updates with segment tree.'),
	(33, 17, 'C', 'Round Two Flow', 'Compute max flow in directed graph.'),
	(34, 18, 'A', 'Grand Final Intro', 'Fast I/O and arithmetic basics.'),
	(35, 18, 'B', 'Grand Final Strings', 'Pattern matching with automata.'),
	(36, 18, 'C', 'Grand Final Optimization', 'Minimize objective under constraints.');

-- =========================
-- INPUT_OUTPUT
-- =========================

INSERT INTO input_output (problem_id, input, output)
SELECT
	p.id,
	'Sample input for problem ' || p.item || ' (event ' || p.event_id || ')',
	'Sample output for problem ' || p.item || ' (event ' || p.event_id || ')'
FROM problem AS p;

INSERT INTO input_output (problem_id, input, output)
SELECT
	p.id,
	'Second sample for problem ' || p.item,
	'Second output for problem ' || p.item
FROM problem AS p
WHERE p.item IN ('A', 'C');

-- =========================
-- AUTHOR
-- =========================

INSERT INTO author (id, name, nationality)
VALUES
	(1, 'John Doe', 'USA'),
	(2, 'Maria Silva', 'Brazil'),
	(3, 'Carla Mendoza', 'Argentina'),
	(4, 'Pedro Alvarez', 'Chile'),
	(5, 'Laura Gomez', 'Colombia'),
	(6, 'Sofia Martins', 'Portugal'),
	(7, 'Daniel Torres', 'Spain'),
	(8, 'Aisha Khan', 'Canada');

-- =========================
-- AUTHORSHIP
-- =========================

INSERT INTO authorship (author_id, problem_id)
SELECT ((p.id - 1) % 8) + 1, p.id
FROM problem AS p;

INSERT INTO authorship (author_id, problem_id)
SELECT ((p.id + 3) % 8) + 1, p.id
FROM problem AS p
WHERE p.id % 2 = 0;

-- =========================
-- SUBMISSION
-- =========================

INSERT INTO submission (status, language, code, submission_time, team_event_id, problem_id)
SELECT
	CASE (te.id + p.id) % 7
		WHEN 0 THEN 'Accepted'::status
		WHEN 1 THEN 'WrongAnswer'::status
		WHEN 2 THEN 'TimeLimitExceeded'::status
		WHEN 3 THEN 'MemoryLimitExceeded'::status
		WHEN 4 THEN 'PresentationError'::status
		WHEN 5 THEN 'CompilationError'::status
		ELSE 'RuntimeError'::status
	END,
	CASE te.id % 4
		WHEN 0 THEN 'C++'
		WHEN 1 THEN 'Python'
		WHEN 2 THEN 'Java'
		ELSE 'Rust'
	END,
	'/* seed submission for team_event ' || te.id || ' and problem ' || p.id || ' */',
	now() - ((te.id + p.id) || ' minutes')::interval,
	te.id,
	p.id
FROM team_event AS te
JOIN problem AS p
	ON p.event_id = te.event_id
WHERE p.item IN ('A', 'B');

-- Keep serial sequences aligned after explicit id inserts.
SELECT setval(pg_get_serial_sequence('organizer', 'id'), (SELECT MAX(id) FROM organizer));
SELECT setval(pg_get_serial_sequence('location', 'id'), (SELECT MAX(id) FROM location));
SELECT setval(pg_get_serial_sequence('competition', 'id'), (SELECT MAX(id) FROM competition));
SELECT setval(pg_get_serial_sequence('event', 'id'), (SELECT MAX(id) FROM event));
SELECT setval(pg_get_serial_sequence('institution', 'id'), (SELECT MAX(id) FROM institution));
SELECT setval(pg_get_serial_sequence('team', 'id'), (SELECT MAX(id) FROM team));
SELECT setval(pg_get_serial_sequence('team_event', 'id'), (SELECT MAX(id) FROM team_event));
SELECT setval(pg_get_serial_sequence('member', 'id'), (SELECT MAX(id) FROM member));
SELECT setval(pg_get_serial_sequence('problem', 'id'), (SELECT MAX(id) FROM problem));
SELECT setval(pg_get_serial_sequence('author', 'id'), (SELECT MAX(id) FROM author));
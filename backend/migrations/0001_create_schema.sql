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
  'Continent',
  'Country',
  'Region',
  'Province',
  'Prefecture',
  'City',
  'Campus'
);
CREATE TYPE scope AS ENUM (
  'Global',
  'InterContinental',
  'Continental',
  'International',
  'National',
  'InterRegional',
  'Regional',
  'Internal'
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
  name VARCHAR NOT NULL,
  level INT,
  scope scope NOT NULL,
  UNIQUE (competition_id, name)
);

CREATE INDEX idx_event_competition_id ON event(competition_id);

CREATE TABLE event_instance (
  id SERIAL PRIMARY KEY,
  event_id INT NOT NULL REFERENCES event(id),
  location_id INT NOT NULL REFERENCES location(id),
  date DATE NOT NULL,
  UNIQUE (event_id, date)
);

CREATE INDEX idx_event_instance_event_id ON event_instance(event_id);
CREATE INDEX idx_event_instance_location_id ON event_instance(location_id);

CREATE TABLE institution (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  short_name TEXT,
  site TEXT,
  main_location_id INT REFERENCES location(id)
);

CREATE INDEX idx_institution_main_location_id ON institution(main_location_id);

CREATE TABLE institution_location (
  institution_id INT REFERENCES institution(id),
  location_id INT REFERENCES location(id),
  PRIMARY KEY (institution_id, location_id)
);

CREATE TABLE team (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  institution_id INT NOT NULL REFERENCES institution(id)
);

CREATE INDEX idx_team_institution_id ON team(institution_id);
CREATE INDEX idx_team_name_institution_id ON team(name, institution_id);

CREATE TABLE problem (
  id SERIAL PRIMARY KEY,
  event_instance_id INT NOT NULL REFERENCES event_instance(id),
  item VARCHAR(10) NOT NULL,
  title TEXT NOT NULL,
  statement TEXT NOT NULL,
  UNIQUE (event_instance_id, item)
);

CREATE INDEX idx_problem_event_instance_id ON problem(event_instance_id);

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
  event_instance_id INT NOT NULL REFERENCES event_instance(id),
  campus_location_id INT REFERENCES location(id),
  rank INT NOT NULL,
  UNIQUE (team_id, event_instance_id)
);

CREATE INDEX idx_team_event_team_id ON team_event(team_id);
CREATE INDEX idx_team_event_event_instance_id ON team_event(event_instance_id);
CREATE INDEX idx_team_event_campus_location_id ON team_event(campus_location_id);

CREATE TABLE member (
  id SERIAL PRIMARY KEY,
  gender gender NOT NULL
);

-- Contestants are part of a team's identity. Event-specific participation is
-- mirrored in team_event_member so that roles remain explicit at that level.
CREATE TABLE team_contestant (
  team_id INT NOT NULL REFERENCES team(id) ON DELETE CASCADE,
  member_id INT NOT NULL REFERENCES member(id),
  PRIMARY KEY (team_id, member_id)
);

CREATE INDEX idx_team_contestant_member_id ON team_contestant(member_id);

CREATE TABLE team_event_member (
  member_id INT NOT NULL REFERENCES member(id),
  team_event_id INT NOT NULL REFERENCES team_event(id) ON DELETE CASCADE,
  role role NOT NULL,
  PRIMARY KEY (member_id, team_event_id)
);

CREATE INDEX idx_team_event_member_team_event_id ON team_event_member(team_event_id);

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

-- =========================
-- FUNCTIONS
-- =========================
CREATE OR REPLACE FUNCTION get_location_tree(start_location_id INT)
RETURNS TABLE (
    id INT,
    parent_id INT,
    name TEXT,
    type location_type,
    depth INT
)
LANGUAGE sql
AS $$
WITH RECURSIVE location_tree AS (
    -- anchor
    SELECT
        l.id,
        l.parent_id,
        l.name,
        l.type,
        1 AS depth
    FROM location l
    WHERE l.id = start_location_id

    UNION ALL

    -- recursive
    SELECT
        parent.id,
        parent.parent_id,
        parent.name,
        parent.type,
        lt.depth + 1
    FROM location parent
    JOIN location_tree lt
        ON parent.id = lt.parent_id
)
SELECT * FROM location_tree;
$$;

-- =========================
-- TEAM IDENTITY AND ROSTER INTEGRITY
-- =========================

-- PostgreSQL cannot express equality between child-row sets with a regular
-- UNIQUE constraint. This deferred assertion compares the canonical,
-- order-independent set of Contestants after the surrounding transaction has
-- finished assembling it.
CREATE OR REPLACE FUNCTION assert_team_identity(candidate_team_id INT)
RETURNS VOID
LANGUAGE plpgsql
AS $function$
DECLARE
  candidate_name VARCHAR;
  candidate_institution_id INT;
  candidate_contestants INT[];
BEGIN
  -- This lock is shared by every mutation of the three identity components
  -- and by creation of a participation. It prevents a roster from changing
  -- while it is being copied into team_event_member.
  PERFORM pg_advisory_xact_lock(
    hashtext('team_identity'),
    candidate_team_id
  );

  SELECT name, institution_id
  INTO candidate_name, candidate_institution_id
  FROM team
  WHERE id = candidate_team_id;

  -- A deleted team no longer has an identity to validate.
  IF NOT FOUND THEN
    RETURN;
  END IF;

  -- Teams with the same visible identity are serialized so concurrent
  -- transactions cannot create equal Contestant sets.
  PERFORM pg_advisory_xact_lock(
    candidate_institution_id,
    hashtext(candidate_name)
  );

  SELECT ARRAY(
    SELECT tc.member_id
    FROM team_contestant AS tc
    WHERE tc.team_id = candidate_team_id
    ORDER BY tc.member_id
  )
  INTO candidate_contestants;

  IF cardinality(candidate_contestants) = 0 THEN
    RAISE EXCEPTION
      'team % must have at least one Contestant',
      candidate_team_id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_has_contestants';
  END IF;

  IF EXISTS (
    SELECT 1
    FROM team AS other_team
    WHERE other_team.id <> candidate_team_id
      AND other_team.name = candidate_name
      AND other_team.institution_id = candidate_institution_id
      AND ARRAY(
        SELECT other_tc.member_id
        FROM team_contestant AS other_tc
        WHERE other_tc.team_id = other_team.id
        ORDER BY other_tc.member_id
      ) = candidate_contestants
  ) THEN
    RAISE EXCEPTION
      'team % duplicates the name, institution and Contestant set of another team',
      candidate_team_id
      USING ERRCODE = '23505',
            CONSTRAINT = 'uq_team_identity';
  END IF;
END
$function$;

CREATE OR REPLACE FUNCTION validate_team_identity_from_team()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  PERFORM assert_team_identity(NEW.id);
  RETURN NULL;
END
$function$;

CREATE CONSTRAINT TRIGGER validate_team_identity_from_team
AFTER INSERT OR UPDATE OF name, institution_id
ON team
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION validate_team_identity_from_team();

CREATE OR REPLACE FUNCTION validate_team_identity_from_contestant()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF TG_OP = 'INSERT' THEN
    PERFORM assert_team_identity(NEW.team_id);
  ELSIF TG_OP = 'UPDATE' THEN
    PERFORM assert_team_identity(OLD.team_id);
    IF NEW.team_id IS DISTINCT FROM OLD.team_id THEN
      PERFORM assert_team_identity(NEW.team_id);
    END IF;
  ELSE
    PERFORM assert_team_identity(OLD.team_id);
  END IF;

  RETURN NULL;
END
$function$;

CREATE CONSTRAINT TRIGGER validate_team_identity_from_contestant
AFTER INSERT OR UPDATE OR DELETE
ON team_contestant
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION validate_team_identity_from_contestant();

CREATE OR REPLACE FUNCTION reject_participating_team_attribute_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.name IS NOT DISTINCT FROM OLD.name
    AND NEW.institution_id IS NOT DISTINCT FROM OLD.institution_id
  THEN
    RETURN NEW;
  END IF;

  PERFORM pg_advisory_xact_lock(hashtext('team_identity'), OLD.id);

  IF EXISTS (
    SELECT 1
    FROM team_event
    WHERE team_id = OLD.id
  ) THEN
    RAISE EXCEPTION
      'the name and institution of team % cannot change after the team has participated',
      OLD.id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_identity_immutable';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER reject_participating_team_attribute_change
BEFORE UPDATE OF name, institution_id
ON team
FOR EACH ROW
EXECUTE FUNCTION reject_participating_team_attribute_change();

CREATE OR REPLACE FUNCTION reject_participating_team_roster_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF TG_OP = 'INSERT' THEN
    PERFORM pg_advisory_xact_lock(hashtext('team_identity'), NEW.team_id);

    IF EXISTS (
      SELECT 1
      FROM team_event
      WHERE team_id = NEW.team_id
    ) THEN
      RAISE EXCEPTION
        'the Contestant set of team % cannot change after the team has participated',
        NEW.team_id
        USING ERRCODE = '23514',
              CONSTRAINT = 'ck_team_contestant_roster_immutable';
    END IF;

    RETURN NEW;
  END IF;

  IF TG_OP = 'UPDATE'
    AND NEW.team_id IS NOT DISTINCT FROM OLD.team_id
    AND NEW.member_id IS NOT DISTINCT FROM OLD.member_id
  THEN
    RETURN NEW;
  END IF;

  IF TG_OP = 'UPDATE' AND NEW.team_id IS DISTINCT FROM OLD.team_id THEN
    PERFORM pg_advisory_xact_lock(
      hashtext('team_identity'),
      LEAST(OLD.team_id, NEW.team_id)
    );
    PERFORM pg_advisory_xact_lock(
      hashtext('team_identity'),
      GREATEST(OLD.team_id, NEW.team_id)
    );
  ELSE
    PERFORM pg_advisory_xact_lock(hashtext('team_identity'), OLD.team_id);
  END IF;

  IF EXISTS (
      SELECT 1
      FROM team_event
      WHERE team_id = OLD.team_id
  ) THEN
    RAISE EXCEPTION
      'the Contestant set of team % cannot change after the team has participated',
      OLD.team_id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_contestant_roster_immutable';
  END IF;

  IF TG_OP = 'UPDATE'
    AND NEW.team_id IS DISTINCT FROM OLD.team_id
    AND EXISTS (
      SELECT 1
      FROM team_event
      WHERE team_id = NEW.team_id
    )
  THEN
    RAISE EXCEPTION
      'the Contestant set of team % cannot change after the team has participated',
      NEW.team_id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_contestant_roster_immutable';
  END IF;

  IF TG_OP = 'DELETE' THEN
    RETURN OLD;
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER reject_participating_team_roster_change
BEFORE INSERT OR UPDATE OR DELETE
ON team_contestant
FOR EACH ROW
EXECUTE FUNCTION reject_participating_team_roster_change();

-- A Contestant may participate in several events of the same annual
-- competition, but must represent one team throughout that competition-year.
-- The event-year check also prevents participation in two instances of the
-- same event during the same year.
CREATE OR REPLACE FUNCTION enforce_unique_contestant_registration()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
DECLARE
  registration_team_id INT;
  registration_competition_id INT;
  registration_event_id INT;
  registration_year INT;
  previous_member_id INT;
  previous_team_event_id INT;
BEGIN
  IF NEW.role IS DISTINCT FROM 'Contestant'::role THEN
    RETURN NEW;
  END IF;

  SELECT
    te.team_id,
    e.competition_id,
    e.id,
    EXTRACT(YEAR FROM ei.date)::INT
  INTO
    registration_team_id,
    registration_competition_id,
    registration_event_id,
    registration_year
  FROM team_event AS te
  JOIN event_instance AS ei ON ei.id = te.event_instance_id
  JOIN event AS e ON e.id = ei.event_id
  WHERE te.id = NEW.team_event_id
  FOR KEY SHARE OF te, ei, e;

  -- Let the foreign-key constraint report an unknown team_event_id.
  IF NOT FOUND THEN
    RETURN NEW;
  END IF;

  IF TG_OP = 'UPDATE' THEN
    previous_member_id := OLD.member_id;
    previous_team_event_id := OLD.team_event_id;
  END IF;

  -- These transaction-scoped locks close the races between each lookup and a
  -- concurrent registration for the same member and annual scope.
  PERFORM pg_advisory_xact_lock(
    NEW.member_id,
    hashtext(
      'competition:' || registration_competition_id::TEXT || ':' || registration_year::TEXT
    )
  );

  IF EXISTS (
    SELECT 1
    FROM team_event_member AS existing_membership
    JOIN team_event AS existing_team_event
      ON existing_team_event.id = existing_membership.team_event_id
    JOIN event_instance AS existing_instance
      ON existing_instance.id = existing_team_event.event_instance_id
    JOIN event AS existing_event
      ON existing_event.id = existing_instance.event_id
    WHERE existing_membership.member_id = NEW.member_id
      AND existing_membership.role = 'Contestant'::role
      AND existing_event.competition_id = registration_competition_id
      AND EXTRACT(YEAR FROM existing_instance.date)::INT = registration_year
      AND existing_team_event.team_id <> registration_team_id
      AND NOT (
        existing_membership.member_id IS NOT DISTINCT FROM previous_member_id
        AND existing_membership.team_event_id IS NOT DISTINCT FROM previous_team_event_id
      )
  ) THEN
    RAISE EXCEPTION
      'member % is already a Contestant for another team in competition % in %',
      NEW.member_id,
      registration_competition_id,
      registration_year
      USING ERRCODE = '23505',
            CONSTRAINT = 'uq_contestant_registration_competition_year';
  END IF;

  PERFORM pg_advisory_xact_lock(
    NEW.member_id,
    hashtext('event:' || registration_event_id::TEXT || ':' || registration_year::TEXT)
  );

  IF EXISTS (
    SELECT 1
    FROM team_event_member AS existing_membership
    JOIN team_event AS existing_team_event
      ON existing_team_event.id = existing_membership.team_event_id
    JOIN event_instance AS existing_instance
      ON existing_instance.id = existing_team_event.event_instance_id
    WHERE existing_membership.member_id = NEW.member_id
      AND existing_membership.role = 'Contestant'::role
      AND existing_instance.event_id = registration_event_id
      AND EXTRACT(YEAR FROM existing_instance.date)::INT = registration_year
      AND existing_team_event.id <> NEW.team_event_id
      AND NOT (
        existing_membership.member_id IS NOT DISTINCT FROM previous_member_id
        AND existing_membership.team_event_id IS NOT DISTINCT FROM previous_team_event_id
      )
  ) THEN
    RAISE EXCEPTION
      'member % is already a Contestant in event % in %',
      NEW.member_id,
      registration_event_id,
      registration_year
      USING ERRCODE = '23505',
            CONSTRAINT = 'uq_contestant_event_year';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER enforce_unique_contestant_registration
BEFORE INSERT OR UPDATE OF member_id, team_event_id, role
ON team_event_member
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_contestant_registration();

COMMENT ON FUNCTION enforce_unique_contestant_registration() IS
  'Keeps each Contestant on one team per competition-year and one instance per event-year.';

-- Contestant participation rows are derived from the team's canonical roster.
CREATE OR REPLACE FUNCTION populate_team_event_contestants()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
DECLARE
  inserted_contestants INT;
BEGIN
  PERFORM pg_advisory_xact_lock(hashtext('team_identity'), NEW.team_id);

  INSERT INTO team_event_member (member_id, team_event_id, role)
  SELECT tc.member_id, NEW.id, 'Contestant'::role
  FROM team_contestant AS tc
  WHERE tc.team_id = NEW.team_id
  ORDER BY tc.member_id;

  GET DIAGNOSTICS inserted_contestants = ROW_COUNT;

  IF inserted_contestants = 0 THEN
    RAISE EXCEPTION
      'team % must have a canonical Contestant roster before it participates',
      NEW.team_id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_has_contestants';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER populate_team_event_contestants
AFTER INSERT
ON team_event
FOR EACH ROW
EXECUTE FUNCTION populate_team_event_contestants();

CREATE OR REPLACE FUNCTION assert_team_event_contestant_roster(candidate_team_event_id INT)
RETURNS VOID
LANGUAGE plpgsql
AS $function$
DECLARE
  candidate_team_id INT;
  canonical_contestants INT[];
  participating_contestants INT[];
BEGIN
  SELECT team_id
  INTO candidate_team_id
  FROM team_event
  WHERE id = candidate_team_event_id;

  -- Cascading deletion of a team_event also removes its membership rows.
  IF NOT FOUND THEN
    RETURN;
  END IF;

  SELECT ARRAY(
    SELECT tc.member_id
    FROM team_contestant AS tc
    WHERE tc.team_id = candidate_team_id
    ORDER BY tc.member_id
  )
  INTO canonical_contestants;

  SELECT ARRAY(
    SELECT tem.member_id
    FROM team_event_member AS tem
    WHERE tem.team_event_id = candidate_team_event_id
      AND tem.role = 'Contestant'::role
    ORDER BY tem.member_id
  )
  INTO participating_contestants;

  IF participating_contestants IS DISTINCT FROM canonical_contestants THEN
    RAISE EXCEPTION
      'Contestants in team_event % must equal the canonical roster of team %',
      candidate_team_event_id,
      candidate_team_id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_team_event_contestant_roster';
  END IF;
END
$function$;

CREATE OR REPLACE FUNCTION validate_team_event_contestant_roster()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF TG_OP = 'INSERT' THEN
    PERFORM assert_team_event_contestant_roster(NEW.team_event_id);
  ELSIF TG_OP = 'UPDATE' THEN
    PERFORM assert_team_event_contestant_roster(OLD.team_event_id);
    IF NEW.team_event_id IS DISTINCT FROM OLD.team_event_id THEN
      PERFORM assert_team_event_contestant_roster(NEW.team_event_id);
    END IF;
  ELSE
    PERFORM assert_team_event_contestant_roster(OLD.team_event_id);
  END IF;

  RETURN NULL;
END
$function$;

CREATE CONSTRAINT TRIGGER validate_team_event_contestant_roster
AFTER INSERT OR UPDATE OR DELETE
ON team_event_member
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION validate_team_event_contestant_roster();

-- The competition and year of a registration are derived through
-- team_event, event_instance and event. Once Contestants exist, changing any
-- of those ancestors could otherwise invalidate the invariants retroactively.
CREATE OR REPLACE FUNCTION reject_contestant_team_event_scope_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.team_id IS NOT DISTINCT FROM OLD.team_id
    AND NEW.event_instance_id IS NOT DISTINCT FROM OLD.event_instance_id
  THEN
    RETURN NEW;
  END IF;

  IF EXISTS (
    SELECT 1
    FROM team_event_member
    WHERE team_event_id = OLD.id
      AND role = 'Contestant'::role
  ) THEN
    RAISE EXCEPTION
      'team_event % cannot change team or event instance while it has Contestants',
      OLD.id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_contestant_registration_scope_immutable';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER reject_contestant_team_event_scope_change
BEFORE UPDATE OF team_id, event_instance_id
ON team_event
FOR EACH ROW
EXECUTE FUNCTION reject_contestant_team_event_scope_change();

CREATE OR REPLACE FUNCTION reject_contestant_event_instance_scope_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.event_id IS NOT DISTINCT FROM OLD.event_id
    AND EXTRACT(YEAR FROM NEW.date)::INT = EXTRACT(YEAR FROM OLD.date)::INT
  THEN
    RETURN NEW;
  END IF;

  IF EXISTS (
    SELECT 1
    FROM team_event AS te
    JOIN team_event_member AS tem ON tem.team_event_id = te.id
    WHERE te.event_instance_id = OLD.id
      AND tem.role = 'Contestant'::role
  ) THEN
    RAISE EXCEPTION
      'event_instance % cannot change event or year while it has Contestants',
      OLD.id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_contestant_registration_scope_immutable';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER reject_contestant_event_instance_scope_change
BEFORE UPDATE OF event_id, date
ON event_instance
FOR EACH ROW
EXECUTE FUNCTION reject_contestant_event_instance_scope_change();

CREATE OR REPLACE FUNCTION reject_contestant_event_competition_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
  IF NEW.competition_id IS NOT DISTINCT FROM OLD.competition_id THEN
    RETURN NEW;
  END IF;

  IF EXISTS (
    SELECT 1
    FROM event_instance AS ei
    JOIN team_event AS te ON te.event_instance_id = ei.id
    JOIN team_event_member AS tem ON tem.team_event_id = te.id
    WHERE ei.event_id = OLD.id
      AND tem.role = 'Contestant'::role
  ) THEN
    RAISE EXCEPTION
      'event % cannot change competition while it has Contestants',
      OLD.id
      USING ERRCODE = '23514',
            CONSTRAINT = 'ck_contestant_registration_scope_immutable';
  END IF;

  RETURN NEW;
END
$function$;

CREATE TRIGGER reject_contestant_event_competition_change
BEFORE UPDATE OF competition_id
ON event
FOR EACH ROW
EXECUTE FUNCTION reject_contestant_event_competition_change();

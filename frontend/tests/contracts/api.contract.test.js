import { beforeAll, describe, expect, test } from 'bun:test';

const apiOrigin = (process.env.CONTRACT_API_URL || 'http://127.0.0.1:8000').replace(/\/$/, '');

function queryString(params = {}) {
  const query = new URLSearchParams();

  Object.entries(params).forEach(([key, value]) => {
    if (value !== null && value !== undefined && value !== '') {
      query.set(key, String(value));
    }
  });

  return query.size ? `?${query}` : '';
}

async function apiGet(path, params) {
  const response = await fetch(`${apiOrigin}${path}${queryString(params)}`, {
    headers: { Accept: 'application/json' },
  });
  const payload = await response.json().catch(() => null);

  expect(response.ok, `${path} returned ${response.status}: ${JSON.stringify(payload)}`).toBe(true);
  return payload;
}

function expectOption(option) {
  expect(Number.isInteger(option.id)).toBe(true);
  expect(typeof option.name).toBe('string');
}

function expectStats(stats) {
  for (const field of ['total_institutions', 'total_teams', 'total_participants', 'female_participants']) {
    expect(Number.isInteger(stats[field]), `${field} must be an integer`).toBe(true);
    expect(stats[field]).toBeGreaterThanOrEqual(0);
  }

  expect(stats.female_participants).toBeLessThanOrEqual(stats.total_participants);
}

let catalog;

beforeAll(async () => {
  const [organizers, competitions, institutions, teams] = await Promise.all([
    apiGet('/organizers/options'),
    apiGet('/competitions/options'),
    apiGet('/institutions/options'),
    apiGet('/teams/options'),
  ]);

  catalog = { organizers, competitions, institutions, teams };
});

describe('frontend-backend API contract', () => {
  test('option endpoints expose stable identifiers and labels', () => {
    for (const options of Object.values(catalog)) {
      expect(Array.isArray(options)).toBe(true);
      expect(options.length).toBeGreaterThan(0);
      options.forEach(expectOption);
    }
  });

  test('team options expose the institution required to disambiguate repeated names', () => {
    for (const team of catalog.teams) {
      expect(Number.isInteger(team.institution_id)).toBe(true);
      expect(typeof team.institution_name).toBe('string');
      expect(team.institution_short_name === null || typeof team.institution_short_name === 'string').toBe(
        true,
      );
    }
  });

  test('competition snapshots and annual endpoints declare compatible temporal scopes', async () => {
    const competitionId = catalog.competitions[0].id;
    const [snapshot] = await apiGet('/competitions/structures', {
      competition_ids: competitionId,
    });

    expect(snapshot.id).toBe(competitionId);
    expect(snapshot.snapshot_year).toBe(Math.max(...snapshot.years));
    expect(Array.isArray(snapshot.events)).toBe(true);

    snapshot.events
      .flatMap((event) => event.teams)
      .forEach((entry) => {
        expect(Number.isInteger(entry.id)).toBe(true);
        expect(Number.isInteger(entry.institution_id)).toBe(true);
      });

    const [annual, stats] = await Promise.all([
      apiGet(`/competitions/${competitionId}/structure`, {
        year: snapshot.snapshot_year,
      }),
      apiGet(`/competitions/${competitionId}/stats`, {
        year: snapshot.snapshot_year,
      }),
    ]);

    expect(Array.isArray(annual.events)).toBe(true);
    expectStats(stats);

    const teamEntries = annual.events.flatMap((event) => event.teams || []);
    const participantEntries = teamEntries.reduce(
      (total, team) => total + Number(team.total_members || 0),
      0,
    );
    const femaleParticipantEntries = teamEntries.reduce(
      (total, team) => total + Number(team.female_participants || 0),
      0,
    );

    expect(stats.total_participants).toBeLessThanOrEqual(participantEntries);
    expect(stats.female_participants).toBeLessThanOrEqual(femaleParticipantEntries);
  });

  test('event structure is the authoritative source for identity, years and occurrences', async () => {
    const competitionId = catalog.competitions[0].id;
    const [competition] = await apiGet('/competitions/structures', {
      competition_ids: competitionId,
    });
    const event = competition.events[0];
    const structure = await apiGet(`/events/${event.id}/structure`, {
      year: competition.snapshot_year,
    });

    expect(structure.id).toBe(event.id);
    expect(structure.year).toBe(competition.snapshot_year);
    expect(structure.years).toContain(structure.year);
    expect(structure.competition.id).toBe(competitionId);
    expect(structure.instances.length).toBeGreaterThan(0);
    structure.instances.forEach((instance) => {
      expect(new Date(`${instance.date}T00:00:00`).getFullYear()).toBe(structure.year);
    });

    expectStats(await apiGet(`/events/${event.id}/stats`, { year: structure.year }));
  });

  test('institution event options cover the historical performance contract', async () => {
    const institutionId = catalog.institutions[0].id;
    const options = await apiGet(`/institutions/${institutionId}/events/options`);

    expect(options.length).toBeGreaterThan(0);
    const event = options[0];
    expectOption(event);
    expect(Number.isInteger(event.competition_id)).toBe(true);
    expect(event.years.length).toBeGreaterThan(0);

    const performance = await apiGet(`/institutions/${institutionId}/events/${event.id}/performance`, {
      start_year: Math.min(...event.years),
      end_year: Math.max(...event.years),
    });
    expect(Array.isArray(performance)).toBe(true);
    performance.forEach((row) => {
      expect(event.years).toContain(row.year);
      expect(typeof row.average_performance_rank).toBe('number');
    });
  });

  test('team annual totals belong to the requested competition-year', async () => {
    const teamId = catalog.teams[0].id;
    const [team] = await apiGet('/teams/structures', { team_ids: teamId });
    const competition = team.competitions[0];
    const annual = await apiGet(`/teams/${teamId}/competitions/${competition.id}/structure`, {
      year: competition.snapshot_year,
    });

    expect(Number.isInteger(annual.total_members)).toBe(true);
    expect(Number.isInteger(annual.female_participants)).toBe(true);
    expect(annual.female_participants).toBeLessThanOrEqual(annual.total_members);
    expect(Array.isArray(annual.events)).toBe(true);
  });
});

import { afterEach, beforeEach, describe, expect, test } from 'bun:test';

import { clearApiCache } from '../../app/src/lib/api.js';
import {
  clearDataStoreCache,
  getOptionLabel,
  getSearchCatalog,
  getUniverseSnapshot,
} from '../../app/src/lib/data-store.js';

const originalFetch = globalThis.fetch;
const originalDateNow = Date.now;

const organizerOptions = [{ id: 1, name: 'Mock Organizer' }];
const competitionOptions = [{ id: 2, name: 'Mock Competition' }];
const institutionOptions = [{ id: 3, name: 'Mock University' }];
const teamOptions = [{ id: 4, name: 'Mock Team' }];
const competitionStructures = [
  {
    id: 2,
    name: 'Mock Competition',
    events: [
      {
        id: 10,
        name: 'Regional',
        date: '2024-05-10',
        location: 'Campinas',
        location_types: ['Country', 'City'],
      },
      {
        id: 11,
        name: 'Final',
        date: '2025-06-15',
        location: 'Sao Paulo',
        location_types: ['Country', 'City'],
      },
    ],
  },
];

const payloadsByPath = new Map([
  ['/api/organizers/options', organizerOptions],
  ['/api/competitions/options', competitionOptions],
  ['/api/institutions/options', institutionOptions],
  ['/api/teams/options', teamOptions],
  ['/api/competitions/structures', competitionStructures],
  ['/api/organizers/structures', [{ id: 1, name: 'Mock Organizer', competitions: [] }]],
  ['/api/institutions/structures', [{ id: 3, name: 'Mock University', competitions: [] }]],
  ['/api/teams/structures', [{ id: 4, name: 'Mock Team', competitions: [] }]],
]);

function jsonResponse(payload, status = 200) {
  return new Response(JSON.stringify(payload), {
    status,
    headers: { 'Content-Type': 'application/json' },
  });
}

function installApiFetch({ empty = false, failOrganizerOnce = false } = {}) {
  const requests = [];
  let organizerAttempts = 0;

  globalThis.fetch = async (url) => {
    const parsed = new URL(url, 'http://frontend.test');
    requests.push(parsed);

    if (parsed.pathname === '/api/organizers/options') {
      organizerAttempts += 1;
      if (failOrganizerOnce && organizerAttempts === 1) {
        return jsonResponse({ error: 'temporary catalog failure' }, 503);
      }
    }

    const payload = empty ? [] : payloadsByPath.get(parsed.pathname);
    if (payload === undefined) {
      throw new Error(`Unhandled unit-test API path: ${parsed.pathname}`);
    }

    return jsonResponse(payload);
  };

  return requests;
}

function countRequests(requests, pathname) {
  return requests.filter((request) => request.pathname === pathname).length;
}

beforeEach(() => {
  clearApiCache();
  clearDataStoreCache();
});

afterEach(() => {
  clearApiCache();
  clearDataStoreCache();
  globalThis.fetch = originalFetch;
  Date.now = originalDateNow;
});

describe('frontend data snapshots', () => {
  test('deduplicates concurrent loads, honours the TTL and supports forced refreshes', async () => {
    let now = 1_000;
    Date.now = () => now;
    const requests = installApiFetch();

    const [first, concurrent] = await Promise.all([getSearchCatalog(), getSearchCatalog()]);

    expect(concurrent).toBe(first);
    expect(countRequests(requests, '/api/organizers/options')).toBe(1);
    expect(countRequests(requests, '/api/competitions/structures')).toBe(1);
    expect(first.eventIndex.map((event) => event.id)).toEqual([11, 10]);
    expect(first.eventIndex[0]).toMatchObject({
      competitionId: 2,
      competitionName: 'Mock Competition',
      locationTypes: ['Country', 'City'],
      year: 2025,
    });
    expect(first.searchIndex.find((entry) => entry.type === 'Event' && entry.id === 11)).toMatchObject({
      href: '/events/11?year=2025',
      subtitle: 'Mock Competition · Sao Paulo',
    });

    expect(await getSearchCatalog()).toBe(first);
    expect(countRequests(requests, '/api/organizers/options')).toBe(1);

    now += 60_000;
    const afterTtl = await getSearchCatalog();
    expect(afterTtl).not.toBe(first);
    expect(countRequests(requests, '/api/organizers/options')).toBe(2);

    await getSearchCatalog({ force: true });
    expect(countRequests(requests, '/api/organizers/options')).toBe(3);
  });

  test('drops a rejected snapshot so the next call can retry', async () => {
    const requests = installApiFetch({ failOrganizerOnce: true });

    await expect(getSearchCatalog()).rejects.toThrow('temporary catalog failure');
    await expect(getSearchCatalog()).resolves.toMatchObject({ organizerOptions });
    expect(countRequests(requests, '/api/organizers/options')).toBe(2);
  });

  test('caches a complete universe and refreshes every collection on demand', async () => {
    const requests = installApiFetch();

    const first = await getUniverseSnapshot();
    const cached = await getUniverseSnapshot();

    expect(cached).toBe(first);
    expect(first).toMatchObject({
      organizerOptions,
      competitionOptions,
      institutionOptions,
      teamOptions,
      competitions: competitionStructures,
    });
    expect(countRequests(requests, '/api/organizers/structures')).toBe(1);
    expect(countRequests(requests, '/api/institutions/structures')).toBe(1);
    expect(countRequests(requests, '/api/teams/structures')).toBe(1);

    const refreshed = await getUniverseSnapshot({ force: true });
    expect(refreshed).not.toBe(first);
    expect(countRequests(requests, '/api/organizers/structures')).toBe(2);
    expect(countRequests(requests, '/api/institutions/structures')).toBe(2);
    expect(countRequests(requests, '/api/teams/structures')).toBe(2);
  });

  test('does not request structure endpoints when every option catalog is empty', async () => {
    const requests = installApiFetch({ empty: true });

    const snapshot = await getUniverseSnapshot();

    expect(snapshot.events).toEqual([]);
    expect(snapshot.searchIndex).toEqual([]);
    expect(snapshot.organizers).toEqual([]);
    expect(snapshot.competitions).toEqual([]);
    expect(snapshot.institutions).toEqual([]);
    expect(snapshot.teams).toEqual([]);
    expect(requests.some((request) => request.pathname.endsWith('/structures'))).toBe(false);
  });

  test('resolves option labels from numeric or string identifiers', () => {
    expect(getOptionLabel(teamOptions, '4')).toBe('Mock Team');
    expect(getOptionLabel(teamOptions, 999)).toBe('Unknown');
  });
});

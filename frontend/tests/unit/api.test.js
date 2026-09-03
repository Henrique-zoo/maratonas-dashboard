import { afterEach, describe, expect, test } from 'bun:test';

import {
  clearApiCache,
  getCompetitionOptions,
  getCompetitionLocationStats,
  getEventLocationStats,
  getEventStructure,
  getInstitutionEventPerformance,
  getOrganizerOptions,
  getTeamCompetitionYearStructure,
} from '../../app/src/lib/api.js';

const originalFetch = globalThis.fetch;

afterEach(() => {
  clearApiCache();
  globalThis.fetch = originalFetch;
});

function jsonResponse(payload, status = 200) {
  return new Response(JSON.stringify(payload), {
    status,
    headers: { 'Content-Type': 'application/json' },
  });
}

describe('API transport', () => {
  test('deduplicates only concurrent requests and does not retain stale responses', async () => {
    const calls = [];
    globalThis.fetch = async (url) => {
      calls.push(url);
      await Promise.resolve();
      return jsonResponse([]);
    };

    await Promise.all([getOrganizerOptions(), getOrganizerOptions()]);
    expect(calls).toEqual(['/api/organizers/options']);

    await getOrganizerOptions();
    expect(calls).toEqual(['/api/organizers/options', '/api/organizers/options']);
  });

  test('uses an identity-only event URL and leaves metadata to the backend', async () => {
    const calls = [];
    globalThis.fetch = async (url) => {
      calls.push(url);
      return jsonResponse({ id: 10, year: 2025 });
    };

    await getEventStructure(10, 2025);

    expect(calls).toEqual(['/api/events/10/structure?year=2025']);
  });

  test('uses explicit semantic subresources for analytical and annual queries', async () => {
    const calls = [];
    globalThis.fetch = async (url) => {
      calls.push(url);
      return jsonResponse([]);
    };

    await getCompetitionLocationStats(1, 'Country', 2025);
    await getEventLocationStats(10, 'City', 2025);
    await getInstitutionEventPerformance(200, 10, 2020, 2025);
    await getTeamCompetitionYearStructure(100, 1, 2025);

    expect(calls).toEqual([
      '/api/competitions/1/location-stats?location_type=Country&year=2025',
      '/api/events/10/location-stats?location_type=City&year=2025',
      '/api/institutions/200/events/10/performance?start_year=2020&end_year=2025',
      '/api/teams/100/competitions/1/structure?year=2025',
    ]);
  });

  test('does not cache failed requests', async () => {
    let attempts = 0;
    globalThis.fetch = async () => {
      attempts += 1;
      return attempts === 1 ? jsonResponse({ error: 'temporary' }, 503) : jsonResponse([]);
    };

    await expect(getOrganizerOptions()).rejects.toThrow('temporary');
    await expect(getOrganizerOptions()).resolves.toEqual([]);
    expect(attempts).toBe(2);
  });

  test('sends explicit JSON transport options and encodes query values', async () => {
    let capturedRequest;
    globalThis.fetch = async (url, options) => {
      capturedRequest = { url, options };
      return jsonResponse([]);
    };

    await getCompetitionLocationStats(7, 'State / Province', 2025);

    expect(capturedRequest).toEqual({
      url: '/api/competitions/7/location-stats?location_type=State+%2F+Province&year=2025',
      options: {
        cache: 'no-store',
        headers: { Accept: 'application/json' },
      },
    });
  });

  test('uses the HTTP status when an error response is not JSON', async () => {
    globalThis.fetch = async () =>
      new Response('upstream unavailable', {
        status: 502,
        headers: { 'Content-Type': 'text/plain' },
      });

    await expect(getCompetitionOptions()).rejects.toThrow('Request failed with status 502');
  });

  test('retries after a network rejection', async () => {
    let attempts = 0;
    globalThis.fetch = async () => {
      attempts += 1;

      if (attempts === 1) {
        throw new TypeError('network unavailable');
      }

      return jsonResponse([{ id: 1, name: 'Recovered' }]);
    };

    await expect(getOrganizerOptions()).rejects.toThrow('network unavailable');
    await expect(getOrganizerOptions()).resolves.toEqual([{ id: 1, name: 'Recovered' }]);
    expect(attempts).toBe(2);
  });

  test('does not let an older request remove a newer in-flight request after a cache clear', async () => {
    const resolvers = [];
    let calls = 0;
    globalThis.fetch = () => {
      calls += 1;
      return new Promise((resolve) => resolvers.push(resolve));
    };

    const first = getOrganizerOptions();
    clearApiCache();
    const second = getOrganizerOptions();

    resolvers[0](jsonResponse([{ id: 1, name: 'First' }]));
    await first;

    const third = getOrganizerOptions();
    const callsBeforeResolution = calls;

    resolvers[1](jsonResponse([{ id: 2, name: 'Second' }]));
    if (resolvers[2]) {
      resolvers[2](jsonResponse([{ id: 3, name: 'Unexpected duplicate' }]));
    }

    const [secondPayload, thirdPayload] = await Promise.all([second, third]);
    expect(callsBeforeResolution).toBe(2);
    expect(secondPayload).toEqual([{ id: 2, name: 'Second' }]);
    expect(thirdPayload).toEqual(secondPayload);
  });
});

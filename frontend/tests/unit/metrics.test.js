import { describe, expect, test } from 'bun:test';

import {
  flattenInstitutionEvents,
  getCompetitionOverview,
  getInstitutionOverview,
  getOrganizerOverview,
  getTeamOverview,
  groupBy,
  latestYear,
  snapshotYear,
  sortByDateDesc,
  uniqueCount,
} from '../../app/src/lib/metrics.js';

const team = (id, members, women = 0) => ({
  id,
  total_members: members,
  female_participants: women,
});

describe('snapshot semantics', () => {
  test('distinguishes unique teams from team and contestant entries', () => {
    const overview = getCompetitionOverview({
      years: [2024, 2025],
      snapshot_year: 2025,
      events: [{ teams: [team(1, 3, 1), team(2, 3)] }, { teams: [team(1, 3, 1)] }],
    });

    expect(overview.snapshotYear).toBe(2025);
    expect(overview.uniqueTeams).toBe(2);
    expect(overview.teamEntries).toBe(3);
    expect(overview.participantEntries).toBe(9);
    expect(overview.femaleParticipantEntries).toBe(2);
    expect(overview.totalParticipants).toBeUndefined();
    expect(overview.femaleParticipants).toBeUndefined();
  });

  test('keeps the reference seasons of portfolio snapshots explicit', () => {
    const competitions = [
      {
        snapshot_year: 2023,
        total_members: 3,
        female_participants: 1,
        events: [
          {
            total_teams: 1,
            total_participants: 3,
            female_participants: 1,
            teams: [team(1, 3, 1)],
          },
        ],
      },
      {
        snapshot_year: 2025,
        total_members: 6,
        female_participants: 2,
        events: [
          {
            total_teams: 2,
            total_participants: 6,
            female_participants: 2,
            teams: [team(2, 3), team(3, 3, 2)],
          },
        ],
      },
    ];

    expect(getTeamOverview({ competitions }).snapshotYears).toEqual([2023, 2025]);
    expect(getInstitutionOverview({ competitions }).snapshotYears).toEqual([2023, 2025]);
    expect(getOrganizerOverview({ competitions }).snapshotYears).toEqual([2023, 2025]);
  });

  test('uses explicit snapshot metadata before event and declared-year fallbacks', () => {
    expect(snapshotYear({ snapshot_year: '2025', events: [], years: [2024] })).toBe(2025);
    expect(
      snapshotYear({
        events: [{ date: 'invalid' }, { date: '2024-03-01' }],
        years: [2023],
      }),
    ).toBe(2024);
    expect(snapshotYear({ events: [], years: ['2022', '2023'] })).toBe(2023);
    expect(snapshotYear({ events: [], years: [] })).toBeNull();
    expect(latestYear([])).toBeNull();
  });

  test('returns stable zero-valued overviews for empty collections', () => {
    expect(getCompetitionOverview({ events: [], years: [] })).toEqual({
      eventCount: 0,
      teamEntries: 0,
      uniqueTeams: 0,
      participantEntries: 0,
      femaleParticipantEntries: 0,
      yearSpan: '—',
      latestYear: null,
      snapshotYear: null,
    });
    expect(getTeamOverview({ competitions: [] })).toMatchObject({
      competitionCount: 0,
      eventCount: 0,
      totalMembers: 0,
      snapshotYears: [],
    });
    expect(getInstitutionOverview({ competitions: [] })).toMatchObject({
      competitionCount: 0,
      eventCount: 0,
      teamEntries: 0,
      snapshotYears: [],
    });
    expect(getOrganizerOverview({ competitions: [] })).toMatchObject({
      competitionCount: 0,
      eventCount: 0,
      totalTeams: 0,
      locationTypes: [],
    });
  });

  test('flattens contextual event data without mutating the source', () => {
    const sourceEvent = { id: 10, name: 'Regional' };
    const flattened = flattenInstitutionEvents({
      competitions: [
        {
          id: 2,
          name: 'Mock Competition',
          website_url: 'https://example.test',
          events: [sourceEvent],
        },
      ],
    });

    expect(flattened).toEqual([
      {
        id: 10,
        name: 'Regional',
        competition_id: 2,
        competition_name: 'Mock Competition',
        competition_url: 'https://example.test',
      },
    ]);
    expect(sourceEvent).toEqual({ id: 10, name: 'Regional' });
  });

  test('counts unique values, sorts copies and groups records deterministically', () => {
    const records = [
      { id: 1, date: '2024-01-01', kind: 'regional' },
      { id: 1, date: '2025-01-01', kind: 'final' },
      { id: 2, date: '2023-01-01', kind: 'regional' },
    ];

    expect(uniqueCount(records, (record) => record.id)).toBe(2);
    expect(sortByDateDesc(records).map((record) => record.date)).toEqual([
      '2025-01-01',
      '2024-01-01',
      '2023-01-01',
    ]);
    expect(records[0].date).toBe('2024-01-01');
    expect(groupBy(records, (record) => record.kind).get('regional')).toHaveLength(2);
  });
});

import { formatYearSpan, sumBy } from './ui.js';

export function latestYear(years = []) {
  return years.length ? Math.max(...years.map((value) => Number(value))) : null;
}

export function uniqueCount(items = [], selector = (value) => value) {
  return new Set(items.map(selector)).size;
}

export function flattenCompetitionTeams(competition) {
  return competition.events.flatMap((event) => event.teams || []);
}

export function snapshotYear(structure) {
  if (Number(structure?.snapshot_year)) {
    return Number(structure.snapshot_year);
  }

  const eventYears = (structure?.events || [])
    .map((event) => new Date(`${event.date}T00:00:00`).getFullYear())
    .filter(Number.isInteger);

  return latestYear(eventYears) || latestYear(structure?.years || []);
}

export function snapshotYears(competitions = []) {
  return Array.from(new Set(competitions.map(snapshotYear).filter(Boolean))).sort(
    (left, right) => left - right,
  );
}

export function getCompetitionOverview(competition) {
  const teams = flattenCompetitionTeams(competition);

  return {
    eventCount: competition.events.length,
    teamEntries: teams.length,
    uniqueTeams: uniqueCount(teams, (team) => team.id),
    participantEntries: sumBy(teams, (team) => team.total_members),
    femaleParticipantEntries: sumBy(teams, (team) => team.female_participants),
    yearSpan: formatYearSpan(competition.years),
    latestYear: snapshotYear(competition),
    snapshotYear: snapshotYear(competition),
  };
}

export function getTeamOverview(team) {
  const events = team.competitions.flatMap((competition) => competition.events || []);

  return {
    competitionCount: team.competitions.length,
    eventCount: events.length,
    participantEntries: sumBy(team.competitions, (competition) => competition.total_members),
    femaleParticipantEntries: sumBy(team.competitions, (competition) => competition.female_participants),
    totalMembers: sumBy(team.competitions, (competition) => competition.total_members),
    snapshotYears: snapshotYears(team.competitions),
  };
}

export function flattenInstitutionEvents(institution) {
  return institution.competitions.flatMap((competition) =>
    competition.events.map((event) => ({
      ...event,
      competition_id: competition.id,
      competition_name: competition.name,
      competition_url: competition.website_url,
    })),
  );
}

export function getInstitutionOverview(institution) {
  const events = flattenInstitutionEvents(institution);
  const teams = events.flatMap((event) => event.teams || []);

  return {
    competitionCount: institution.competitions.length,
    eventCount: events.length,
    teamEntries: teams.length,
    participantEntries: sumBy(teams, (team) => team.total_members),
    femaleParticipantEntries: sumBy(teams, (team) => team.female_participants),
    snapshotYears: snapshotYears(institution.competitions),
  };
}

export function flattenOrganizerEvents(organizer) {
  return organizer.competitions.flatMap((competition) =>
    competition.events.map((event) => ({
      ...event,
      competition_id: competition.id,
      competition_name: competition.name,
      competition_years: competition.years,
      competition_location_types: competition.location_types,
    })),
  );
}

export function getOrganizerOverview(organizer) {
  const events = flattenOrganizerEvents(organizer);

  return {
    competitionCount: organizer.competitions.length,
    eventCount: events.length,
    teamEntries: sumBy(events, (event) => event.total_teams),
    totalTeams: sumBy(events, (event) => event.total_teams),
    participantEntries: sumBy(events, (event) => event.total_participants),
    femaleParticipantEntries: sumBy(events, (event) => event.female_participants),
    snapshotYears: snapshotYears(organizer.competitions),
    locationTypes: Array.from(
      new Set(
        organizer.competitions.flatMap((competition) =>
          (competition.location_types || []).map((locationType) => String(locationType)),
        ),
      ),
    ),
  };
}

export function sortByDateDesc(items = [], selector = (item) => item.date) {
  return [...items].sort((left, right) => new Date(selector(right)) - new Date(selector(left)));
}

export function groupBy(items = [], selector = (value) => value) {
  return items.reduce((accumulator, item) => {
    const key = selector(item);

    if (!accumulator.has(key)) {
      accumulator.set(key, []);
    }

    accumulator.get(key).push(item);
    return accumulator;
  }, new Map());
}

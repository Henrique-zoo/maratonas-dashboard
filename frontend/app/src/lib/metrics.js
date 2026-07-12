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

export function getCompetitionOverview(competition) {
  const teams = flattenCompetitionTeams(competition);

  return {
    eventCount: competition.events.length,
    teamEntries: teams.length,
    uniqueTeams: uniqueCount(teams, (team) => team.id),
    totalParticipants: sumBy(teams, (team) => team.total_members),
    femaleParticipants: sumBy(teams, (team) => team.female_participants),
    yearSpan: formatYearSpan(competition.years),
    latestYear: latestYear(competition.years),
  };
}

export function getTeamOverview(team) {
  const events = team.competitions.flatMap((competition) => competition.events || []);

  return {
    competitionCount: team.competitions.length,
    eventCount: events.length,
    totalMembers: sumBy(team.competitions, (competition) => competition.total_members),
    femaleParticipants: sumBy(team.competitions, (competition) => competition.female_participants),
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
    totalParticipants: sumBy(teams, (team) => team.total_members),
    femaleParticipants: sumBy(teams, (team) => team.female_participants),
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
    totalTeams: sumBy(events, (event) => event.total_teams),
    totalParticipants: sumBy(events, (event) => event.total_participants),
    femaleParticipants: sumBy(events, (event) => event.female_participants),
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

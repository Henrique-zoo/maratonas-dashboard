import {
  getCompetitionOptions,
  getCompetitionStructures,
  getInstitutionOptions,
  getInstitutionStructures,
  getOrganizerOptions,
  getOrganizerStructures,
  getTeamOptions,
  getTeamStructures,
} from './api.js';

let searchCatalogPromise;
let universePromise;

function buildEventIndex(competitions) {
  const deduped = new Map();

  competitions.forEach((competition) => {
    competition.events.forEach((event) => {
      deduped.set(event.id, {
        id: event.id,
        name: event.name,
        date: event.date,
        location: event.location,
        locationTypes: event.location_types,
        competitionId: competition.id,
        competitionName: competition.name,
        year: new Date(`${event.date}T00:00:00`).getFullYear(),
      });
    });
  });

  return Array.from(deduped.values()).sort((left, right) => {
    return new Date(right.date) - new Date(left.date);
  });
}

function buildSearchIndex({
  organizerOptions,
  competitionOptions,
  institutionOptions,
  teamOptions,
  eventIndex,
}) {
  return [
    ...organizerOptions.map((item) => ({
      type: 'Organizer',
      id: item.id,
      name: item.name,
      subtitle: 'Competition network and calendar owner',
      href: `/organizers/${item.id}`,
      searchText: `${item.name} organizer`.toLowerCase(),
    })),
    ...competitionOptions.map((item) => ({
      type: 'Competition',
      id: item.id,
      name: item.name,
      subtitle: 'Season overview, rankings and geo stats',
      href: `/competitions/${item.id}`,
      searchText: `${item.name} competition`.toLowerCase(),
    })),
    ...institutionOptions.map((item) => ({
      type: 'Institution',
      id: item.id,
      name: item.name,
      subtitle: 'Programs, lineups and historical event performance',
      href: `/institutions/${item.id}`,
      searchText: `${item.name} institution university`.toLowerCase(),
    })),
    ...teamOptions.map((item) => ({
      type: 'Team',
      id: item.id,
      name: item.name,
      subtitle: 'Competition portfolio and event trail',
      href: `/teams/${item.id}`,
      searchText: `${item.name} team`.toLowerCase(),
    })),
    ...eventIndex.map((item) => ({
      type: 'Event',
      id: item.id,
      name: item.name,
      subtitle: `${item.competitionName} · ${item.location}`,
      href: `/events/${item.id}?year=${item.year}&name=${encodeURIComponent(item.name)}&date=${item.date}&location=${encodeURIComponent(item.location)}&locationTypes=${item.locationTypes.join(',')}&competitionId=${item.competitionId}&competitionName=${encodeURIComponent(item.competitionName)}`,
      searchText: `${item.name} ${item.competitionName} ${item.location}`.toLowerCase(),
    })),
  ];
}

export async function getSearchCatalog() {
  if (!searchCatalogPromise) {
    searchCatalogPromise = (async () => {
      const [organizerOptions, competitionOptions, institutionOptions, teamOptions] = await Promise.all([
        getOrganizerOptions(),
        getCompetitionOptions(),
        getInstitutionOptions(),
        getTeamOptions(),
      ]);

      const competitionStructures = competitionOptions.length
        ? await getCompetitionStructures(competitionOptions.map((item) => item.id))
        : [];

      const eventIndex = buildEventIndex(competitionStructures);

      return {
        organizerOptions,
        competitionOptions,
        institutionOptions,
        teamOptions,
        competitionStructures,
        eventIndex,
        searchIndex: buildSearchIndex({
          organizerOptions,
          competitionOptions,
          institutionOptions,
          teamOptions,
          eventIndex,
        }),
      };
    })();
  }

  return searchCatalogPromise;
}

export async function getUniverseSnapshot() {
  if (!universePromise) {
    universePromise = (async () => {
      const catalog = await getSearchCatalog();

      const [organizers, institutions, teams] = await Promise.all([
        catalog.organizerOptions.length
          ? getOrganizerStructures(catalog.organizerOptions.map((item) => item.id))
          : [],
        catalog.institutionOptions.length
          ? getInstitutionStructures(catalog.institutionOptions.map((item) => item.id))
          : [],
        catalog.teamOptions.length ? getTeamStructures(catalog.teamOptions.map((item) => item.id)) : [],
      ]);

      return {
        organizerOptions: catalog.organizerOptions,
        competitionOptions: catalog.competitionOptions,
        institutionOptions: catalog.institutionOptions,
        teamOptions: catalog.teamOptions,
        searchIndex: catalog.searchIndex,
        events: catalog.eventIndex,
        organizers,
        competitions: catalog.competitionStructures,
        institutions,
        teams,
      };
    })();
  }

  return universePromise;
}

export async function getEventMetadata(eventId) {
  const universe = await getUniverseSnapshot();
  return universe.events.find((item) => item.id === Number(eventId)) || null;
}

export function getOptionLabel(options, id) {
  return options.find((item) => item.id === Number(id))?.name || 'Unknown';
}

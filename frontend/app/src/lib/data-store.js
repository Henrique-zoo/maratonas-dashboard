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
let searchCatalogExpiresAt = 0;
let universeExpiresAt = 0;

const SNAPSHOT_TTL_MS = 60_000;

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
      href: `/events/${item.id}?year=${item.year}`,
      searchText: `${item.name} ${item.competitionName} ${item.location}`.toLowerCase(),
    })),
  ];
}

export async function getSearchCatalog({ force = false } = {}) {
  if (force || Date.now() >= searchCatalogExpiresAt) {
    searchCatalogPromise = undefined;
  }

  if (!searchCatalogPromise) {
    searchCatalogExpiresAt = Date.now() + SNAPSHOT_TTL_MS;
    let guardedCatalogPromise;
    const pendingCatalogPromise = (async () => {
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

    guardedCatalogPromise = pendingCatalogPromise.catch((error) => {
      if (searchCatalogPromise === guardedCatalogPromise) {
        searchCatalogPromise = undefined;
        searchCatalogExpiresAt = 0;
      }

      throw error;
    });
    searchCatalogPromise = guardedCatalogPromise;
  }

  return searchCatalogPromise;
}

export async function getUniverseSnapshot({ force = false } = {}) {
  if (force || Date.now() >= universeExpiresAt) {
    universePromise = undefined;
  }

  if (!universePromise) {
    universeExpiresAt = Date.now() + SNAPSHOT_TTL_MS;
    let guardedUniversePromise;
    const pendingUniversePromise = (async () => {
      const catalog = await getSearchCatalog({ force });

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

    guardedUniversePromise = pendingUniversePromise.catch((error) => {
      if (universePromise === guardedUniversePromise) {
        universePromise = undefined;
        universeExpiresAt = 0;
      }

      throw error;
    });
    universePromise = guardedUniversePromise;
  }

  return universePromise;
}

export function clearDataStoreCache() {
  searchCatalogPromise = undefined;
  universePromise = undefined;
  searchCatalogExpiresAt = 0;
  universeExpiresAt = 0;
}

export function getOptionLabel(options, id) {
  return options.find((item) => item.id === Number(id))?.name || 'Unknown';
}

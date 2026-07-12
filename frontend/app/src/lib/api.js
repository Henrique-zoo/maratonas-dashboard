const API_BASE = '/api';
const requestCache = new Map();

function buildQuery(params = {}) {
  const searchParams = new URLSearchParams();

  Object.entries(params).forEach(([key, value]) => {
    if (value === undefined || value === null) {
      return;
    }

    searchParams.set(key, String(value));
  });

  const query = searchParams.toString();
  return query ? `?${query}` : '';
}

async function request(path, params = {}) {
  const url = `${API_BASE}${path}${buildQuery(params)}`;

  if (!requestCache.has(url)) {
    requestCache.set(
      url,
      fetch(url).then(async (response) => {
        const payload = await response.json().catch(() => ({}));

        if (!response.ok) {
          throw new Error(payload.error || `Request failed with status ${response.status}`);
        }

        return payload;
      }),
    );
  }

  return requestCache.get(url);
}

function csv(values) {
  return Array.isArray(values) && values.length ? values.join(',') : '';
}

export function clearApiCache() {
  requestCache.clear();
}

export function getOrganizerOptions() {
  return request('/organizers/options');
}

export function getOrganizerStructures(ids) {
  return request('/organizers/structures', { organizer_ids: csv(ids) });
}

export function getOrganizerCompetitionYearStructure(id, year) {
  return request(`/organizers/competitions/${id}/structure`, { year });
}

export function getCompetitionOptions(organizerIds = null) {
  return request('/competitions/options', { organizer_ids: csv(organizerIds) });
}

export function getCompetitionStructures(ids) {
  return request('/competitions/structures', { competition_ids: csv(ids) });
}

export function getCompetitionYearStructure(id, year) {
  return request(`/competitions/${id}/structure`, { year });
}

export function getCompetitionStats(id, year) {
  return request(`/competitions/${id}/stats`, { year });
}

export function getCompetitionLocationStats(id, locationType, year) {
  return request(`/competitions/${id}/location_stats`, {
    location_type: locationType,
    year,
  });
}

export function getInstitutionOptions(competitionIds = null) {
  return request('/institutions/options', { competition_ids: csv(competitionIds) });
}

export function getInstitutionStructures(ids) {
  return request('/institutions/structures', { institution_ids: csv(ids) });
}

export function getInstitutionEventPerformance(institutionId, eventId, startYear, endYear) {
  return request(`/institutions/${institutionId}/events/${eventId}`, {
    start_year: startYear,
    end_year: endYear,
  });
}

export function getTeamOptions(competitionIds = null, institutionIds = null) {
  return request('/teams/options', {
    competition_ids: csv(competitionIds),
    institution_ids: csv(institutionIds),
  });
}

export function getTeamStructures(ids) {
  return request('/teams/structures', { team_ids: csv(ids) });
}

export function getTeamCompetitionYearStructure(teamId, competitionId, year) {
  return request(`/teams/${teamId}/competitions/${competitionId}`, { year });
}

export function getEventStats(id, year) {
  return request(`/events/${id}/stats`, { year });
}

export function getEventLocationStats(id, locationType, year) {
  return request(`/events/${id}/location_stats`, {
    location_type: locationType,
    year,
  });
}

import {
  getCompetitionLocationStats,
  getCompetitionOptions,
  getCompetitionStats,
  getCompetitionYearStructure,
  getEventLocationStats,
  getEventStats,
  getInstitutionOptions,
  getOrganizerCompetitionYearStructure,
  getTeamOptions,
} from '../lib/api.js';
import { initCustomDropdowns, renderDropdown, setDropdownOptions } from '../lib/custom-select.js';
import { getUniverseSnapshot } from '../lib/data-store.js';
import {
  chooseLocationType,
  escapeHtml,
  formatDate,
  formatNumber,
  renderBarList,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
  renderStatGrid,
  renderTable,
  serialiseQuery,
} from '../lib/ui.js';
import {
  getCompetitionOverview,
  getInstitutionOverview,
  getOrganizerOverview,
  getTeamOverview,
  latestYear,
  sortByDateDesc,
} from '../lib/metrics.js';

let dropdownAbortController = null;

function renderCompetitionSpotlight(competition) {
  const overview = getCompetitionOverview(competition);
  const latestEvent = sortByDateDesc(competition.events)[0];

  return `
    <article class="spotlight-card">
      <div class="spotlight-card__header">
        <div>
          ${renderChip(competition.gender_category, 'amber')}
          <h3><a href="/competitions/${competition.id}" data-link>${escapeHtml(competition.name)}</a></h3>
          <p>Snapshot ${overview.snapshotYear} · ${escapeHtml(String(overview.eventCount))} events</p>
        </div>
        <a class="button button--ghost" href="/competitions/${competition.id}" data-link>Open competition</a>
      </div>
      ${renderMetricStrip([
        { label: 'Unique teams', value: formatNumber(overview.uniqueTeams) },
        { label: 'Entries', value: formatNumber(overview.teamEntries) },
        {
          label: 'Contestant entries',
          value: formatNumber(overview.participantEntries),
        },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
        },
      ])}
      ${latestEvent ? `<p class="card-note">Latest event in this snapshot: <a href="/events/${latestEvent.id}${serialiseQuery({ year: overview.snapshotYear })}" data-link>${escapeHtml(latestEvent.name)}</a> on ${escapeHtml(formatDate(latestEvent.date))}</p>` : ''}
    </article>
  `;
}

function queryId(value) {
  const id = Number(value);
  return Number.isInteger(id) && id > 0 ? id : null;
}

function queryIds(value) {
  if (!value) {
    return [];
  }

  return Array.from(
    new Set(
      String(value)
        .split(',')
        .map((item) => Number(item))
        .filter((item) => Number.isInteger(item) && item > 0),
    ),
  );
}

function selectedOptionIds(options, requestedIds) {
  const ids = optionIdSet(options);
  return requestedIds.filter((id) => ids.has(id));
}

function itemById(items, id) {
  return id ? items.find((item) => item.id === id) || null : null;
}

function itemsByIds(items, ids) {
  const idSet = new Set(ids);
  return items.filter((item) => idSet.has(item.id));
}

function optionLabel(options, id) {
  return itemById(options, id)?.name || null;
}

function optionLabels(options, ids) {
  return ids.map((id) => optionLabel(options, id)).filter(Boolean);
}

function optionIdSet(options) {
  return new Set(options.map((option) => option.id));
}

function teamOptionsForDisplay(options, selectedInstitutionIds) {
  if (selectedInstitutionIds.length === 1) {
    return options;
  }

  return options.map((option) => {
    const institutionLabel = option.institution_short_name || option.institution_name;

    return institutionLabel ? { ...option, name: `${option.name} (${institutionLabel})` } : option;
  });
}

function queryListValue(ids = []) {
  return ids.length ? ids.join(',') : '';
}

function selectionChipLabel(options, ids, singular, plural) {
  const names = optionLabels(options, ids).join(', ');
  return ids.length === 1 ? `${singular}: ${names}` : `${plural}: ${names}`;
}

function selectionSummaryLabel(labels, suffix = '') {
  return `Showing ${labels.join(', ')}${suffix}.`;
}

function teamMatchesInstitutions(team, institutions = []) {
  if (!institutions.length) {
    return true;
  }

  const institutionIds = new Set(institutions.map((institution) => institution.id));
  return institutionIds.has(team.institution_id);
}

function filterEventTeams(event, institutions = [], teamIds = []) {
  if (!institutions.length && !teamIds.length) {
    return event;
  }

  const teamIdSet = new Set(teamIds);

  return {
    ...event,
    teams: (event.teams || []).filter((team) => {
      return teamMatchesInstitutions(team, institutions) && (!teamIdSet.size || teamIdSet.has(team.id));
    }),
  };
}

function filterCompetitionEntries(competition, institutions = [], teamIds = []) {
  if (!institutions.length && !teamIds.length) {
    return competition;
  }

  return {
    ...competition,
    events: (competition.events || [])
      .map((event) => filterEventTeams(event, institutions, teamIds))
      .filter((event) => event.teams.length),
  };
}

function competitionIdsForStructure(structure) {
  return new Set((structure?.competitions || []).map((competition) => competition.id));
}

function filterInstitutionEntries(institution, competitionIds, teamIds = []) {
  const teamIdSet = new Set(teamIds);

  return {
    ...institution,
    competitions: (institution.competitions || [])
      .filter((competition) => !competitionIds || competitionIds.has(competition.id))
      .map((competition) => ({
        ...competition,
        events: (competition.events || [])
          .map((event) => ({
            ...event,
            teams: (event.teams || []).filter((team) => !teamIdSet.size || teamIdSet.has(team.id)),
          }))
          .filter((event) => !teamIdSet.size || event.teams.length),
      }))
      .filter((competition) => competition.events.length),
  };
}

function filterOrganizerEntries(organizer, competitionIds) {
  return {
    ...organizer,
    competitions: (organizer.competitions || []).filter((competition) => {
      return !competitionIds || competitionIds.has(competition.id);
    }),
  };
}

function competitionEvents(competitions) {
  return sortByDateDesc(
    competitions.flatMap((competition) =>
      (competition.events || []).map((event) => ({
        id: event.id,
        name: event.name,
        date: event.date,
        location: event.location,
        locationTypes: event.location_types || event.locationTypes || [],
        competitionId: competition.id,
        competitionName: competition.name,
        year: new Date(`${event.date}T00:00:00`).getFullYear(),
      })),
    ),
    (event) => event.date,
  );
}

function uniqueTeamCount(competitions) {
  return new Set(
    competitions.flatMap((competition) =>
      (competition.events || []).flatMap((event) => (event.teams || []).map((team) => team.id)),
    ),
  ).size;
}

function structureYearsLabel(years = []) {
  return years.length ? [...years].sort((left, right) => left - right).join(', ') : '-';
}

function eventYear(event) {
  return event?.date ? new Date(`${event.date}T00:00:00`).getFullYear() : null;
}

function competitionYears(competition) {
  if (competition?.years?.length) {
    return competition.years;
  }

  return Array.from(new Set((competition?.events || []).map(eventYear).filter(Boolean)));
}

function normalizeYears(years = []) {
  return Array.from(
    new Set(years.map((year) => Number(year)).filter((year) => Number.isInteger(year) && year > 0)),
  ).sort((left, right) => right - left);
}

function selectedCompetitionYear(years = [], requestedYear = null) {
  const availableYears = normalizeYears(years);
  const normalizedRequestedYear = Number(requestedYear);

  if (availableYears.includes(normalizedRequestedYear)) {
    return normalizedRequestedYear;
  }

  return latestYear(availableYears);
}

function normalizeLocationTypes(types = []) {
  return Array.from(new Set(types.filter(Boolean).map((type) => String(type))));
}

function selectedLocationType(locationTypes = [], requestedLocationType = null) {
  const normalizedTypes = normalizeLocationTypes(locationTypes);

  if (requestedLocationType && normalizedTypes.includes(requestedLocationType)) {
    return requestedLocationType;
  }

  return chooseLocationType(normalizedTypes);
}

function eventLocationTypes(event) {
  return event?.location_types || event?.locationTypes || [];
}

function competitionLocationTypes(competition) {
  return normalizeLocationTypes(
    competition?.location_types?.length
      ? competition.location_types
      : (competition?.events || []).flatMap(eventLocationTypes),
  );
}

function eventMatchesLocationType(event, locationType) {
  if (!locationType) {
    return true;
  }

  const locationTypes = eventLocationTypes(event);
  return locationTypes.length ? locationTypes.includes(locationType) : true;
}

function eventTeams(event) {
  return event?.teams || [];
}

function rankedTeams(event) {
  return [...eventTeams(event)].sort((left, right) => Number(left.rank || 0) - Number(right.rank || 0));
}

function eventParticipants(event) {
  return Number(
    event?.total_participants ??
      eventTeams(event).reduce((total, team) => total + Number(team.total_members || 0), 0),
  );
}

function eventFemaleParticipants(event) {
  return Number(
    event?.female_participants ??
      eventTeams(event).reduce((total, team) => total + Number(team.female_participants || 0), 0),
  );
}

function renderStructureShell({ eyebrow, title, actionHref, actionLabel, body }) {
  return `
    <section class="section-block" data-structure-result>
      <div class="section-head">
        <div>
          <span class="eyebrow">${escapeHtml(eyebrow)}</span>
          <h2>${escapeHtml(title)}</h2>
        </div>
        ${actionHref ? `<a class="button button--ghost" href="${escapeHtml(actionHref)}" data-link>${escapeHtml(actionLabel)}</a>` : ''}
      </div>
      ${body}
    </section>
  `;
}

function homeIndexHref(params = {}) {
  return `/${serialiseQuery(params)}`;
}

function findStructureEvent(competition, eventId) {
  return (competition?.events || []).find((event) => event.id === eventId) || null;
}

function replaceStructureCompetitionSeason(organizer, competitionId, seasonStructure) {
  if (!organizer || !seasonStructure) {
    return organizer;
  }

  return {
    ...organizer,
    competitions: (organizer.competitions || []).map((competition) =>
      competition.id === competitionId
        ? {
            ...competition,
            events: seasonStructure.events || [],
            location_types: seasonStructure.location_types || competition.location_types || [],
          }
        : competition,
    ),
  };
}

function eventTotals(events = []) {
  return events.reduce(
    (total, event) => ({
      teams: total.teams + Number(event.total_teams || eventTeams(event).length || 0),
      participantEntries: total.participantEntries + eventParticipants(event),
      femaleParticipantEntries: total.femaleParticipantEntries + eventFemaleParticipants(event),
    }),
    { teams: 0, participantEntries: 0, femaleParticipantEntries: 0 },
  );
}

function renderHomeOrganizerEventList({ organizer, competition, events, displayYear, selectedEventId }) {
  if (!events.length) {
    return '<p class="card-note">No events this year.</p>';
  }

  return `
    <div class="organizer-event-list">
      ${events
        .map((event) => {
          const locationTypes = normalizeLocationTypes(
            eventLocationTypes(event).length ? eventLocationTypes(event) : competition.location_types || [],
          );

          return `
            <a
              class="organizer-event-button ${event.id === selectedEventId ? 'is-selected' : ''}"
              href="${homeIndexHref({
                organizer: organizer.id,
                organizerCompetition: competition.id,
                organizerEvent: event.id,
                organizerYear: displayYear || eventYear(event),
                organizerLocationType: selectedLocationType(locationTypes, null),
              })}"
              data-home-organizer-event
              data-competition-id="${competition.id}"
              data-event-id="${event.id}"
            >
              <span>${escapeHtml(formatDate(event.date))}</span>
              <strong>${escapeHtml(event.name)}</strong>
              <small>${formatNumber(event.total_teams || eventTeams(event).length)} unique teams · ${escapeHtml(event.location || '-')}</small>
            </a>
          `;
        })
        .join('')}
    </div>
  `;
}

function renderHomeOrganizerCompetitionCard({
  organizer,
  competition,
  displayYear,
  selectedCompetitionId,
  selectedEventId,
}) {
  const selected = competition.id === selectedCompetitionId;
  const events = competition.events || [];
  const totals = eventTotals(events);
  const locationTypes = competitionLocationTypes({ ...competition, events });
  const selectHref = homeIndexHref({
    organizer: organizer.id,
    organizerCompetition: competition.id,
    organizerYear: displayYear,
    organizerLocationType: selectedLocationType(locationTypes, null),
  });

  return `
    <article
      class="organizer-competition-card ${selected ? 'is-selected' : ''}"
      data-home-organizer-card
      data-select-href="${escapeHtml(selectHref)}"
      data-organizer-id="${organizer.id}"
      data-competition-id="${competition.id}"
      data-display-year="${displayYear || ''}"
      role="button"
      tabindex="0"
    >
      <div class="organizer-competition-card__head">
        <div class="organizer-competition-card__identity">
          <div class="organizer-competition-card__title-row">
            <strong>${escapeHtml(competition.name)}</strong>
            ${
              competition.website_url
                ? `<a class="external-link-icon" href="${escapeHtml(competition.website_url)}" target="_blank" rel="noreferrer" aria-label="${escapeHtml(`Open ${competition.name} website`)}"></a>`
                : ''
            }
          </div>
        </div>
        <form class="organizer-competition-year-form" data-home-organizer-year-form>
          <input type="hidden" name="organizer" value="${organizer.id}" />
          <input type="hidden" name="organizerCompetition" value="${competition.id}" />
          ${renderDropdown({
            name: 'organizerYear',
            label: 'Year',
            options: normalizeYears(competitionYears(competition)).map((year) => ({
              id: String(year),
              name: String(year),
            })),
            selectedValue: displayYear,
            placeholder: 'Year',
            disabled: !competitionYears(competition).length,
            hideLabel: true,
          })}
        </form>
      </div>

      ${renderMetricStrip([
        { label: 'Events', value: formatNumber(events.length) },
        { label: 'Team entries', value: formatNumber(totals.teams) },
        {
          label: 'Contestant entries',
          value: formatNumber(totals.participantEntries),
        },
      ])}

      ${renderHomeOrganizerEventList({
        organizer,
        competition,
        events,
        displayYear,
        selectedEventId: selected ? selectedEventId : null,
      })}
    </article>
  `;
}

function renderHomeOrganizerInspector({
  target,
  stats,
  locationStats,
  locationChoices,
  selectedLocationType: selectedTier,
  organizerId,
}) {
  if (!target) {
    return '';
  }

  return `
    <aside class="organizer-inspector">
      <details class="organizer-inspector__drawer" open>
        <summary>
          <span>${escapeHtml(target.kind)} data</span>
          <strong>${escapeHtml(selectedTier || 'Annual totals')}</strong>
        </summary>

        <div class="section-head">
          <div>
            <span class="eyebrow">${escapeHtml(target.kind)} details</span>
            <h2>${escapeHtml(target.name)}</h2>
            <p>${escapeHtml(`Year ${target.year}`)}</p>
          </div>
        </div>

        <form class="organizer-inspector__form" id="home-organizer-location-filter">
          <input type="hidden" name="organizer" value="${organizerId}" />
          <input type="hidden" name="organizerCompetition" value="${target.competitionId}" />
          ${target.eventId ? `<input type="hidden" name="organizerEvent" value="${target.eventId}" />` : ''}
          <input type="hidden" name="organizerYear" value="${target.year}" />
          ${renderDropdown({
            name: 'organizerLocationType',
            label: 'Location level',
            options: locationChoices.map((locationType) => ({
              id: locationType,
              name: locationType,
            })),
            selectedValue: selectedTier,
            placeholder: 'Select location',
            disabled: !locationChoices.length,
          })}
          <button class="button" type="submit">Apply</button>
        </form>

        ${renderMetricStrip([
          {
            label: 'Institutions',
            value: formatNumber(stats.total_institutions),
            hint: 'Distinct',
          },
          {
            label: 'Teams',
            value: formatNumber(stats.total_teams),
            hint: 'Distinct',
          },
          {
            label: 'Participants',
            value: formatNumber(stats.total_participants),
            hint: 'Distinct people',
          },
          {
            label: 'Female participants',
            value: formatNumber(stats.female_participants),
            hint: 'Distinct people',
          },
        ])}

        ${
          selectedTier
            ? `
              <div class="organizer-inspector__split">
                <span class="eyebrow">${escapeHtml(selectedTier)} split</span>
                ${renderBarList(
                  locationStats.map((item) => ({
                    label: item.name,
                    value: item.total_teams,
                    subtitle: `${formatNumber(item.total_participants)} participants`,
                  })),
                  { valueFormatter: formatNumber },
                )}
              </div>
            `
            : '<p class="card-note">Choose a location level to filter.</p>'
        }
      </details>
    </aside>
  `;
}

function renderOrganizerStructureResult(
  organizer,
  {
    selectedCompetitionId = null,
    selectedEventId = null,
    selectedCompetitionYear = null,
    inspector = null,
  } = {},
) {
  if (!organizer) {
    return renderEmptyState('Organizer not found', 'Selected organizer is unavailable.');
  }

  const safeOrganizer = {
    ...organizer,
    competitions: organizer.competitions || [],
  };
  const overview = getOrganizerOverview(safeOrganizer);

  return renderStructureShell({
    eyebrow: 'Organizer',
    title: safeOrganizer.name,
    actionHref: `/organizers/${safeOrganizer.id}`,
    actionLabel: 'Open organizer',
    body: `
      ${renderStatGrid([
        {
          label: 'Competitions',
          value: formatNumber(overview.competitionCount),
          hint: 'Available',
        },
        {
          label: 'Snapshot events',
          value: formatNumber(overview.eventCount),
          hint: 'Latest season each',
        },
        {
          label: 'Contestant entries',
          value: formatNumber(overview.participantEntries),
          hint: `Reference seasons: ${overview.snapshotYears.join(', ')}`,
        },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
        },
      ])}

      <section class="organizer-index-layout ${inspector?.target ? 'has-inspector' : ''}">
        <div class="organizer-index-layout__main">
          <article class="organizer-structure-card">
            <div class="list-card__title">
              <div>
                <h3>${escapeHtml(safeOrganizer.name)}</h3>
                <p>${escapeHtml(String(overview.competitionCount))} competitions · ${escapeHtml(String(overview.eventCount))} events</p>
              </div>
            </div>

            <div class="organizer-competition-scroll" aria-label="${escapeHtml(`${safeOrganizer.name} competitions`)}">
              ${
                safeOrganizer.competitions.length
                  ? safeOrganizer.competitions
                      .map((competition) =>
                        renderHomeOrganizerCompetitionCard({
                          organizer: safeOrganizer,
                          competition,
                          displayYear:
                            competition.id === selectedCompetitionId
                              ? selectedCompetitionYear
                              : latestYear(competitionYears(competition)),
                          selectedCompetitionId,
                          selectedEventId,
                        }),
                      )
                      .join('')
                  : renderEmptyState('No competitions', 'This organizer has no competitions attached.')
              }
            </div>
          </article>
        </div>
        ${renderHomeOrganizerInspector({
          ...(inspector || {
            target: null,
            stats: null,
            locationStats: [],
            locationChoices: [],
            selectedLocationType: null,
          }),
          organizerId: safeOrganizer.id,
        })}
      </section>
    `,
  });
}

function eventDetailId(event) {
  return `competition-event-ranking-${event.id}`;
}

function eventDetailHref(event, selectedYear) {
  return `/events/${event.id}${serialiseQuery({
    year: selectedYear || eventYear(event),
  })}`;
}

function renderEventRankingPanel(event, competition, selectedYear) {
  const teams = rankedTeams(event);

  return `
    <div class="event-ranking-panel">
      <div class="event-ranking-panel__header">
        <div>
          <span class="eyebrow">Event ranking</span>
          <h3>${escapeHtml(event.name)}</h3>
        </div>
        <a class="button button--ghost" href="${eventDetailHref(event, selectedYear)}" data-link>Open event</a>
      </div>
      <p class="card-note">${escapeHtml(event.location || '-')} · ${formatNumber(teams.length)} ranked teams</p>
      ${
        teams.length
          ? renderTable({
              compact: true,
              columns: [
                { label: 'Rank' },
                { label: 'Team' },
                { label: 'Institution' },
                { label: 'Contestant entries', align: 'right' },
                { label: 'Female entries', align: 'right' },
              ],
              rows: teams.map((team) => [
                { value: `<strong>#${formatNumber(team.rank)}</strong>` },
                {
                  value: `<a href="/teams/${team.id}${serialiseQuery({
                    competition: competition.id,
                    year: selectedYear || eventYear(event),
                  })}" data-link>${escapeHtml(team.name)}</a>`,
                },
                {
                  value: `<span>${escapeHtml(team.institution_short_name || team.institution_name || '-')}</span>`,
                },
                { value: formatNumber(team.total_members), align: 'right' },
                {
                  value: formatNumber(team.female_participants),
                  align: 'right',
                },
              ]),
            })
          : renderEmptyState('No ranking rows', 'This event has no team rows in the selected year.')
      }
    </div>
  `;
}

function renderCompetitionEventsTable(events, competition, selectedYear) {
  return `
    <div class="table-wrap">
      <table class="data-table competition-events-table" data-event-rankings-table>
        <thead>
          <tr>
            <th>Event</th>
            <th>Date</th>
            <th>Location</th>
            <th class="is-right">Teams</th>
            <th class="is-right">Participants</th>
            <th class="is-right">Female participants</th>
          </tr>
        </thead>
        <tbody>
          ${events
            .map(
              (event) => `
                <tr class="competition-event-row" data-event-row data-event-id="${event.id}">
                  <td data-label="Event">
                    <button
                      class="event-row-toggle"
                      type="button"
                      aria-expanded="false"
                      aria-controls="${eventDetailId(event)}"
                      data-event-toggle
                    >
                      <span class="event-row-toggle__icon" aria-hidden="true"></span>
                      <span>
                        <strong>${escapeHtml(event.name)}</strong>
                        <small>Team ranking</small>
                      </span>
                    </button>
                  </td>
                  <td data-label="Date">${escapeHtml(formatDate(event.date))}</td>
                  <td data-label="Location">${escapeHtml(event.location || '-')}</td>
                  <td class="is-right" data-label="Teams">${formatNumber(eventTeams(event).length)}</td>
                  <td class="is-right" data-label="Participants">${formatNumber(eventParticipants(event))}</td>
                  <td class="is-right" data-label="Female participants">${formatNumber(eventFemaleParticipants(event))}</td>
                </tr>
                <tr class="competition-event-ranking-row" id="${eventDetailId(event)}" data-event-ranking-row hidden>
                  <td colspan="6">
                    ${renderEventRankingPanel(event, competition, selectedYear)}
                  </td>
                </tr>
              `,
            )
            .join('')}
        </tbody>
      </table>
    </div>
  `;
}

function renderCompetitionStructureResult(
  competition,
  { selectedYear = null, selectedLocationType = null, locationStats = [] } = {},
) {
  if (!competition) {
    return renderEmptyState('Competition not found', 'Selected competition is unavailable.');
  }

  const locationTypes = competitionLocationTypes(competition);
  const visibleEvents = sortByDateDesc(
    (competition.events || []).filter((event) => eventMatchesLocationType(event, selectedLocationType)),
  );
  const safeCompetition = {
    ...competition,
    events: visibleEvents,
    years: competitionYears(competition),
    location_types: locationTypes,
  };
  const overview = getCompetitionOverview(safeCompetition);
  const sortedYears = normalizeYears(safeCompetition.years);
  const locationChoices = locationTypes.length
    ? locationTypes
    : selectedLocationType
      ? [selectedLocationType]
      : [];

  return renderStructureShell({
    eyebrow: 'Competition',
    title: safeCompetition.name,
    actionHref: `/competitions/${safeCompetition.id}`,
    actionLabel: 'Open competition',
    body: `
      ${renderStatGrid([
        {
          label: 'Year',
          value: selectedYear ? String(selectedYear) : escapeHtml(structureYearsLabel(safeCompetition.years)),
          hint: `${escapeHtml(structureYearsLabel(safeCompetition.years))} available`,
        },
        {
          label: 'Events',
          value: formatNumber(overview.eventCount),
          hint: 'Shown below',
        },
        {
          label: 'Unique teams',
          value: formatNumber(overview.uniqueTeams),
          hint: 'Distinct teams',
        },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
          hint: `Year ${selectedYear || overview.snapshotYear}`,
        },
      ])}

      <section class="competition-workbench">
        <aside class="competition-workbench__aside">
          <details class="competition-filter-drawer" open>
            <summary>
              <span>Locations</span>
              <strong>${escapeHtml(selectedLocationType || 'All locations')}</strong>
            </summary>
            <form class="competition-scope-form" id="home-competition-location-filter">
              ${renderDropdown({
                name: 'locationType',
                label: 'Location level',
                options: locationChoices.map((locationType) => ({
                  id: locationType,
                  name: locationType,
                })),
                selectedValue: selectedLocationType,
                placeholder: 'Select location',
                disabled: !locationChoices.length,
              })}
              <button class="button" type="submit">Apply</button>
            </form>
            ${
              selectedLocationType
                ? `
                  <div class="competition-filter-drawer__stats">
                    <span class="eyebrow">Breakdown</span>
                    ${renderBarList(
                      locationStats.map((item) => ({
                        label: item.name,
                        value: item.total_teams,
                      })),
                      { valueFormatter: formatNumber },
                    )}
                  </div>
                `
                : ''
            }
          </details>
        </aside>

        <div class="competition-workbench__main">
          <article class="panel">
            <div class="section-head">
              <div>
                <h2>Events</h2>
              </div>
              <form class="competition-year-form" id="home-competition-year-filter">
                ${renderDropdown({
                  name: 'year',
                  label: 'Year',
                  options: sortedYears.map((year) => ({
                    id: String(year),
                    name: `Year ${year}`,
                  })),
                  selectedValue: selectedYear,
                  placeholder: 'Select year',
                  disabled: !sortedYears.length,
                  hideLabel: true,
                })}
                <button class="button" type="submit">Apply</button>
              </form>
            </div>
            ${
              visibleEvents.length
                ? renderCompetitionEventsTable(visibleEvents, safeCompetition, selectedYear)
                : renderEmptyState('No events', 'No events match the selected filters.')
            }
          </article>
        </div>
      </section>
    `,
  });
}

function renderCompetitionCollectionResult(competitions = []) {
  return renderStructureShell({
    eyebrow: 'Competitions',
    title: `${competitions.length} competitions selected`,
    actionHref: '/competitions',
    actionLabel: 'Open competitions',
    body: `
      <section class="card-grid card-grid--two">
        ${competitions
          .map((competition) => {
            const safeCompetition = {
              ...competition,
              events: competition.events || [],
            };
            const overview = getCompetitionOverview(safeCompetition);
            const latestEvent = sortByDateDesc(safeCompetition.events)[0];

            return `
              <article class="list-card list-card--competition">
                <div class="list-card__title">
                  <div>
                    <h3>${escapeHtml(safeCompetition.name)}</h3>
                    <p>Snapshot ${overview.snapshotYear} · ${formatNumber(overview.eventCount)} events</p>
                  </div>
                  ${safeCompetition.gender_category ? renderChip(safeCompetition.gender_category, 'navy') : ''}
                </div>
                ${renderMetricStrip([
                  {
                    label: 'Unique teams',
                    value: formatNumber(overview.uniqueTeams),
                  },
                  {
                    label: 'Entries',
                    value: formatNumber(overview.teamEntries),
                  },
                  {
                    label: 'Contestant entries',
                    value: formatNumber(overview.participantEntries),
                  },
                ])}
                ${latestEvent ? `<p class="card-note">Latest fixture: ${escapeHtml(latestEvent.name)} on ${escapeHtml(formatDate(latestEvent.date))}.</p>` : ''}
              </article>
            `;
          })
          .join('')}
      </section>
    `,
  });
}

function renderInstitutionStructureResult(institution) {
  if (!institution) {
    return renderEmptyState('Institution not found', 'Selected institution is unavailable.');
  }

  const safeInstitution = {
    ...institution,
    competitions: institution.competitions || [],
  };
  const overview = getInstitutionOverview(safeInstitution);
  const events = sortByDateDesc(
    safeInstitution.competitions.flatMap((competition) =>
      (competition.events || []).map((event) => ({
        ...event,
        competitionId: competition.id,
        competitionName: competition.name,
      })),
    ),
  );

  return renderStructureShell({
    eyebrow: 'Institution',
    title: safeInstitution.short_name || safeInstitution.name,
    actionHref: `/institutions/${safeInstitution.id}`,
    actionLabel: 'Open institution',
    body: `
      ${renderStatGrid([
        {
          label: 'Competitions',
          value: formatNumber(overview.competitionCount),
          hint: 'Latest participation snapshot each',
        },
        { label: 'Snapshot events', value: formatNumber(overview.eventCount) },
        { label: 'Team entries', value: formatNumber(overview.teamEntries) },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
          hint: `Reference seasons: ${overview.snapshotYears.join(', ')}`,
        },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition breakdown</span>
              <h2>Competitions</h2>
            </div>
          </div>
          ${
            safeInstitution.competitions.length
              ? renderTable({
                  compact: true,
                  columns: [
                    { label: 'Competition' },
                    { label: 'Events', align: 'right' },
                    { label: 'Teams', align: 'right' },
                    { label: 'Female entries', align: 'right' },
                  ],
                  rows: safeInstitution.competitions.map((competition) => {
                    const teams = (competition.events || []).flatMap((event) => eventTeams(event));

                    return [
                      {
                        value: `<a href="/competitions/${competition.id}" data-link><strong>${escapeHtml(competition.name)}</strong></a>`,
                      },
                      {
                        value: formatNumber((competition.events || []).length),
                        align: 'right',
                      },
                      { value: formatNumber(teams.length), align: 'right' },
                      {
                        value: formatNumber(
                          teams.reduce((total, team) => total + Number(team.female_participants || 0), 0),
                        ),
                        align: 'right',
                      },
                    ];
                  }),
                })
              : renderEmptyState('No competitions', 'This institution has no competitions.')
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Events</span>
              <h2>Recent events</h2>
            </div>
          </div>
          ${
            events.length
              ? `
                <div class="timeline-list">
                  ${events
                    .slice(0, 8)
                    .map(
                      (event) => `
                        <a class="timeline-item" href="/events/${event.id}${serialiseQuery({
                          year: eventYear(event),
                        })}" data-link>
                          <span>${escapeHtml(formatDate(event.date))}</span>
                          <strong>${escapeHtml(event.name)}</strong>
                          <small>${escapeHtml(event.competitionName)}</small>
                        </a>
                      `,
                    )
                    .join('')}
                </div>
              `
              : '<p class="card-note">No events for this institution.</p>'
          }
        </article>
      </section>
    `,
  });
}

function renderTeamStructureResult(team) {
  if (!team) {
    return renderEmptyState('Team not found', 'Selected team is unavailable.');
  }

  const safeTeam = { ...team, competitions: team.competitions || [] };
  const overview = getTeamOverview(safeTeam);
  const events = sortByDateDesc(
    safeTeam.competitions.flatMap((competition) =>
      (competition.events || []).map((event) => ({
        ...event,
        competitionId: competition.id,
        competitionName: competition.name,
      })),
    ),
  );

  return renderStructureShell({
    eyebrow: 'Team',
    title: safeTeam.name,
    actionHref: `/teams/${safeTeam.id}`,
    actionLabel: 'Open team',
    body: `
      ${renderStatGrid([
        {
          label: 'Competitions',
          value: formatNumber(overview.competitionCount),
          hint: 'Portfolio',
        },
        {
          label: 'Snapshot events',
          value: formatNumber(overview.eventCount),
          hint: 'Latest participation',
        },
        {
          label: 'Contestant entries',
          value: formatNumber(overview.participantEntries),
          hint: `Reference seasons: ${overview.snapshotYears.join(', ')}`,
        },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
        },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition portfolio</span>
              <h2>Competitions</h2>
            </div>
          </div>
          ${
            safeTeam.competitions.length
              ? renderTable({
                  compact: true,
                  columns: [
                    { label: 'Competition' },
                    { label: 'Years' },
                    { label: 'Events', align: 'right' },
                    { label: 'Contestant entries', align: 'right' },
                    { label: 'Female entries', align: 'right' },
                  ],
                  rows: safeTeam.competitions.map((competition) => [
                    {
                      value: `<a href="/teams/${safeTeam.id}${serialiseQuery({
                        competition: competition.id,
                      })}" data-link><strong>${escapeHtml(competition.name)}</strong></a>`,
                    },
                    {
                      value: escapeHtml(structureYearsLabel(competitionYears(competition))),
                    },
                    {
                      value: formatNumber((competition.events || []).length),
                      align: 'right',
                    },
                    {
                      value: formatNumber(competition.total_members),
                      align: 'right',
                    },
                    {
                      value: formatNumber(competition.female_participants),
                      align: 'right',
                    },
                  ]),
                })
              : renderEmptyState('No competitions', 'This team has no competitions.')
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Events</span>
              <h2>Results</h2>
            </div>
          </div>
          ${
            events.length
              ? `
                <div class="timeline-list">
                  ${events
                    .slice(0, 8)
                    .map(
                      (event) => `
                        <a class="timeline-item" href="/events/${event.id}${serialiseQuery({
                          year: eventYear(event),
                        })}" data-link>
                          <span>${escapeHtml(formatDate(event.date))}</span>
                          <strong>${escapeHtml(event.name)}</strong>
                          <small>${escapeHtml(event.competitionName)}</small>
                        </a>
                      `,
                    )
                    .join('')}
                </div>
              `
              : '<p class="card-note">No events for this team.</p>'
          }
        </article>
      </section>
    `,
  });
}

function renderSelectedStructureResult({
  selectedOrganizerRecords,
  selectedCompetitionRecords,
  selectedInstitutionRecords,
  selectedTeamRecords,
  selectedOrganizerCompetitionId,
  selectedOrganizerEventId,
  selectedOrganizerCompetitionYear,
  organizerInspector,
  selectedCompetitionYear,
  selectedCompetitionLocationType,
  selectedCompetitionLocationStats,
}) {
  if (selectedTeamRecords.length) {
    return selectedTeamRecords.map((team) => renderTeamStructureResult(team)).join('');
  }

  if (selectedInstitutionRecords.length) {
    return selectedInstitutionRecords
      .map((institution) => renderInstitutionStructureResult(institution))
      .join('');
  }

  if (selectedCompetitionRecords.length === 1) {
    return renderCompetitionStructureResult(selectedCompetitionRecords[0], {
      selectedYear: selectedCompetitionYear,
      selectedLocationType: selectedCompetitionLocationType,
      locationStats: selectedCompetitionLocationStats,
    });
  }

  if (selectedCompetitionRecords.length > 1) {
    return renderCompetitionCollectionResult(selectedCompetitionRecords);
  }

  if (selectedOrganizerRecords.length) {
    return selectedOrganizerRecords
      .map((organizer) =>
        renderOrganizerStructureResult(organizer, {
          selectedCompetitionId: selectedOrganizerCompetitionId,
          selectedEventId: selectedOrganizerEventId,
          selectedCompetitionYear: selectedOrganizerCompetitionYear,
          inspector: organizerInspector,
        }),
      )
      .join('');
  }

  return renderEmptyState('No selection', 'Choose filters and apply them.');
}

function initCompetitionEventRows(root) {
  const table = root.querySelector('[data-event-rankings-table]');

  if (!table) {
    return;
  }

  const setRowExpanded = (row, expanded) => {
    const eventId = row.dataset.eventId;
    const detailRow = table.querySelector(`#${eventDetailId({ id: eventId })}`);
    const toggle = row.querySelector('[data-event-toggle]');

    row.classList.toggle('is-expanded', expanded);
    toggle?.setAttribute('aria-expanded', String(expanded));

    if (detailRow) {
      detailRow.hidden = !expanded;
    }
  };

  const toggleRow = (row) => {
    const toggle = row.querySelector('[data-event-toggle]');
    setRowExpanded(row, toggle?.getAttribute('aria-expanded') !== 'true');
  };

  table.addEventListener('click', (event) => {
    const row = event.target.closest('[data-event-row]');

    if (!row) {
      return;
    }

    toggleRow(row);
  });
}

function initHomeOrganizerCards(root, { selectCompetition, selectEvent }) {
  root.querySelectorAll('[data-home-organizer-card]').forEach((card) => {
    const selectCard = () => {
      const competitionId = queryId(card.dataset.competitionId);

      if (competitionId) {
        selectCompetition(competitionId);
      }
    };

    card.addEventListener('click', (event) => {
      if (event.target.closest('a, button, input, select, form, [data-custom-select]')) {
        return;
      }

      selectCard();
    });

    card.addEventListener('keydown', (event) => {
      if (!['Enter', ' '].includes(event.key)) {
        return;
      }

      event.preventDefault();
      selectCard();
    });
  });

  root.querySelectorAll('[data-home-organizer-event]').forEach((eventLink) => {
    eventLink.addEventListener('click', (event) => {
      event.preventDefault();

      const competitionId = queryId(eventLink.dataset.competitionId);
      const eventId = queryId(eventLink.dataset.eventId);

      if (competitionId && eventId) {
        selectEvent(competitionId, eventId);
      }
    });
  });
}

export async function render({ query, navigate }) {
  const universe = await getUniverseSnapshot();
  const organizerOptions = universe.organizerOptions;

  const selectedOrganizers = selectedOptionIds(organizerOptions, queryIds(query.organizer));
  const competitionOptions = selectedOrganizers.length ? await getCompetitionOptions(selectedOrganizers) : [];
  const selectedCompetitions = selectedOptionIds(competitionOptions, queryIds(query.competition));
  const selectedOrganizerRecords = itemsByIds(universe.organizers, selectedOrganizers);
  const selectedCompetitionRecords = itemsByIds(universe.competitions, selectedCompetitions);
  const institutionOptions = selectedCompetitions.length
    ? await getInstitutionOptions(selectedCompetitions)
    : [];
  const selectedInstitutions = selectedOptionIds(institutionOptions, queryIds(query.institution));
  const selectedInstitutionRecords = itemsByIds(universe.institutions, selectedInstitutions);
  const rawTeamOptions =
    selectedCompetitions.length && selectedInstitutions.length
      ? await getTeamOptions(selectedCompetitions, selectedInstitutions)
      : [];
  const teamOptions = teamOptionsForDisplay(rawTeamOptions, selectedInstitutions);
  const selectedTeams = selectedOptionIds(teamOptions, queryIds(query.team));
  const selectedTeamRecords = itemsByIds(universe.teams, selectedTeams);
  const competitionOptionIds = selectedOrganizers.length ? optionIdSet(competitionOptions) : null;
  const selectedCompetitionIds = new Set(selectedCompetitions);
  const institutionCompetitionIds = new Set(
    selectedInstitutionRecords.flatMap((institution) => Array.from(competitionIdsForStructure(institution))),
  );
  const teamCompetitionIds = new Set(
    selectedTeamRecords.flatMap((team) => Array.from(competitionIdsForStructure(team))),
  );
  const hasActiveFilters = Boolean(
    selectedOrganizers.length ||
    selectedCompetitions.length ||
    selectedInstitutions.length ||
    selectedTeams.length,
  );
  const isCompetitionStructureScope = Boolean(
    selectedCompetitionRecords.length === 1 &&
    !selectedInstitutionRecords.length &&
    !selectedTeamRecords.length,
  );
  const isOrganizerStructureScope = Boolean(
    selectedOrganizerRecords.length &&
    !selectedCompetitionRecords.length &&
    !selectedInstitutionRecords.length &&
    !selectedTeamRecords.length,
  );
  let selectedCompetitionStructureRecords = selectedCompetitionRecords;
  let selectedCompetitionStructureYear = null;
  let selectedCompetitionStructureLocationType = null;
  let selectedCompetitionLocationStats = [];
  let selectedOrganizerStructureRecords = selectedOrganizerRecords;
  let selectedOrganizerCompetitionYear = null;
  let organizerInspector = {
    target: null,
    stats: null,
    locationStats: [],
    locationChoices: [],
    selectedLocationType: null,
  };

  if (isOrganizerStructureScope) {
    const selectedOrganizerCompetitionId = queryId(query.organizerCompetition);
    const selectedOrganizerEventId = queryId(query.organizerEvent);
    const selectedOrganizerCompetitionBase = selectedOrganizerRecords
      .flatMap((organizer) => organizer.competitions || [])
      .find((competition) => competition.id === selectedOrganizerCompetitionId);

    selectedOrganizerCompetitionYear = selectedOrganizerCompetitionBase
      ? selectedCompetitionYear(competitionYears(selectedOrganizerCompetitionBase), query.organizerYear)
      : null;

    const defaultOrganizerCompetitionYear = selectedOrganizerCompetitionBase
      ? latestYear(competitionYears(selectedOrganizerCompetitionBase))
      : null;
    const selectedOrganizerYearStructure =
      selectedOrganizerCompetitionBase &&
      selectedOrganizerCompetitionYear &&
      selectedOrganizerCompetitionYear !== defaultOrganizerCompetitionYear
        ? await getOrganizerCompetitionYearStructure(
            selectedOrganizerCompetitionBase.id,
            selectedOrganizerCompetitionYear,
          )
        : null;

    selectedOrganizerStructureRecords = selectedOrganizerRecords.map((organizer) =>
      replaceStructureCompetitionSeason(
        organizer,
        selectedOrganizerCompetitionId,
        selectedOrganizerYearStructure,
      ),
    );

    const selectedOrganizerCompetition = selectedOrganizerStructureRecords
      .flatMap((organizer) => organizer.competitions || [])
      .find((competition) => competition.id === selectedOrganizerCompetitionId);
    const selectedOrganizerEvent = findStructureEvent(selectedOrganizerCompetition, selectedOrganizerEventId);

    if (selectedOrganizerCompetition) {
      const targetYear =
        selectedOrganizerCompetitionYear || latestYear(competitionYears(selectedOrganizerCompetition));
      const locationChoices = selectedOrganizerEvent
        ? normalizeLocationTypes(
            eventLocationTypes(selectedOrganizerEvent).length
              ? eventLocationTypes(selectedOrganizerEvent)
              : selectedOrganizerCompetition.location_types || [],
          )
        : competitionLocationTypes(selectedOrganizerCompetition);
      const organizerLocationType = query.organizerLocationType
        ? selectedLocationType(locationChoices, query.organizerLocationType)
        : null;

      if (selectedOrganizerEvent) {
        const [stats, locationStats] = await Promise.all([
          getEventStats(selectedOrganizerEvent.id, targetYear),
          organizerLocationType
            ? getEventLocationStats(selectedOrganizerEvent.id, organizerLocationType, targetYear)
            : Promise.resolve([]),
        ]);

        organizerInspector = {
          target: {
            kind: 'Event',
            name: selectedOrganizerEvent.name,
            competitionId: selectedOrganizerCompetition.id,
            eventId: selectedOrganizerEvent.id,
            year: targetYear,
          },
          stats,
          locationStats,
          locationChoices,
          selectedLocationType: organizerLocationType,
        };
      } else {
        const [stats, locationStats] = await Promise.all([
          getCompetitionStats(selectedOrganizerCompetition.id, targetYear),
          organizerLocationType
            ? getCompetitionLocationStats(selectedOrganizerCompetition.id, organizerLocationType, targetYear)
            : Promise.resolve([]),
        ]);

        organizerInspector = {
          target: {
            kind: 'Competition',
            name: selectedOrganizerCompetition.name,
            competitionId: selectedOrganizerCompetition.id,
            year: targetYear,
          },
          stats,
          locationStats,
          locationChoices,
          selectedLocationType: organizerLocationType,
        };
      }
    }
  }

  if (isCompetitionStructureScope) {
    const selectedCompetitionRecord = selectedCompetitionRecords[0];
    const availableYears = competitionYears(selectedCompetitionRecord);
    selectedCompetitionStructureYear = selectedCompetitionYear(availableYears, query.year);
    const defaultYear = latestYear(availableYears);
    const shouldLoadYearStructure =
      Boolean(selectedCompetitionStructureYear) && selectedCompetitionStructureYear !== defaultYear;
    const yearStructure = shouldLoadYearStructure
      ? await getCompetitionYearStructure(selectedCompetitionRecord.id, selectedCompetitionStructureYear)
      : null;
    const structureEvents = yearStructure?.events || selectedCompetitionRecord.events || [];
    const locationTypes = normalizeLocationTypes(
      yearStructure?.location_types?.length
        ? yearStructure.location_types
        : selectedCompetitionRecord.location_types?.length
          ? selectedCompetitionRecord.location_types
          : structureEvents.flatMap(eventLocationTypes),
    );

    selectedCompetitionStructureLocationType = query.locationType
      ? selectedLocationType(locationTypes, query.locationType)
      : null;
    selectedCompetitionStructureRecords = [
      {
        ...selectedCompetitionRecord,
        events: structureEvents,
        location_types: locationTypes,
      },
    ];

    if (selectedCompetitionStructureYear && selectedCompetitionStructureLocationType) {
      selectedCompetitionLocationStats = await getCompetitionLocationStats(
        selectedCompetitionRecord.id,
        selectedCompetitionStructureLocationType,
        selectedCompetitionStructureYear,
      );
    }
  }

  const scopedCompetitions = universe.competitions
    .filter((competition) => {
      if (selectedOrganizers.length && !competitionOptionIds.has(competition.id)) {
        return false;
      }

      if (selectedCompetitionIds.size && !selectedCompetitionIds.has(competition.id)) {
        return false;
      }

      if (selectedInstitutions.length && !institutionCompetitionIds.has(competition.id)) {
        return false;
      }

      if (selectedTeams.length && !teamCompetitionIds.has(competition.id)) {
        return false;
      }

      return true;
    })
    .map((competition) => filterCompetitionEntries(competition, selectedInstitutionRecords, selectedTeams))
    .filter((competition) => !hasActiveFilters || competition.events.length);

  const scopedCompetitionIds = optionIdSet(scopedCompetitions);
  const competitionScope = hasActiveFilters ? scopedCompetitionIds : null;

  const competitions = scopedCompetitions
    .map((competition) => ({
      competition,
      overview: getCompetitionOverview(competition),
    }))
    .sort((left, right) => right.overview.uniqueTeams - left.overview.uniqueTeams);

  const scopedInstitutions = universe.institutions
    .filter((institution) => !selectedInstitutions.length || selectedInstitutions.includes(institution.id))
    .map((institution) => filterInstitutionEntries(institution, competitionScope, selectedTeams))
    .filter((institution) => !hasActiveFilters || institution.competitions.length);

  const institutions = scopedInstitutions
    .map((institution) => ({
      institution,
      overview: getInstitutionOverview(institution),
    }))
    .sort((left, right) => right.overview.teamEntries - left.overview.teamEntries)
    .slice(0, 8);

  const organizers = universe.organizers
    .filter((organizer) => !selectedOrganizers.length || selectedOrganizers.includes(organizer.id))
    .map((organizer) => filterOrganizerEntries(organizer, competitionScope))
    .filter((organizer) => !hasActiveFilters || organizer.competitions.length)
    .map((organizer) => ({
      organizer,
      overview: getOrganizerOverview(organizer),
    }))
    .sort((left, right) => left.organizer.name.localeCompare(right.organizer.name))
    .slice(0, 6);

  const scopedEvents = competitionEvents(scopedCompetitions);
  const latestEvents = scopedEvents.slice(0, 8);
  const heroCompetition = competitions[0]?.competition;
  const activeFilterChips = [
    selectedOrganizers.length
      ? selectionChipLabel(organizerOptions, selectedOrganizers, 'Organizer', 'Organizers')
      : null,
    selectedCompetitions.length
      ? selectionChipLabel(competitionOptions, selectedCompetitions, 'Competition', 'Competitions')
      : null,
    selectedInstitutions.length
      ? selectionChipLabel(institutionOptions, selectedInstitutions, 'Institution', 'Institutions')
      : null,
    selectedTeams.length ? selectionChipLabel(teamOptions, selectedTeams, 'Team', 'Teams') : null,
    isOrganizerStructureScope && selectedOrganizerCompetitionYear
      ? `Year: ${selectedOrganizerCompetitionYear}`
      : null,
    isOrganizerStructureScope && organizerInspector.selectedLocationType
      ? `Location: ${organizerInspector.selectedLocationType}`
      : null,
    isCompetitionStructureScope && selectedCompetitionStructureYear
      ? `Year: ${selectedCompetitionStructureYear}`
      : null,
    isCompetitionStructureScope && selectedCompetitionStructureLocationType
      ? `Location: ${selectedCompetitionStructureLocationType}`
      : null,
  ]
    .filter(Boolean)
    .map((label) => renderChip(label, 'slate'));
  const activeStructureSummary = selectedTeams.length
    ? selectionSummaryLabel(optionLabels(teamOptions, selectedTeams))
    : selectedInstitutions.length
      ? selectionSummaryLabel(optionLabels(institutionOptions, selectedInstitutions))
      : selectedCompetitions.length
        ? selectionSummaryLabel(
            optionLabels(competitionOptions, selectedCompetitions),
            selectedCompetitionStructureYear ? ` in ${selectedCompetitionStructureYear}` : '',
          )
        : selectedOrganizers.length
          ? selectionSummaryLabel(optionLabels(organizerOptions, selectedOrganizers))
          : null;

  const html = `
    ${renderPageIntro({
      eyebrow: 'Dashboard',
      title: 'Competition overview',
      blurb: 'Browse competitions, teams, institutions and organizers in one place.',
      actions: `
        <div class="action-stack">
          <a class="button" href="/competitions" data-link>Browse competitions</a>
          <a class="button button--ghost" href="/teams" data-link>Browse teams</a>
        </div>
      `,
      meta: [
        renderChip(`${universe.organizers.length} organizers`, 'navy'),
        renderChip(`${universe.competitions.length} competitions`, 'navy'),
        renderChip(`${universe.events.length} latest-season events`, 'navy'),
      ],
    })}

    <section class="panel panel--filters" aria-label="Entity filters">
      <form class="filter-bar filter-bar--cascade" id="home-cascade-filters">
        ${renderDropdown({
          name: 'organizer',
          label: 'Organizer',
          options: organizerOptions,
          selectedValue: selectedOrganizers,
          placeholder: 'Select organizers',
          multiple: true,
        })}
        ${renderDropdown({
          name: 'competition',
          label: 'Competition',
          options: competitionOptions,
          selectedValue: selectedCompetitions,
          placeholder: selectedOrganizers.length ? 'Select competitions' : 'Select organizer first',
          disabled: !selectedOrganizers.length,
          multiple: true,
        })}
        ${renderDropdown({
          name: 'institution',
          label: 'Institution',
          options: institutionOptions,
          selectedValue: selectedInstitutions,
          placeholder: selectedCompetitions.length ? 'Select institutions' : 'Select competition first',
          disabled: !selectedCompetitions.length,
          multiple: true,
        })}
        ${renderDropdown({
          name: 'team',
          label: 'Team',
          options: teamOptions,
          selectedValue: selectedTeams,
          placeholder: selectedInstitutions.length ? 'Select teams' : 'Select institution first',
          disabled: !selectedInstitutions.length,
          multiple: true,
        })}
        <button class="button" type="submit">Apply Filters</button>
        <a class="button button--ghost" href="/" data-link>Clear</a>
      </form>
      <div class="filter-summary">
        <p>
          ${
            hasActiveFilters
              ? activeStructureSummary
              : `${formatNumber(universe.competitions.length)} competitions, ${formatNumber(universe.events.length)} events and ${formatNumber(uniqueTeamCount(universe.competitions))} unique teams in the latest available season snapshot of each competition.`
          }
        </p>
        <div class="filter-summary__chips">
          ${activeFilterChips.length ? activeFilterChips.join('') : renderChip('Latest season snapshots', 'navy')}
        </div>
      </div>
    </section>

    ${
      hasActiveFilters
        ? renderSelectedStructureResult({
            selectedOrganizerRecords: selectedOrganizerStructureRecords,
            selectedCompetitionRecords: selectedCompetitionStructureRecords,
            selectedInstitutionRecords,
            selectedTeamRecords,
            selectedOrganizerCompetitionId: queryId(query.organizerCompetition),
            selectedOrganizerEventId: queryId(query.organizerEvent),
            selectedOrganizerCompetitionYear,
            organizerInspector,
            selectedCompetitionYear: selectedCompetitionStructureYear,
            selectedCompetitionLocationType: selectedCompetitionStructureLocationType,
            selectedCompetitionLocationStats,
          })
        : ''
    }

    <div class="home-dashboard" ${hasActiveFilters ? 'hidden' : ''}>
    <section class="hero-grid">
      <article class="hero-card hero-card--feature">
        <span class="eyebrow">Overview</span>
        <h2>${escapeHtml(heroCompetition?.name || 'Competition map')}</h2>
        <p>
          ${escapeHtml(
            heroCompetition
              ? 'The most connected competition anchors the overview below.'
              : 'Start with any competition, team, institution or organizer from the navigation above.',
          )}
        </p>
        ${
          heroCompetition
            ? renderMetricStrip([
                {
                  label: 'Years',
                  value: getCompetitionOverview(heroCompetition).yearSpan,
                },
                {
                  label: 'Events',
                  value: formatNumber(getCompetitionOverview(heroCompetition).eventCount),
                },
                {
                  label: 'Unique teams',
                  value: formatNumber(getCompetitionOverview(heroCompetition).uniqueTeams),
                },
                {
                  label: 'Female entries',
                  value: formatNumber(getCompetitionOverview(heroCompetition).femaleParticipantEntries),
                },
              ])
            : ''
        }
        ${heroCompetition ? `<a class="button" href="/competitions/${heroCompetition.id}" data-link>Open competition</a>` : ''}
      </article>

      <article class="hero-card">
        <span class="eyebrow">Summary</span>
        <h3>${hasActiveFilters ? 'Selected data' : 'Latest season snapshots'}</h3>
        ${renderStatGrid([
          {
            label: 'Competitions',
            value: formatNumber(scopedCompetitions.length),
            hint: 'Calendars',
          },
          {
            label: 'Teams',
            value: formatNumber(uniqueTeamCount(scopedCompetitions)),
            hint: 'Distinct squads',
          },
          {
            label: 'Institutions',
            value: formatNumber(scopedInstitutions.length),
            hint: 'Academic programs',
          },
          {
            label: 'Events',
            value: formatNumber(scopedEvents.length),
            hint: 'Latest season each',
          },
        ])}
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Competitions</span>
          <h2>${hasActiveFilters ? 'Selected competitions' : 'Competition snapshots by unique teams'}</h2>
        </div>
        <a class="button button--ghost" href="/competitions" data-link>Full competition index</a>
      </div>
      ${
        competitions.length
          ? `
        <div class="card-grid card-grid--two">
          ${competitions
            .slice(0, 4)
            .map(({ competition }) => renderCompetitionSpotlight(competition))
            .join('')}
        </div>
      `
          : renderEmptyState('No competitions', 'This filter combination has no competition results.')
      }
    </section>

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Institutions</span>
            <h2>Institution snapshots by team entries</h2>
          </div>
          <a class="button button--ghost" href="/institutions" data-link>Browse institutions</a>
        </div>
        ${
          institutions.length
            ? renderTable({
                compact: true,
                columns: [
                  { label: 'Institution' },
                  { label: 'Location' },
                  { label: 'Entries', align: 'right' },
                  { label: 'Female entries', align: 'right' },
                ],
                rows: institutions.map(({ institution, overview }) => [
                  {
                    value: `<a href="/institutions/${institution.id}" data-link><strong>${escapeHtml(institution.short_name || institution.name)}</strong></a><small>${escapeHtml(institution.name)}</small>`,
                  },
                  { value: escapeHtml(institution.location) },
                  { value: formatNumber(overview.teamEntries), align: 'right' },
                  {
                    value: formatNumber(overview.femaleParticipantEntries),
                    align: 'right',
                  },
                ]),
              })
            : '<p class="card-note">No institutions in this selection.</p>'
        }
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Events</span>
            <h2>Recent events</h2>
          </div>
        </div>
        ${
          latestEvents.length
            ? `
          <div class="timeline-list">
            ${latestEvents
              .map(
                (event) => `
                <a class="timeline-item" href="/events/${event.id}${serialiseQuery({ year: event.year })}" data-link>
                  <span>${escapeHtml(formatDate(event.date))}</span>
                  <strong>${escapeHtml(event.name)}</strong>
                  <small>${escapeHtml(event.competitionName)} · ${escapeHtml(event.location)}</small>
                </a>
              `,
              )
              .join('')}
          </div>
        `
            : '<p class="card-note">No recent events in this selection.</p>'
        }
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Organizers</span>
          <h2>Organizer portfolio snapshots</h2>
        </div>
        <a class="button button--ghost" href="/organizers" data-link>Organizer overview</a>
      </div>
      <div class="card-grid card-grid--three">
        ${
          organizers.length
            ? organizers
                .map(
                  ({ organizer, overview }) => `
              <article class="list-card">
                <div class="list-card__title">
                  <h3><a href="/organizers/${organizer.id}" data-link>${escapeHtml(organizer.name)}</a></h3>
                  ${renderChip(`${overview.competitionCount} competitions`, 'slate')}
                </div>
                ${renderMetricStrip([
                  { label: 'Events', value: formatNumber(overview.eventCount) },
                  {
                    label: 'Team entries',
                    value: formatNumber(overview.teamEntries),
                  },
                  {
                    label: 'Female entries',
                    value: formatNumber(overview.femaleParticipantEntries),
                  },
                ])}
              </article>
            `,
                )
                .join('')
            : '<p class="card-note">No organizers in this selection.</p>'
        }
      </div>
    </section>
    </div>
  `;

  return {
    title: 'Home',
    html,
    afterRender() {
      const form = document.getElementById('home-cascade-filters');
      const organizerSelect = form.querySelector('select[name="organizer"]');
      const competitionSelect = form.querySelector('select[name="competition"]');
      const institutionSelect = form.querySelector('select[name="institution"]');
      const teamSelect = form.querySelector('select[name="team"]');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);
      initCompetitionEventRows(document);

      let localOrganizerRecords = selectedOrganizerStructureRecords;
      const localOrganizerBaseRecords = selectedOrganizerRecords;
      let localOrganizerCompetitionId = queryId(query.organizerCompetition);
      let localOrganizerEventId = queryId(query.organizerEvent);
      let localOrganizerYear = selectedOrganizerCompetitionYear;
      let localOrganizerInspector = organizerInspector;

      const findOrganizerCompetition = (records, competitionId) =>
        records
          .flatMap((organizer) => organizer.competitions || [])
          .find((competition) => competition.id === competitionId) || null;

      const buildOrganizerInspector = ({
        competitionId,
        eventId = null,
        locationType = null,
        stats = null,
        locationStats = [],
      }) => {
        const competition = findOrganizerCompetition(localOrganizerRecords, competitionId);

        if (!competition) {
          return {
            target: null,
            stats: null,
            locationStats: [],
            locationChoices: [],
            selectedLocationType: null,
          };
        }

        const targetYear = localOrganizerYear || latestYear(competitionYears(competition));
        const selectedEvent = findStructureEvent(competition, eventId);
        const locationChoices = selectedEvent
          ? normalizeLocationTypes(
              eventLocationTypes(selectedEvent).length
                ? eventLocationTypes(selectedEvent)
                : competition.location_types || [],
            )
          : competitionLocationTypes(competition);

        return {
          target: selectedEvent
            ? {
                kind: 'Event',
                name: selectedEvent.name,
                competitionId: competition.id,
                eventId: selectedEvent.id,
                year: targetYear,
              }
            : {
                kind: 'Competition',
                name: competition.name,
                competitionId: competition.id,
                year: targetYear,
              },
          stats,
          locationStats,
          locationChoices,
          selectedLocationType: locationType,
        };
      };

      const renderLocalOrganizerResult = () => {
        if (!isOrganizerStructureScope) {
          return;
        }

        const currentResult = document.querySelector('[data-structure-result]');

        if (!currentResult) {
          return;
        }

        currentResult.outerHTML = renderSelectedStructureResult({
          selectedOrganizerRecords: localOrganizerRecords,
          selectedCompetitionRecords: [],
          selectedInstitutionRecords: [],
          selectedTeamRecords: [],
          selectedOrganizerCompetitionId: localOrganizerCompetitionId,
          selectedOrganizerEventId: localOrganizerEventId,
          selectedOrganizerCompetitionYear: localOrganizerYear,
          organizerInspector: localOrganizerInspector,
          selectedCompetitionYear: null,
          selectedCompetitionLocationType: null,
          selectedCompetitionLocationStats: [],
        });

        const nextResult = document.querySelector('[data-structure-result]');
        initCustomDropdowns(nextResult, dropdownAbortController.signal);
        initHomeOrganizerCards(nextResult, organizerController);
        wireHomeOrganizerForms(nextResult);
      };

      async function selectCompetition(competitionId) {
        const competition = findOrganizerCompetition(localOrganizerRecords, competitionId);

        if (!competition) {
          return;
        }

        const previousCompetitionId = localOrganizerCompetitionId;
        localOrganizerCompetitionId = competitionId;
        localOrganizerEventId = null;
        localOrganizerYear =
          previousCompetitionId === competitionId
            ? selectedCompetitionYear(competitionYears(competition), localOrganizerYear)
            : latestYear(competitionYears(competition));
        const stats = await getCompetitionStats(competitionId, localOrganizerYear);
        localOrganizerInspector = buildOrganizerInspector({
          competitionId,
          stats,
        });
        renderLocalOrganizerResult();
      }

      async function selectEvent(competitionId, eventId) {
        const competition = findOrganizerCompetition(localOrganizerRecords, competitionId);

        if (!competition || !findStructureEvent(competition, eventId)) {
          return;
        }

        localOrganizerCompetitionId = competitionId;
        localOrganizerEventId = eventId;
        localOrganizerYear = selectedCompetitionYear(competitionYears(competition), localOrganizerYear);
        const stats = await getEventStats(eventId, localOrganizerYear);
        localOrganizerInspector = buildOrganizerInspector({
          competitionId,
          eventId,
          stats,
        });
        renderLocalOrganizerResult();
      }

      async function selectOrganizerCompetitionYear(competitionId, year) {
        const baseCompetition = findOrganizerCompetition(localOrganizerBaseRecords, competitionId);

        if (!baseCompetition) {
          return;
        }

        const defaultYear = latestYear(competitionYears(baseCompetition));
        const yearStructure =
          year && year !== defaultYear
            ? await getOrganizerCompetitionYearStructure(baseCompetition.id, year)
            : null;

        localOrganizerRecords = localOrganizerBaseRecords.map((organizer) =>
          replaceStructureCompetitionSeason(organizer, competitionId, yearStructure),
        );
        localOrganizerCompetitionId = competitionId;
        localOrganizerEventId = null;
        localOrganizerYear = year || defaultYear;
        const stats = await getCompetitionStats(competitionId, localOrganizerYear);
        localOrganizerInspector = buildOrganizerInspector({
          competitionId,
          stats,
        });
        renderLocalOrganizerResult();
      }

      async function applyOrganizerLocationFilter(filterForm) {
        const data = new FormData(filterForm);
        const competitionId = queryId(data.get('organizerCompetition'));
        const eventId = queryId(data.get('organizerEvent'));
        const year = queryId(data.get('organizerYear'));
        const locationType = String(data.get('organizerLocationType') || '');

        if (!competitionId || !locationType) {
          return;
        }

        const [stats, locationStats] = eventId
          ? await Promise.all([
              getEventStats(eventId, year),
              getEventLocationStats(eventId, locationType, year),
            ])
          : await Promise.all([
              getCompetitionStats(competitionId, year),
              getCompetitionLocationStats(competitionId, locationType, year),
            ]);

        localOrganizerCompetitionId = competitionId;
        localOrganizerEventId = eventId;
        localOrganizerYear = year;
        localOrganizerInspector = buildOrganizerInspector({
          competitionId,
          eventId,
          locationType,
          stats,
          locationStats,
        });
        renderLocalOrganizerResult();
      }

      function wireHomeOrganizerForms(root) {
        root.querySelectorAll('[data-home-organizer-year-form]').forEach((yearForm) => {
          yearForm.addEventListener('change', (event) => {
            if (!event.target.matches('select')) {
              return;
            }

            const data = new FormData(yearForm);
            selectOrganizerCompetitionYear(
              queryId(data.get('organizerCompetition')),
              queryId(data.get('organizerYear')),
            );
          });
        });

        const locationFilter = root.querySelector('#home-organizer-location-filter');

        locationFilter?.addEventListener('submit', (event) => {
          event.preventDefault();
          applyOrganizerLocationFilter(event.currentTarget);
        });

        locationFilter?.addEventListener('change', (event) => {
          if (event.target.matches('select')) {
            event.currentTarget.requestSubmit();
          }
        });
      }

      const organizerController = { selectCompetition, selectEvent };

      if (isOrganizerStructureScope) {
        initHomeOrganizerCards(document, organizerController);
        wireHomeOrganizerForms(document);
      }

      const selectValues = (select) =>
        Array.from(select.selectedOptions)
          .map((option) => option.value)
          .filter(Boolean);

      const readState = () => ({
        organizer: queryListValue(selectValues(organizerSelect)),
        competition: queryListValue(selectValues(competitionSelect)),
        institution: queryListValue(selectValues(institutionSelect)),
        team: queryListValue(selectValues(teamSelect)),
      });

      let cascadeRefreshSequence = 0;
      const refreshCascade = async (changedName) => {
        const currentRefresh = ++cascadeRefreshSequence;
        const isCurrentRefresh = () => currentRefresh === cascadeRefreshSequence;
        const organizers = selectValues(organizerSelect).map(Number);
        const nextCompetitionOptions = organizers.length ? await getCompetitionOptions(organizers) : [];

        if (!isCurrentRefresh()) {
          return;
        }

        const competitions =
          changedName === 'organizer'
            ? []
            : selectedOptionIds(nextCompetitionOptions, selectValues(competitionSelect).map(Number));
        const nextInstitutionOptions = competitions.length ? await getInstitutionOptions(competitions) : [];

        if (!isCurrentRefresh()) {
          return;
        }

        const institutions = ['organizer', 'competition'].includes(changedName)
          ? []
          : selectedOptionIds(nextInstitutionOptions, selectValues(institutionSelect).map(Number));
        const nextRawTeamOptions =
          competitions.length && institutions.length ? await getTeamOptions(competitions, institutions) : [];
        const nextTeamOptions = teamOptionsForDisplay(nextRawTeamOptions, institutions);

        if (!isCurrentRefresh()) {
          return;
        }

        const teams =
          changedName === 'team'
            ? selectedOptionIds(nextTeamOptions, selectValues(teamSelect).map(Number))
            : [];

        setDropdownOptions(competitionSelect, {
          options: nextCompetitionOptions,
          selectedValue: competitions,
          placeholder: organizers.length ? 'Select competitions' : 'Select organizer first',
          disabled: !organizers.length,
        });
        setDropdownOptions(institutionSelect, {
          options: nextInstitutionOptions,
          selectedValue: institutions,
          placeholder: competitions.length ? 'Select institutions' : 'Select competition first',
          disabled: !competitions.length,
        });
        setDropdownOptions(teamSelect, {
          options: nextTeamOptions,
          selectedValue: teams,
          placeholder: institutions.length ? 'Select teams' : 'Select institution first',
          disabled: !institutions.length,
        });
      };

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        navigate(`/${serialiseQuery(readState())}`);
      });

      document.getElementById('home-competition-year-filter')?.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);

        navigate(
          `/${serialiseQuery({
            organizer: queryListValue(selectedOrganizers),
            competition: queryListValue(selectedCompetitions),
            year: data.get('year'),
            locationType: selectedCompetitionStructureLocationType,
          })}`,
        );
      });

      document.getElementById('home-competition-location-filter')?.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);

        navigate(
          `/${serialiseQuery({
            organizer: queryListValue(selectedOrganizers),
            competition: queryListValue(selectedCompetitions),
            year: selectedCompetitionStructureYear,
            locationType: data.get('locationType'),
          })}`,
        );
      });

      form.addEventListener('change', async (event) => {
        if (event.target.matches('select')) {
          await refreshCascade(event.target.name);
        }
      });
    },
  };
}

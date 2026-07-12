import { getCompetitionOptions } from '../lib/api.js';
import { getUniverseSnapshot } from '../lib/data-store.js';
import {
  escapeHtml,
  formatDate,
  formatNumber,
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
          <p>${escapeHtml(overview.yearSpan)} seasons · ${escapeHtml(String(overview.eventCount))} events tracked</p>
        </div>
        <a class="button button--ghost" href="/competitions/${competition.id}" data-link>Open competition</a>
      </div>
      ${renderMetricStrip([
        { label: 'Unique teams', value: formatNumber(overview.uniqueTeams) },
        { label: 'Entries', value: formatNumber(overview.teamEntries) },
        { label: 'Participants', value: formatNumber(overview.totalParticipants) },
        { label: 'Women tracked', value: formatNumber(overview.femaleParticipants) },
      ])}
      ${latestEvent ? `<p class="card-note">Latest event: <a href="/events/${latestEvent.id}${serialiseQuery({ year: new Date(`${latestEvent.date}T00:00:00`).getFullYear(), name: latestEvent.name, date: latestEvent.date, location: latestEvent.location, locationTypes: (latestEvent.location_types || []).join(','), competitionId: competition.id, competitionName: competition.name })}" data-link>${escapeHtml(latestEvent.name)}</a> on ${escapeHtml(formatDate(latestEvent.date))}</p>` : ''}
    </article>
  `;
}

function queryId(value) {
  const id = Number(value);
  return Number.isInteger(id) && id > 0 ? id : null;
}

function selectedOptionId(options, requestedId) {
  return options.some((option) => option.id === requestedId) ? requestedId : null;
}

function itemById(items, id) {
  return id ? items.find((item) => item.id === id) || null : null;
}

function optionLabel(options, id) {
  return itemById(options, id)?.name || null;
}

function optionIdSet(options) {
  return new Set(options.map((option) => option.id));
}

function optionItems(options, placeholder) {
  return [
    { id: '', name: placeholder },
    ...options.map((option) => ({ id: String(option.id), name: option.name })),
  ];
}

function isSelectedOption(value, selectedValue) {
  return String(selectedValue ?? '') === String(value ?? '');
}

function optionMarkup(options, selectedValue, placeholder) {
  return optionItems(options, placeholder)
    .map(
      (option) =>
        `<option value="${escapeHtml(option.id)}" ${isSelectedOption(option.id, selectedValue) ? 'selected' : ''}>${escapeHtml(option.name)}</option>`,
    )
    .join('');
}

function dropdownOptionMarkup(items, selectedValue) {
  return items
    .map(
      (option) => `
        <button
          class="app-select__option ${isSelectedOption(option.id, selectedValue) ? 'is-selected' : ''}"
          type="button"
          role="option"
          aria-selected="${isSelectedOption(option.id, selectedValue)}"
          data-custom-select-option
          data-value="${escapeHtml(option.id)}"
        >
          ${escapeHtml(option.name)}
        </button>
      `,
    )
    .join('');
}

function renderDropdown({ name, label, options, selectedValue, placeholder, disabled = false }) {
  const items = optionItems(options, placeholder);
  const selectedItem = items.find((item) => isSelectedOption(item.id, selectedValue)) || items[0];
  const disabledAttribute = disabled ? 'disabled' : '';

  return `
    <label class="filter-step ${disabled ? 'is-disabled' : ''}">
      <span>${escapeHtml(label)}</span>
      <div class="app-select ${disabled ? 'is-disabled' : ''}" data-custom-select>
        <select class="app-select__native" name="${escapeHtml(name)}" aria-label="${escapeHtml(label)}" ${disabledAttribute}>
          ${optionMarkup(options, selectedValue, placeholder)}
        </select>
        <button
          class="app-select__trigger"
          type="button"
          aria-haspopup="listbox"
          aria-expanded="false"
          data-custom-select-trigger
          ${disabledAttribute}
        >
          <span data-custom-select-label>${escapeHtml(selectedItem.name)}</span>
          <span class="app-select__arrow" aria-hidden="true"></span>
        </button>
        <div class="app-select__menu" role="listbox" data-custom-select-menu hidden>
          ${dropdownOptionMarkup(items, selectedValue)}
        </div>
      </div>
    </label>
  `;
}

function sortOptions(options) {
  return [...options].sort((left, right) => left.name.localeCompare(right.name));
}

function optionsByIds(options, ids) {
  return sortOptions(options.filter((option) => ids.has(option.id)));
}

function institutionAliases(institution) {
  return new Set([institution?.name, institution?.short_name].filter(Boolean));
}

function teamMatchesInstitution(team, institution) {
  if (!institution) {
    return true;
  }

  const aliases = institutionAliases(institution);
  return aliases.has(team.institution_name) || aliases.has(team.institution_short_name);
}

function filterEventTeams(event, institution, teamId) {
  if (!institution && !teamId) {
    return event;
  }

  return {
    ...event,
    teams: (event.teams || []).filter((team) => {
      return teamMatchesInstitution(team, institution) && (!teamId || team.id === teamId);
    }),
  };
}

function filterCompetitionEntries(competition, institution, teamId) {
  if (!institution && !teamId) {
    return competition;
  }

  return {
    ...competition,
    events: (competition.events || [])
      .map((event) => filterEventTeams(event, institution, teamId))
      .filter((event) => event.teams.length),
  };
}

function competitionIdsForStructure(structure) {
  return new Set((structure?.competitions || []).map((competition) => competition.id));
}

async function loadCompetitionOptionsByOrganizer(organizerOptions) {
  const entries = await Promise.all(
    organizerOptions.map(async (organizer) => [organizer.id, await getCompetitionOptions([organizer.id])]),
  );

  return new Map(entries);
}

function competitionOptionsForOrganizer(universe, organizerId, competitionOptionsByOrganizer = null) {
  const endpointOptions = competitionOptionsByOrganizer?.get(organizerId);

  if (endpointOptions) {
    return sortOptions(endpointOptions);
  }

  const organizer = itemById(universe.organizers, organizerId);
  const ids = new Set((organizer?.competitions || []).map((competition) => competition.id));
  return optionsByIds(universe.competitionOptions, ids);
}

function institutionOptionsForCompetition(universe, competitionId) {
  const ids = new Set(
    universe.institutions
      .filter((institution) => competitionIdsForStructure(institution).has(competitionId))
      .map((institution) => institution.id),
  );
  return optionsByIds(universe.institutionOptions, ids);
}

function teamOptionsForCompetitionAndInstitution(universe, competitionId, institution) {
  const competition = itemById(universe.competitions, competitionId);
  const ids = new Set(
    (competition?.events || []).flatMap((event) =>
      (event.teams || []).filter((team) => teamMatchesInstitution(team, institution)).map((team) => team.id),
    ),
  );

  return optionsByIds(universe.teamOptions, ids);
}

function filterInstitutionEntries(institution, competitionIds, teamId) {
  return {
    ...institution,
    competitions: (institution.competitions || [])
      .filter((competition) => !competitionIds || competitionIds.has(competition.id))
      .map((competition) => ({
        ...competition,
        events: (competition.events || [])
          .map((event) => ({
            ...event,
            teams: (event.teams || []).filter((team) => !teamId || team.id === teamId),
          }))
          .filter((event) => !teamId || event.teams.length),
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

function eventTeams(event) {
  return event?.teams || [];
}

function eventParticipants(event) {
  return (
    Number(event?.total_participants || 0) ||
    eventTeams(event).reduce((total, team) => total + Number(team.total_members || 0), 0)
  );
}

function eventFemaleParticipants(event) {
  return (
    Number(event?.female_participants || 0) ||
    eventTeams(event).reduce((total, team) => total + Number(team.female_participants || 0), 0)
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

function renderOrganizerStructureResult(organizer) {
  if (!organizer) {
    return renderEmptyState('Organizer not found', 'The selected organizer structure is unavailable.');
  }

  const safeOrganizer = { ...organizer, competitions: organizer.competitions || [] };
  const overview = getOrganizerOverview(safeOrganizer);
  const latestEvents = sortByDateDesc(
    safeOrganizer.competitions.flatMap((competition) =>
      (competition.events || []).map((event) => ({
        ...event,
        competitionId: competition.id,
        competitionName: competition.name,
      })),
    ),
  ).slice(0, 6);

  return renderStructureShell({
    eyebrow: 'Organizer structure',
    title: safeOrganizer.name,
    actionHref: `/organizers/${safeOrganizer.id}`,
    actionLabel: 'Open organizer',
    body: `
      ${renderStatGrid([
        { label: 'Competitions', value: formatNumber(overview.competitionCount), hint: 'Linked structures' },
        { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Tracked calendar rows' },
        { label: 'Participants', value: formatNumber(overview.totalParticipants), hint: 'Across events' },
        { label: 'Women tracked', value: formatNumber(overview.femaleParticipants), hint: 'Absolute count' },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition structures</span>
              <h2>Competitions under this organizer</h2>
            </div>
          </div>
          ${
            safeOrganizer.competitions.length
              ? renderTable({
                  compact: true,
                  columns: [
                    { label: 'Competition' },
                    { label: 'Seasons' },
                    { label: 'Events', align: 'right' },
                    { label: 'Location tiers' },
                  ],
                  rows: safeOrganizer.competitions.map((competition) => [
                    {
                      value: `<a href="/competitions/${competition.id}" data-link><strong>${escapeHtml(competition.name)}</strong></a>`,
                    },
                    { value: escapeHtml(structureYearsLabel(competitionYears(competition))) },
                    { value: formatNumber((competition.events || []).length), align: 'right' },
                    { value: escapeHtml((competition.location_types || []).join(', ') || '-') },
                  ]),
                })
              : renderEmptyState('No competitions', 'This organizer has no competition structures attached.')
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Recent events</span>
              <h2>Latest rows in the structure</h2>
            </div>
          </div>
          ${
            latestEvents.length
              ? `
                <div class="timeline-list">
                  ${latestEvents
                    .map(
                      (event) => `
                        <a class="timeline-item" href="/events/${event.id}${serialiseQuery({
                          year: eventYear(event),
                          name: event.name,
                          date: event.date,
                          location: event.location,
                          locationTypes: (event.location_types || []).join(','),
                          competitionId: event.competitionId,
                          competitionName: event.competitionName,
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
              : '<p class="card-note">No event rows in this organizer structure.</p>'
          }
        </article>
      </section>
    `,
  });
}

function renderCompetitionStructureResult(competition) {
  if (!competition) {
    return renderEmptyState('Competition not found', 'The selected competition structure is unavailable.');
  }

  const safeCompetition = {
    ...competition,
    events: competition.events || [],
    years: competitionYears(competition),
  };
  const overview = getCompetitionOverview(safeCompetition);

  return renderStructureShell({
    eyebrow: 'Competition structure',
    title: safeCompetition.name,
    actionHref: `/competitions/${safeCompetition.id}`,
    actionLabel: 'Open competition',
    body: `
      ${renderStatGrid([
        {
          label: 'Seasons',
          value: escapeHtml(structureYearsLabel(safeCompetition.years)),
          hint: 'Tracked years',
        },
        { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Event structures' },
        { label: 'Unique teams', value: formatNumber(overview.uniqueTeams), hint: 'Distinct teams' },
        { label: 'Women tracked', value: formatNumber(overview.femaleParticipants), hint: 'Across events' },
      ])}

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Event structures</span>
            <h2>Events returned for this competition</h2>
          </div>
          <div class="tag-row">
            ${(safeCompetition.location_types || []).map((locationType) => renderChip(locationType, 'slate')).join('')}
          </div>
        </div>
        ${
          safeCompetition.events.length
            ? renderTable({
                columns: [
                  { label: 'Event' },
                  { label: 'Date' },
                  { label: 'Location' },
                  { label: 'Teams', align: 'right' },
                  { label: 'Participants', align: 'right' },
                  { label: 'Women', align: 'right' },
                ],
                rows: sortByDateDesc(safeCompetition.events).map((event) => [
                  {
                    value: `<a href="/events/${event.id}${serialiseQuery({
                      year: eventYear(event),
                      name: event.name,
                      date: event.date,
                      location: event.location,
                      locationTypes: (event.location_types || []).join(','),
                      competitionId: safeCompetition.id,
                      competitionName: safeCompetition.name,
                    })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                  },
                  { value: escapeHtml(formatDate(event.date)) },
                  { value: escapeHtml(event.location || '-') },
                  { value: formatNumber(eventTeams(event).length), align: 'right' },
                  { value: formatNumber(eventParticipants(event)), align: 'right' },
                  { value: formatNumber(eventFemaleParticipants(event)), align: 'right' },
                ]),
              })
            : renderEmptyState('No events', 'This competition structure has no event rows.')
        }
      </article>
    `,
  });
}

function renderInstitutionStructureResult(institution) {
  if (!institution) {
    return renderEmptyState('Institution not found', 'The selected institution structure is unavailable.');
  }

  const safeInstitution = { ...institution, competitions: institution.competitions || [] };
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
    eyebrow: 'Institution structure',
    title: safeInstitution.short_name || safeInstitution.name,
    actionHref: `/institutions/${safeInstitution.id}`,
    actionLabel: 'Open institution',
    body: `
      ${renderStatGrid([
        { label: 'Competitions', value: formatNumber(overview.competitionCount), hint: 'Linked structures' },
        { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Tracked appearances' },
        { label: 'Entries', value: formatNumber(overview.teamEntries), hint: 'Team rows' },
        {
          label: 'Women tracked',
          value: formatNumber(overview.femaleParticipants),
          hint: safeInstitution.location,
        },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition breakdown</span>
              <h2>Competitions in this institution structure</h2>
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
                    { label: 'Women', align: 'right' },
                  ],
                  rows: safeInstitution.competitions.map((competition) => {
                    const teams = (competition.events || []).flatMap((event) => eventTeams(event));

                    return [
                      {
                        value: `<a href="/competitions/${competition.id}" data-link><strong>${escapeHtml(competition.name)}</strong></a>`,
                      },
                      { value: formatNumber((competition.events || []).length), align: 'right' },
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
              : renderEmptyState('No competitions', 'This institution structure has no competition rows.')
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Event rows</span>
              <h2>Events returned by the structure</h2>
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
                          name: event.name,
                          date: event.date,
                          location: event.location,
                          locationTypes: (event.location_types || []).join(','),
                          competitionId: event.competitionId,
                          competitionName: event.competitionName,
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
              : '<p class="card-note">No events in this institution structure.</p>'
          }
        </article>
      </section>
    `,
  });
}

function renderTeamStructureResult(team) {
  if (!team) {
    return renderEmptyState('Team not found', 'The selected team structure is unavailable.');
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
    eyebrow: 'Team structure',
    title: safeTeam.name,
    actionHref: `/teams/${safeTeam.id}`,
    actionLabel: 'Open team',
    body: `
      ${renderStatGrid([
        { label: 'Competitions', value: formatNumber(overview.competitionCount), hint: 'Portfolio rows' },
        { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Tracked results' },
        { label: 'Members', value: formatNumber(overview.totalMembers), hint: 'Across competitions' },
        { label: 'Women tracked', value: formatNumber(overview.femaleParticipants), hint: 'Absolute count' },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition portfolio</span>
              <h2>Competitions returned for this team</h2>
            </div>
          </div>
          ${
            safeTeam.competitions.length
              ? renderTable({
                  compact: true,
                  columns: [
                    { label: 'Competition' },
                    { label: 'Seasons' },
                    { label: 'Events', align: 'right' },
                    { label: 'Members', align: 'right' },
                    { label: 'Women', align: 'right' },
                  ],
                  rows: safeTeam.competitions.map((competition) => [
                    {
                      value: `<a href="/teams/${safeTeam.id}${serialiseQuery({
                        competition: competition.id,
                      })}" data-link><strong>${escapeHtml(competition.name)}</strong></a>`,
                    },
                    { value: escapeHtml(structureYearsLabel(competitionYears(competition))) },
                    { value: formatNumber((competition.events || []).length), align: 'right' },
                    { value: formatNumber(competition.total_members), align: 'right' },
                    { value: formatNumber(competition.female_participants), align: 'right' },
                  ]),
                })
              : renderEmptyState('No competitions', 'This team structure has no competition portfolio.')
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Event rows</span>
              <h2>Results returned by the team structure</h2>
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
                          name: event.name,
                          date: event.date,
                          location: event.location,
                          locationTypes: (event.location_types || []).join(','),
                          competitionId: event.competitionId,
                          competitionName: event.competitionName,
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
              : '<p class="card-note">No events in this team structure.</p>'
          }
        </article>
      </section>
    `,
  });
}

function renderSelectedStructureResult({
  selectedOrganizerRecord,
  selectedCompetitionRecord,
  selectedInstitutionRecord,
  selectedTeamRecord,
}) {
  if (selectedTeamRecord) {
    return renderTeamStructureResult(selectedTeamRecord);
  }

  if (selectedInstitutionRecord) {
    return renderInstitutionStructureResult(selectedInstitutionRecord);
  }

  if (selectedCompetitionRecord) {
    return renderCompetitionStructureResult(selectedCompetitionRecord);
  }

  if (selectedOrganizerRecord) {
    return renderOrganizerStructureResult(selectedOrganizerRecord);
  }

  return renderEmptyState('No structure selected', 'Choose a filter and apply it to load a structure.');
}

function syncCustomDropdown(dropdown) {
  const select = dropdown.querySelector('select');
  const trigger = dropdown.querySelector('[data-custom-select-trigger]');
  const label = dropdown.querySelector('[data-custom-select-label]');
  const options = Array.from(dropdown.querySelectorAll('[data-custom-select-option]'));
  const selectedOption = select.selectedOptions[0];

  label.textContent = selectedOption?.textContent || '';
  trigger.disabled = select.disabled;
  dropdown.classList.toggle('is-disabled', select.disabled);
  dropdown.closest('.filter-step')?.classList.toggle('is-disabled', select.disabled);

  options.forEach((option) => {
    const selected = option.dataset.value === select.value;
    option.classList.toggle('is-selected', selected);
    option.setAttribute('aria-selected', String(selected));
  });
}

function setDropdownOptions(select, { options, selectedValue = '', placeholder, disabled = false }) {
  const dropdown = select.closest('[data-custom-select]');
  const menu = dropdown.querySelector('[data-custom-select-menu]');
  const normalizedValue = selectedOptionId(options, queryId(selectedValue)) || '';

  select.innerHTML = optionMarkup(options, normalizedValue, placeholder);
  select.disabled = disabled;
  select.value = String(normalizedValue);
  menu.innerHTML = dropdownOptionMarkup(optionItems(options, placeholder), normalizedValue);

  syncCustomDropdown(dropdown);
}

function initCustomDropdowns(form, signal) {
  const dropdowns = Array.from(form.querySelectorAll('[data-custom-select]'));

  const closeDropdown = (dropdown) => {
    const menu = dropdown.querySelector('[data-custom-select-menu]');
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');

    menu.hidden = true;
    trigger.setAttribute('aria-expanded', 'false');
    dropdown.classList.remove('is-open');
  };

  const closeAll = (except = null) => {
    dropdowns.forEach((dropdown) => {
      if (dropdown !== except) {
        closeDropdown(dropdown);
      }
    });
  };

  const focusOption = (dropdown, target = 'selected') => {
    const options = Array.from(dropdown.querySelectorAll('[data-custom-select-option]'));
    const selected = options.find((option) => option.classList.contains('is-selected'));

    if (target === 'last') {
      options.at(-1)?.focus();
      return;
    }

    (target === 'first' ? options[0] : selected || options[0])?.focus();
  };

  const openDropdown = (dropdown, focusTarget = null) => {
    const select = dropdown.querySelector('select');
    const menu = dropdown.querySelector('[data-custom-select-menu]');
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');

    if (select.disabled) {
      return;
    }

    closeAll(dropdown);
    menu.hidden = false;
    trigger.setAttribute('aria-expanded', 'true');
    dropdown.classList.add('is-open');

    if (focusTarget) {
      focusOption(dropdown, focusTarget);
    }
  };

  dropdowns.forEach((dropdown) => {
    const select = dropdown.querySelector('select');
    const trigger = dropdown.querySelector('[data-custom-select-trigger]');
    const menu = dropdown.querySelector('[data-custom-select-menu]');

    syncCustomDropdown(dropdown);

    trigger.addEventListener('click', () => {
      if (menu.hidden) {
        openDropdown(dropdown);
      } else {
        closeDropdown(dropdown);
      }
    });

    trigger.addEventListener('keydown', (event) => {
      if (!['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
        return;
      }

      event.preventDefault();
      openDropdown(dropdown, event.key === 'End' ? 'last' : 'selected');
    });

    select.addEventListener('change', () => {
      syncCustomDropdown(dropdown);
    });

    menu.addEventListener('click', (event) => {
      const option = event.target.closest('[data-custom-select-option]');

      if (!option) {
        return;
      }

      select.value = option.dataset.value;
      select.dispatchEvent(new Event('change', { bubbles: true }));
      syncCustomDropdown(dropdown);
      closeDropdown(dropdown);
      trigger.focus();
    });

    menu.addEventListener('keydown', (event) => {
      const option = event.target.closest('[data-custom-select-option]');

      if (event.key === 'Escape') {
        event.preventDefault();
        closeDropdown(dropdown);
        trigger.focus();
        return;
      }

      if (!option) {
        return;
      }

      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        option.click();
        return;
      }

      if (!['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
        return;
      }

      event.preventDefault();
      const options = Array.from(dropdown.querySelectorAll('[data-custom-select-option]'));
      const currentIndex = options.indexOf(option);
      let nextIndex = currentIndex;

      if (event.key === 'ArrowDown') nextIndex = currentIndex + 1;
      if (event.key === 'ArrowUp') nextIndex = currentIndex - 1;
      if (event.key === 'Home') nextIndex = 0;
      if (event.key === 'End') nextIndex = options.length - 1;

      options[Math.max(0, Math.min(nextIndex, options.length - 1))]?.focus();
    });
  });

  document.addEventListener(
    'click',
    (event) => {
      if (!event.target.closest('[data-custom-select]')) {
        closeAll();
      }
    },
    { signal },
  );

  document.addEventListener(
    'keydown',
    (event) => {
      if (event.key === 'Escape') {
        closeAll();
      }
    },
    { signal },
  );
}

export async function render({ query, navigate }) {
  const universe = await getUniverseSnapshot();
  const organizerOptions = universe.organizerOptions;
  const competitionOptionsByOrganizer = await loadCompetitionOptionsByOrganizer(organizerOptions);

  const selectedOrganizer = selectedOptionId(organizerOptions, queryId(query.organizer));
  const competitionOptions = selectedOrganizer
    ? competitionOptionsForOrganizer(universe, selectedOrganizer, competitionOptionsByOrganizer)
    : [];
  const selectedCompetition = selectedOptionId(competitionOptions, queryId(query.competition));
  const selectedOrganizerRecord = itemById(universe.organizers, selectedOrganizer);
  const selectedCompetitionRecord = itemById(universe.competitions, selectedCompetition);
  const institutionOptions = selectedCompetition
    ? institutionOptionsForCompetition(universe, selectedCompetition)
    : [];
  const selectedInstitution = selectedOptionId(institutionOptions, queryId(query.institution));
  const selectedInstitutionRecord = itemById(universe.institutions, selectedInstitution);
  const teamOptions =
    selectedCompetition && selectedInstitution
      ? teamOptionsForCompetitionAndInstitution(universe, selectedCompetition, selectedInstitutionRecord)
      : [];
  const selectedTeam = selectedOptionId(teamOptions, queryId(query.team));
  const selectedTeamRecord = itemById(universe.teams, selectedTeam);
  const competitionOptionIds = selectedOrganizer ? optionIdSet(competitionOptions) : null;
  const institutionCompetitionIds = competitionIdsForStructure(selectedInstitutionRecord);
  const teamCompetitionIds = competitionIdsForStructure(selectedTeamRecord);
  const hasActiveFilters = Boolean(
    selectedOrganizer || selectedCompetition || selectedInstitution || selectedTeam,
  );

  const scopedCompetitions = universe.competitions
    .filter((competition) => {
      if (selectedOrganizer && !competitionOptionIds.has(competition.id)) {
        return false;
      }

      if (selectedCompetition && competition.id !== selectedCompetition) {
        return false;
      }

      if (selectedInstitution && !institutionCompetitionIds.has(competition.id)) {
        return false;
      }

      if (selectedTeam && !teamCompetitionIds.has(competition.id)) {
        return false;
      }

      return true;
    })
    .map((competition) => filterCompetitionEntries(competition, selectedInstitutionRecord, selectedTeam))
    .filter((competition) => !hasActiveFilters || competition.events.length);

  const scopedCompetitionIds = optionIdSet(scopedCompetitions);
  const competitionScope = hasActiveFilters ? scopedCompetitionIds : null;

  const competitions = scopedCompetitions
    .map((competition) => ({ competition, overview: getCompetitionOverview(competition) }))
    .sort((left, right) => right.overview.uniqueTeams - left.overview.uniqueTeams);

  const scopedInstitutions = universe.institutions
    .filter((institution) => !selectedInstitution || institution.id === selectedInstitution)
    .map((institution) => filterInstitutionEntries(institution, competitionScope, selectedTeam))
    .filter((institution) => !hasActiveFilters || institution.competitions.length);

  const institutions = scopedInstitutions
    .map((institution) => ({ institution, overview: getInstitutionOverview(institution) }))
    .sort((left, right) => right.overview.teamEntries - left.overview.teamEntries)
    .slice(0, 8);

  const organizers = universe.organizers
    .filter((organizer) => !selectedOrganizer || organizer.id === selectedOrganizer)
    .map((organizer) => filterOrganizerEntries(organizer, competitionScope))
    .filter((organizer) => !hasActiveFilters || organizer.competitions.length)
    .map((organizer) => ({ organizer, overview: getOrganizerOverview(organizer) }))
    .sort((left, right) => right.overview.totalParticipants - left.overview.totalParticipants)
    .slice(0, 6);

  const scopedEvents = competitionEvents(scopedCompetitions);
  const latestEvents = scopedEvents.slice(0, 8);
  const heroCompetition = competitions[0]?.competition;
  const activeFilterChips = [
    selectedOrganizer ? `Organizer: ${optionLabel(organizerOptions, selectedOrganizer)}` : null,
    selectedCompetition ? `Competition: ${optionLabel(competitionOptions, selectedCompetition)}` : null,
    selectedInstitution ? `Institution: ${optionLabel(institutionOptions, selectedInstitution)}` : null,
    selectedTeam ? `Team: ${optionLabel(teamOptions, selectedTeam)}` : null,
  ]
    .filter(Boolean)
    .map((label) => renderChip(label, 'slate'));
  const activeStructureSummary = selectedTeam
    ? `Showing Team structure for ${optionLabel(teamOptions, selectedTeam)}.`
    : selectedInstitution
      ? `Showing Institution structure for ${optionLabel(institutionOptions, selectedInstitution)}.`
      : selectedCompetition
        ? `Showing Competition structure for ${optionLabel(competitionOptions, selectedCompetition)}.`
        : selectedOrganizer
          ? `Showing Organizer structure for ${optionLabel(organizerOptions, selectedOrganizer)}.`
          : null;

  const html = `
    ${renderPageIntro({
      eyebrow: 'Competition intelligence desk',
      title: 'Track programming ecosystems like a transfer market.',
      blurb:
        'Browse competition ladders, team portfolios, institution pipelines and organizer footprints through one connected scouting view.',
      actions: `
        <div class="action-stack">
          <a class="button" href="/competitions" data-link>Scout competitions</a>
          <a class="button button--ghost" href="/teams" data-link>Explore teams</a>
        </div>
      `,
      meta: [
        renderChip(`${universe.organizers.length} organizers`, 'navy'),
        renderChip(`${universe.competitions.length} competitions`, 'navy'),
        renderChip(`${universe.events.length} events`, 'navy'),
      ],
    })}

    <section class="panel panel--filters" aria-label="Entity filters">
      <form class="filter-bar filter-bar--cascade" id="home-cascade-filters">
        ${renderDropdown({
          name: 'organizer',
          label: 'Organizer',
          options: organizerOptions,
          selectedValue: selectedOrganizer,
          placeholder: 'Select organizer',
        })}
        ${renderDropdown({
          name: 'competition',
          label: 'Competition',
          options: competitionOptions,
          selectedValue: selectedCompetition,
          placeholder: selectedOrganizer ? 'Select competition' : 'Select organizer first',
          disabled: !selectedOrganizer,
        })}
        ${renderDropdown({
          name: 'institution',
          label: 'Institution',
          options: institutionOptions,
          selectedValue: selectedInstitution,
          placeholder: selectedCompetition ? 'Select institution' : 'Select competition first',
          disabled: !selectedCompetition,
        })}
        ${renderDropdown({
          name: 'team',
          label: 'Team',
          options: teamOptions,
          selectedValue: selectedTeam,
          placeholder: selectedInstitution ? 'Select team' : 'Select institution first',
          disabled: !selectedInstitution,
        })}
        <button class="button" type="submit">Apply Filters</button>
        <a class="button button--ghost" href="/" data-link>Clear</a>
      </form>
      <div class="filter-summary">
        <p>
          ${
            hasActiveFilters
              ? activeStructureSummary
              : `${formatNumber(universe.competitions.length)} competitions, ${formatNumber(universe.events.length)} events and ${formatNumber(universe.teams.length)} teams in the full dataset.`
          }
        </p>
        <div class="filter-summary__chips">
          ${activeFilterChips.length ? activeFilterChips.join('') : renderChip('Full dataset', 'navy')}
        </div>
      </div>
    </section>

    ${
      hasActiveFilters
        ? renderSelectedStructureResult({
            selectedOrganizerRecord,
            selectedCompetitionRecord,
            selectedInstitutionRecord,
            selectedTeamRecord,
          })
        : ''
    }

    <div class="home-dashboard" ${hasActiveFilters ? 'hidden' : ''}>
    <section class="hero-grid">
      <article class="hero-card hero-card--feature">
        <span class="eyebrow">Market pulse</span>
        <h2>${escapeHtml(heroCompetition?.name || 'Competition map')}</h2>
        <p>
          ${escapeHtml(
            heroCompetition
              ? 'The most connected competition in the current dataset anchors the season board below.'
              : 'Start with any competition, team, institution or organizer from the navigation above.',
          )}
        </p>
        ${
          heroCompetition
            ? renderMetricStrip([
                { label: 'Seasons', value: getCompetitionOverview(heroCompetition).yearSpan },
                { label: 'Events', value: formatNumber(getCompetitionOverview(heroCompetition).eventCount) },
                {
                  label: 'Unique teams',
                  value: formatNumber(getCompetitionOverview(heroCompetition).uniqueTeams),
                },
                {
                  label: 'Women tracked',
                  value: formatNumber(getCompetitionOverview(heroCompetition).femaleParticipants),
                },
              ])
            : ''
        }
        ${heroCompetition ? `<a class="button" href="/competitions/${heroCompetition.id}" data-link>Open scouting file</a>` : ''}
      </article>

      <article class="hero-card">
        <span class="eyebrow">Universe snapshot</span>
        <h3>${hasActiveFilters ? 'Selected scope, live snapshot.' : 'One board, multiple lenses.'}</h3>
        ${renderStatGrid([
          {
            label: 'Competitions',
            value: formatNumber(scopedCompetitions.length),
            hint: 'Scouted calendars',
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
          { label: 'Events', value: formatNumber(scopedEvents.length), hint: 'Historical fixtures' },
        ])}
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Spotlight competitions</span>
          <h2>${hasActiveFilters ? 'Competition networks in the selected scope' : 'Where the biggest season networks live'}</h2>
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
          : renderEmptyState(
              'No competitions in this scope',
              'This filter combination has no tracked competition results.',
            )
      }
    </section>

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Institutions board</span>
            <h2>Top academic pipelines by tracked entries</h2>
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
                  { label: 'Women', align: 'right' },
                ],
                rows: institutions.map(({ institution, overview }) => [
                  {
                    value: `<a href="/institutions/${institution.id}" data-link><strong>${escapeHtml(institution.short_name || institution.name)}</strong></a><small>${escapeHtml(institution.name)}</small>`,
                  },
                  { value: escapeHtml(institution.location) },
                  { value: formatNumber(overview.teamEntries), align: 'right' },
                  { value: formatNumber(overview.femaleParticipants), align: 'right' },
                ]),
              })
            : '<p class="card-note">No institutions in this scope.</p>'
        }
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Latest fixtures</span>
            <h2>Recent event files</h2>
          </div>
        </div>
        ${
          latestEvents.length
            ? `
          <div class="timeline-list">
            ${latestEvents
              .map(
                (event) => `
                <a class="timeline-item" href="/events/${event.id}${serialiseQuery({ year: event.year, name: event.name, date: event.date, location: event.location, locationTypes: event.locationTypes.join(','), competitionId: event.competitionId, competitionName: event.competitionName })}" data-link>
                  <span>${escapeHtml(formatDate(event.date))}</span>
                  <strong>${escapeHtml(event.name)}</strong>
                  <small>${escapeHtml(event.competitionName)} · ${escapeHtml(event.location)}</small>
                </a>
              `,
              )
              .join('')}
          </div>
        `
            : '<p class="card-note">No recent event files in this scope.</p>'
        }
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Organizer reach</span>
          <h2>Which networks currently move the most volume</h2>
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
                  { label: 'Entries', value: formatNumber(overview.totalTeams) },
                  { label: 'Women', value: formatNumber(overview.femaleParticipants) },
                ])}
                <p class="card-note">Location tiers: ${escapeHtml(overview.locationTypes.join(', ') || '—')}</p>
              </article>
            `,
                )
                .join('')
            : '<p class="card-note">No organizers in this scope.</p>'
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
      initCustomDropdowns(form, dropdownAbortController.signal);

      const readState = () => ({
        organizer: organizerSelect.value,
        competition: competitionSelect.value,
        institution: institutionSelect.value,
        team: teamSelect.value,
      });

      const refreshCascade = (changedName) => {
        const organizer = queryId(organizerSelect.value);
        const nextCompetitionOptions = organizer
          ? competitionOptionsForOrganizer(universe, organizer, competitionOptionsByOrganizer)
          : [];
        const competition =
          changedName === 'organizer'
            ? null
            : selectedOptionId(nextCompetitionOptions, queryId(competitionSelect.value));
        const nextInstitutionOptions = competition
          ? institutionOptionsForCompetition(universe, competition)
          : [];
        const institution = ['organizer', 'competition'].includes(changedName)
          ? null
          : selectedOptionId(nextInstitutionOptions, queryId(institutionSelect.value));
        const institutionRecord = itemById(universe.institutions, institution);
        const nextTeamOptions =
          competition && institution
            ? teamOptionsForCompetitionAndInstitution(universe, competition, institutionRecord)
            : [];
        const team =
          changedName === 'team' ? selectedOptionId(nextTeamOptions, queryId(teamSelect.value)) : null;

        setDropdownOptions(competitionSelect, {
          options: nextCompetitionOptions,
          selectedValue: competition,
          placeholder: organizer ? 'Select competition' : 'Select organizer first',
          disabled: !organizer,
        });
        setDropdownOptions(institutionSelect, {
          options: nextInstitutionOptions,
          selectedValue: institution,
          placeholder: competition ? 'Select institution' : 'Select competition first',
          disabled: !competition,
        });
        setDropdownOptions(teamSelect, {
          options: nextTeamOptions,
          selectedValue: team,
          placeholder: institution ? 'Select team' : 'Select institution first',
          disabled: !institution,
        });
      };

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        navigate(`/${serialiseQuery(readState())}`);
      });

      form.addEventListener('change', (event) => {
        if (event.target.matches('select')) {
          refreshCascade(event.target.name);
        }
      });
    },
  };
}

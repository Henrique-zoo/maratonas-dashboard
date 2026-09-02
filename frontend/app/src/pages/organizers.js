import {
  getCompetitionLocationStats,
  getCompetitionStats,
  getEventLocationStats,
  getEventStats,
  getOrganizerCompetitionYearStructure,
  getOrganizerOptions,
  getOrganizerStructures,
} from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import { getOrganizerOverview, latestYear } from '../lib/metrics.js';
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
  serialiseQuery,
} from '../lib/ui.js';

let dropdownAbortController = null;

function numericQuery(value) {
  const number = Number(value);
  return Number.isInteger(number) && number > 0 ? number : null;
}

function eventYear(event) {
  return event?.date ? new Date(`${event.date}T00:00:00`).getFullYear() : null;
}

function sortedYears(years = []) {
  return Array.from(new Set(years.map(Number).filter(Boolean))).sort((left, right) => right - left);
}

function selectedYearForCompetition(competition, requestedYear) {
  const years = sortedYears(competition?.years || []);
  const requested = Number(requestedYear);

  return years.includes(requested) ? requested : latestYear(years);
}

function locationTypesForEvent(event, fallback = []) {
  const eventTypes = event?.location_types || event?.locationTypes || [];
  return eventTypes.length ? eventTypes : fallback;
}

function locationTypesForCompetition(competition, seasonStructure = null) {
  if (seasonStructure?.location_types?.length) {
    return seasonStructure.location_types;
  }

  if (competition?.location_types?.length) {
    return competition.location_types;
  }

  return Array.from(new Set((competition?.events || []).flatMap((event) => locationTypesForEvent(event))));
}

function chooseSelectedLocationType(locationTypes = [], requestedLocationType = null) {
  if (requestedLocationType && locationTypes.includes(requestedLocationType)) {
    return requestedLocationType;
  }

  return chooseLocationType(locationTypes);
}

function organizerIndexHref(params = {}) {
  return `/organizers${serialiseQuery(params)}`;
}

function optionYears(years = []) {
  return sortedYears(years).map((year) => ({
    id: String(year),
    name: String(year),
  }));
}

function selectedCompetitionFrom(organizers, selectedCompetitionId) {
  return organizers
    .flatMap((organizer) => organizer.competitions || [])
    .find((competition) => competition.id === selectedCompetitionId);
}

function replaceSelectedCompetitionSeason(organizers, selectedCompetitionId, seasonStructure) {
  if (!seasonStructure) {
    return organizers;
  }

  return organizers.map((organizer) => ({
    ...organizer,
    competitions: (organizer.competitions || []).map((competition) =>
      competition.id === selectedCompetitionId
        ? {
            ...competition,
            events: seasonStructure.events || [],
            location_types: seasonStructure.location_types || competition.location_types || [],
          }
        : competition,
    ),
  }));
}

function findCompetition(organizers, competitionId) {
  return organizers
    .flatMap((organizer) => organizer.competitions || [])
    .find((competition) => competition.id === competitionId);
}

function findEvent(competition, eventId) {
  return (competition?.events || []).find((event) => event.id === eventId);
}

function eventStatsRows(events = []) {
  return events.reduce(
    (total, event) => ({
      teams: total.teams + Number(event.total_teams || 0),
      participantEntries: total.participantEntries + Number(event.total_participants || 0),
      femaleParticipantEntries: total.femaleParticipantEntries + Number(event.female_participants || 0),
    }),
    { teams: 0, participantEntries: 0, femaleParticipantEntries: 0 },
  );
}

function renderEventButtons({ competition, events, displayYear, selectedEventId, query }) {
  if (!events.length) {
    return '<p class="card-note">No events this year.</p>';
  }

  return `
    <div class="organizer-event-list">
      ${events
        .map((event) => {
          const eventLocationTypes = locationTypesForEvent(event, competition.location_types || []);
          return `
            <a
              class="organizer-event-button ${event.id === selectedEventId ? 'is-selected' : ''}"
              href="${organizerIndexHref({
                q: query.q,
                competition: competition.id,
                event: event.id,
                year: displayYear || eventYear(event),
                locationType: chooseSelectedLocationType(eventLocationTypes, null),
              })}"
              data-link
            >
              <span>${escapeHtml(formatDate(event.date))}</span>
              <strong>${escapeHtml(event.name)}</strong>
              <small>${formatNumber(event.total_teams)} unique teams · ${escapeHtml(event.location || '-')}</small>
            </a>
          `;
        })
        .join('')}
    </div>
  `;
}

function renderCompetitionCard({
  competition,
  events,
  displayYear,
  selectedCompetitionId,
  selectedEventId,
  query,
}) {
  const selected = competition.id === selectedCompetitionId;
  const totals = eventStatsRows(events);
  const locationTypes = locationTypesForCompetition({ ...competition, events });
  const selectHref = organizerIndexHref({
    q: query.q,
    competition: competition.id,
    year: displayYear,
    locationType: chooseSelectedLocationType(locationTypes, null),
  });

  return `
    <article
      class="organizer-competition-card ${selected ? 'is-selected' : ''}"
      data-organizer-competition-card
      data-select-href="${escapeHtml(selectHref)}"
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
        <form class="organizer-competition-year-form" data-competition-year-form>
          <input type="hidden" name="competition" value="${competition.id}" />
          <input type="hidden" name="q" value="${escapeHtml(query.q || '')}" />
          ${renderDropdown({
            name: 'year',
            label: 'Year',
            options: optionYears(competition.years),
            selectedValue: displayYear,
            placeholder: 'Year',
            disabled: !competition.years?.length,
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

      ${renderEventButtons({
        competition,
        events,
        displayYear,
        selectedEventId: selected ? selectedEventId : null,
        query,
      })}
    </article>
  `;
}

function renderOrganizerCard({
  organizer,
  overview,
  selectedCompetitionId,
  selectedEventId,
  selectedCompetitionYear,
  query,
}) {
  return `
    <article class="organizer-structure-card">
      <div class="list-card__title">
        <div>
          <h2><a href="/organizers/${organizer.id}" data-link>${escapeHtml(organizer.name)}</a></h2>
          <p>${escapeHtml(String(overview.competitionCount))} competitions · latest season snapshot each</p>
        </div>
      </div>

      ${renderMetricStrip([
        { label: 'Snapshot events', value: formatNumber(overview.eventCount) },
        { label: 'Team entries', value: formatNumber(overview.teamEntries) },
        {
          label: 'Female entries',
          value: formatNumber(overview.femaleParticipantEntries),
        },
      ])}

      <div class="organizer-competition-scroll" aria-label="${escapeHtml(`${organizer.name} competitions`)}">
        ${(organizer.competitions || [])
          .map((competition) =>
            renderCompetitionCard({
              competition,
              events: competition.events || [],
              displayYear:
                competition.id === selectedCompetitionId
                  ? selectedCompetitionYear
                  : latestYear(competition.years || []),
              selectedCompetitionId,
              selectedEventId,
              query,
            }),
          )
          .join('')}
      </div>
    </article>
  `;
}

function renderInspector({ target, stats, locationStats, locationChoices, selectedLocationType, query }) {
  if (!target) {
    return '';
  }

  return `
    <aside class="organizer-inspector">
      <details class="organizer-inspector__drawer" open>
        <summary>
          <span>${escapeHtml(target.kind)} filters</span>
          <strong>${escapeHtml(selectedLocationType || 'Location')}</strong>
        </summary>

        <div class="section-head">
          <div>
            <span class="eyebrow">${escapeHtml(target.kind)} details</span>
            <h2>${escapeHtml(target.name)}</h2>
            <p>${escapeHtml(`Year ${target.year}`)}</p>
          </div>
        </div>

        <form class="organizer-inspector__form" id="organizer-location-filter">
          <input type="hidden" name="q" value="${escapeHtml(query.q || '')}" />
          <input type="hidden" name="competition" value="${target.competitionId}" />
          ${target.eventId ? `<input type="hidden" name="event" value="${target.eventId}" />` : ''}
          <input type="hidden" name="year" value="${target.year}" />
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

        <div class="organizer-inspector__split">
          <span class="eyebrow">${escapeHtml(selectedLocationType)} split</span>
          ${renderBarList(
            locationStats.map((item) => ({
              label: item.name,
              value: item.total_teams,
              subtitle: `${formatNumber(item.total_participants)} participants`,
            })),
            { valueFormatter: formatNumber },
          )}
        </div>
      </details>
    </aside>
  `;
}

export async function render({ query, navigate }) {
  const searchTerm = (query.q || '').trim().toLowerCase();
  const selectedCompetitionId = numericQuery(query.competition);
  const selectedEventId = numericQuery(query.event);
  const organizerOptions = await getOrganizerOptions();
  const organizers = organizerOptions.length
    ? await getOrganizerStructures(organizerOptions.map((item) => item.id))
    : [];

  const selectedCompetitionBase = selectedCompetitionFrom(organizers, selectedCompetitionId);
  const selectedCompetitionYear = selectedCompetitionBase
    ? selectedYearForCompetition(selectedCompetitionBase, query.year)
    : null;
  const selectedCompetitionLatestYear = selectedCompetitionBase
    ? latestYear(selectedCompetitionBase.years || [])
    : null;
  const selectedYearStructure =
    selectedCompetitionBase &&
    selectedCompetitionYear &&
    selectedCompetitionYear !== selectedCompetitionLatestYear
      ? await getOrganizerCompetitionYearStructure(selectedCompetitionBase.id, selectedCompetitionYear)
      : null;

  const displayOrganizers = replaceSelectedCompetitionSeason(
    organizers,
    selectedCompetitionId,
    selectedYearStructure,
  );
  const selectedCompetition = findCompetition(displayOrganizers, selectedCompetitionId);
  const selectedEvent = findEvent(selectedCompetition, selectedEventId);
  const displayOrganizerById = new Map(displayOrganizers.map((organizer) => [organizer.id, organizer]));
  const visibleOrganizers = organizers
    .map((organizer) => ({
      organizer: displayOrganizerById.get(organizer.id) || organizer,
      overview: getOrganizerOverview(organizer),
    }))
    .filter(({ organizer }) => {
      if (!searchTerm) {
        return true;
      }

      return organizer.name.toLowerCase().includes(searchTerm);
    })
    .sort((left, right) => left.organizer.name.localeCompare(right.organizer.name));

  let inspector = {
    target: null,
    stats: null,
    locationStats: [],
    locationChoices: [],
    selectedLocationType: null,
  };

  if (selectedCompetition) {
    const targetYear = selectedCompetitionYear || latestYear(selectedCompetition.years || []);
    const locationChoices = selectedEvent
      ? locationTypesForEvent(selectedEvent, selectedCompetition.location_types || [])
      : locationTypesForCompetition(selectedCompetition, selectedYearStructure);
    const selectedLocationType = chooseSelectedLocationType(locationChoices, query.locationType);

    if (selectedEvent) {
      const [stats, locationStats] = await Promise.all([
        getEventStats(selectedEvent.id, targetYear),
        getEventLocationStats(selectedEvent.id, selectedLocationType, targetYear),
      ]);

      inspector = {
        target: {
          kind: 'Event',
          name: selectedEvent.name,
          competitionId: selectedCompetition.id,
          eventId: selectedEvent.id,
          year: targetYear,
        },
        stats,
        locationStats,
        locationChoices,
        selectedLocationType,
      };
    } else {
      const [stats, locationStats] = await Promise.all([
        getCompetitionStats(selectedCompetition.id, targetYear),
        getCompetitionLocationStats(selectedCompetition.id, selectedLocationType, targetYear),
      ]);

      inspector = {
        target: {
          kind: 'Competition',
          name: selectedCompetition.name,
          competitionId: selectedCompetition.id,
          year: targetYear,
        },
        stats,
        locationStats,
        locationChoices,
        selectedLocationType,
      };
    }
  }

  const html = `
    ${renderPageIntro({
      eyebrow: 'Organizer market',
      title: 'Organizers',
      blurb: 'Browse annual competition details without mixing them into portfolio-wide totals.',
      meta: [renderChip(`${visibleOrganizers.length} visible organizers`, 'amber')],
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="organizer-filters">
        <label class="filter-bar__grow">
          <span>Search</span>
          <input name="q" type="search" value="${escapeHtml(query.q || '')}" placeholder="Organizer name" />
        </label>
        <button class="button" type="submit">Search</button>
      </form>
    </section>

    ${
      visibleOrganizers.length
        ? `
          <section class="organizer-index-layout ${inspector.target ? 'has-inspector' : ''}">
            <div class="organizer-index-layout__main">
              ${visibleOrganizers
                .map(({ organizer, overview }) =>
                  renderOrganizerCard({
                    organizer,
                    overview,
                    selectedCompetitionId,
                    selectedEventId,
                    selectedCompetitionYear,
                    query,
                  }),
                )
                .join('')}
            </div>
            ${renderInspector({ ...inspector, query })}
          </section>
        `
        : renderEmptyState(
            'No organizers match your search',
            'Try a shorter search phrase or clear the search input.',
          )
    }
  `;

  return {
    title: 'Organizers',
    html,
    afterRender() {
      const form = document.getElementById('organizer-filters');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);
      initOrganizerCompetitionCards(document, navigate);

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        const search = data.get('q')?.toString().trim();
        navigate(search ? organizerIndexHref({ q: search }) : '/organizers');
      });

      document.querySelectorAll('[data-competition-year-form]').forEach((yearForm) => {
        yearForm.addEventListener('change', (event) => {
          if (!event.target.matches('select')) {
            return;
          }

          const data = new FormData(yearForm);
          navigate(
            organizerIndexHref({
              q: data.get('q')?.toString().trim(),
              competition: data.get('competition'),
              year: data.get('year'),
            }),
          );
        });
      });

      document.getElementById('organizer-location-filter')?.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);
        navigate(
          organizerIndexHref({
            q: data.get('q')?.toString().trim(),
            competition: data.get('competition'),
            event: data.get('event'),
            year: data.get('year'),
            locationType: data.get('locationType'),
          }),
        );
      });

      document.getElementById('organizer-location-filter')?.addEventListener('change', (event) => {
        if (event.target.matches('select')) {
          event.currentTarget.requestSubmit();
        }
      });
    },
  };
}

function initOrganizerCompetitionCards(root, navigate) {
  root.querySelectorAll('[data-organizer-competition-card]').forEach((card) => {
    const selectCard = () => {
      if (card.dataset.selectHref) {
        navigate(card.dataset.selectHref);
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
}

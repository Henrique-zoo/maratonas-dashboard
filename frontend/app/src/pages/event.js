import { getEventLocationStats, getEventStats, getEventStructure } from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import {
  escapeHtml,
  formatDate,
  formatNumber,
  renderBarList,
  renderChip,
  renderMetricStrip,
  renderPageIntro,
  renderStatGrid,
  serialiseQuery,
} from '../lib/ui.js';

let dropdownAbortController = null;

function preferredLocationType(locationTypes = [], requested = null) {
  if (requested && locationTypes.includes(requested)) {
    return requested;
  }

  return locationTypes.includes('Country') ? 'Country' : locationTypes[0] || null;
}

function occurrenceSummary(instances = []) {
  if (!instances.length) {
    return 'No registered occurrence';
  }

  if (instances.length === 1) {
    return `${formatDate(instances[0].date)} · ${instances[0].location}`;
  }

  return `${instances.length} registered occurrences`;
}

export async function render({ params, query, navigate }) {
  const eventId = Number(params.id);
  const requestedYear = query.year ? Number(query.year) : null;
  const structure = await getEventStructure(eventId, requestedYear);
  const year = structure.year;
  const locationTypes = structure.location_types || [];
  const selectedLocationType = preferredLocationType(locationTypes, query.locationType);

  const [stats, locationStats] = await Promise.all([
    getEventStats(eventId, year),
    selectedLocationType ? getEventLocationStats(eventId, selectedLocationType, year) : Promise.resolve([]),
  ]);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Event',
      title: structure.name,
      blurb: 'Review the annual occurrences and participation aggregates for this event.',
      meta: [renderChip(`Year ${year}`, 'amber'), renderChip(occurrenceSummary(structure.instances), 'navy')],
      actions: `<a class="button button--ghost" href="/competitions/${structure.competition.id}${serialiseQuery({ year })}" data-link>${escapeHtml(structure.competition.name)}</a>`,
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="event-detail-filters">
        ${renderDropdown({
          name: 'year',
          label: 'Year',
          options: [...structure.years]
            .sort((left, right) => right - left)
            .map((availableYear) => ({
              id: String(availableYear),
              name: String(availableYear),
            })),
          selectedValue: year,
          placeholder: 'Select year',
        })}
        ${renderDropdown({
          name: 'locationType',
          label: 'Location level',
          options: locationTypes.map((locationType) => ({
            id: locationType,
            name: locationType,
          })),
          selectedValue: selectedLocationType,
          placeholder: 'No location breakdown',
          disabled: !locationTypes.length,
        })}
        <button class="button" type="submit">Apply</button>
      </form>
    </section>

    ${renderStatGrid([
      {
        label: 'Institutions',
        value: formatNumber(stats.total_institutions),
        hint: 'Distinct in this year',
      },
      {
        label: 'Teams',
        value: formatNumber(stats.total_teams),
        hint: 'Distinct in this year',
      },
      {
        label: 'Participants',
        value: formatNumber(stats.total_participants),
        hint: `Distinct people · year ${year}`,
      },
      {
        label: 'Female participants',
        value: formatNumber(stats.female_participants),
        hint: 'Distinct people',
      },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Location spread</span>
            <h2>${escapeHtml(selectedLocationType || 'No')} distribution</h2>
          </div>
        </div>
        ${
          selectedLocationType
            ? renderBarList(
                locationStats.map((item) => ({
                  label: item.name,
                  value: item.total_teams,
                  subtitle: `${formatNumber(item.total_participants)} participants`,
                })),
                { valueFormatter: formatNumber },
              )
            : '<p class="card-note">No participant location level is available for this event.</p>'
        }
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Annual occurrences</span>
            <h2>${escapeHtml(structure.name)} · ${year}</h2>
          </div>
        </div>
        ${renderMetricStrip([
          {
            label: 'Occurrences',
            value: formatNumber(structure.instances.length),
          },
          {
            label: 'Participants',
            value: formatNumber(stats.total_participants),
          },
          {
            label: 'Female participants',
            value: formatNumber(stats.female_participants),
          },
        ])}
        <div class="timeline-list">
          ${structure.instances
            .map(
              (instance) => `
                <div class="timeline-item">
                  <span>${escapeHtml(formatDate(instance.date))}</span>
                  <strong>${escapeHtml(instance.location)}</strong>
                  <small>Event occurrence #${instance.id}</small>
                </div>
              `,
            )
            .join('')}
        </div>
      </article>
    </section>
  `;

  return {
    title: structure.name,
    html,
    afterRender() {
      const form = document.getElementById('event-detail-filters');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/events/${eventId}${serialiseQuery({
            year: data.get('year'),
            locationType: data.get('locationType'),
          })}`,
        );
      });

      form.querySelectorAll('select').forEach((select) => {
        select.addEventListener('change', () => form.requestSubmit());
      });
    },
  };
}

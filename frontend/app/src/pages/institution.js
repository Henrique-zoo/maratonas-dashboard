import {
  getInstitutionEventOptions,
  getInstitutionEventPerformance,
  getInstitutionStructures,
} from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import { getInstitutionOverview } from '../lib/metrics.js';
import {
  escapeHtml,
  formatDate,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderLineChart,
  renderPageIntro,
  renderStatGrid,
  renderTable,
  serialiseQuery,
} from '../lib/ui.js';

let dropdownAbortController = null;

function deriveDefaultRange(years = []) {
  const end = years.length ? Math.max(...years.map(Number)) : new Date().getFullYear();
  return {
    start: Math.max(end - 4, Math.min(...years.map(Number), end), 2000),
    end,
  };
}

export async function render({ params, query, navigate }) {
  const institutionId = Number(params.id);
  const [institution, eventOptions] = await Promise.all([
    getInstitutionStructures([institutionId]).then((structures) => structures[0]),
    getInstitutionEventOptions(institutionId),
  ]);

  if (!institution) {
    return {
      title: 'Institution not found',
      html: renderEmptyState(
        'Institution not found',
        'The requested institution does not exist in the current API snapshot.',
      ),
    };
  }

  const overview = getInstitutionOverview(institution);
  const selectedEvent = eventOptions.find((event) => event.id === Number(query.event)) || eventOptions[0];
  const defaults = deriveDefaultRange(selectedEvent?.years || []);
  const startYear = query.start ? Number(query.start) : defaults.start;
  const endYear = query.end ? Number(query.end) : defaults.end;
  const performance = selectedEvent
    ? await getInstitutionEventPerformance(institutionId, selectedEvent.id, startYear, endYear)
    : [];

  const html = `
    ${renderPageIntro({
      eyebrow: 'Institution',
      title: institution.name,
      blurb:
        'Inspect this institution across competitions, then pivot to a single event and watch performance rank move over time.',
      meta: [
        institution.short_name ? renderChip(institution.short_name, 'amber') : '',
        renderChip(institution.location, 'navy'),
      ].filter(Boolean),
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="institution-detail-filters">
        ${renderDropdown({
          name: 'event',
          label: 'Event',
          options: eventOptions.map((event) => ({
            id: String(event.id),
            name: `${event.competition_name} · ${event.name} (${event.years.join(', ')})`,
          })),
          selectedValue: selectedEvent?.id,
          placeholder: 'Select event',
          labelClass: 'filter-step filter-bar__grow',
          disabled: !eventOptions.length,
        })}
        <label>
          <span>Start year</span>
          <input name="start" type="number" value="${startYear}" min="2000" max="2100" />
        </label>
        <label>
          <span>End year</span>
          <input name="end" type="number" value="${endYear}" min="2000" max="2100" />
        </label>
        <button class="button" type="submit">Update trend</button>
      </form>
    </section>

    ${renderStatGrid([
      {
        label: 'Competitions',
        value: formatNumber(overview.competitionCount),
        hint: 'Latest participation snapshot each',
      },
      {
        label: 'Events',
        value: formatNumber(overview.eventCount),
        hint: 'Snapshot event results',
      },
      {
        label: 'Team entries',
        value: formatNumber(overview.teamEntries),
        hint: 'Across snapshot events',
      },
      {
        label: 'Female entries',
        value: formatNumber(overview.femaleParticipantEntries),
        hint: 'Contestant entries',
      },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Performance over time</span>
            <h2>${escapeHtml(selectedEvent?.name || 'Event trend')}</h2>
          </div>
        </div>
        <p class="card-note">${escapeHtml(selectedEvent ? `${selectedEvent.competition_name} · available years ${selectedEvent.years.join(', ')}` : 'Select an event to load a trend.')}</p>
        ${renderLineChart(
          performance.map((point) => ({
            label: point.year,
            bestRank: point.best_performance_rank,
            averageRank: point.average_performance_rank,
          })),
          {
            yLabel: 'Rank (lower is better)',
            reverseY: true,
            series: [
              { key: 'bestRank', label: 'Best rank' },
              { key: 'averageRank', label: 'Average rank' },
            ],
          },
        )}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Best runs</span>
            <h2>Top result by year</h2>
          </div>
        </div>
        ${
          performance.length
            ? renderTable({
                compact: true,
                columns: [
                  { label: 'Year' },
                  { label: 'Best team' },
                  { label: 'Best rank', align: 'right' },
                  { label: 'Average rank', align: 'right' },
                ],
                rows: performance.map((point) => [
                  { value: `<strong>${point.year}</strong>` },
                  { value: escapeHtml(point.best_performance_team_name) },
                  {
                    value: formatNumber(point.best_performance_rank),
                    align: 'right',
                  },
                  {
                    value: point.average_performance_rank.toFixed(2),
                    align: 'right',
                  },
                ]),
              })
            : renderEmptyState(
                'No historical performance',
                'The selected event has no rows in the chosen year window.',
              )
        }
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Latest participation snapshots</span>
          <h2>Most recent season reached in each competition</h2>
        </div>
      </div>
      <div class="stack-list">
        ${institution.competitions
          .map(
            (competition) => `
              <article class="panel panel--nested">
                <div class="section-head section-head--tight">
                  <div>
                    <h3>${escapeHtml(competition.name)}</h3>
                    <p>${escapeHtml(String(competition.events.length))} events · ${competition.snapshot_year}</p>
                  </div>
                </div>
                ${renderTable({
                  compact: true,
                  columns: [
                    { label: 'Event' },
                    { label: 'Date' },
                    { label: 'Level' },
                    { label: 'Teams', align: 'right' },
                    { label: 'Female entries', align: 'right' },
                  ],
                  rows: competition.events.map((event) => [
                    {
                      value: `<a href="/institutions/${institution.id}${serialiseQuery({
                        event: event.id,
                        start: startYear,
                        end: endYear,
                      })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                    },
                    { value: escapeHtml(formatDate(event.date)) },
                    { value: escapeHtml(event.scope) },
                    { value: formatNumber(event.teams.length), align: 'right' },
                    {
                      value: formatNumber(
                        event.teams.reduce((acc, team) => acc + team.female_participants, 0),
                      ),
                      align: 'right',
                    },
                  ]),
                })}
              </article>
            `,
          )
          .join('')}
      </div>
    </section>
  `;

  return {
    title: institution.short_name || institution.name,
    html,
    afterRender() {
      const form = document.getElementById('institution-detail-filters');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/institutions/${institutionId}${serialiseQuery({
            event: data.get('event'),
            start: data.get('start'),
            end: data.get('end'),
          })}`,
        );
      });

      form.querySelector('select[name="event"]').addEventListener('change', () => form.requestSubmit());
    },
  };
}

import { getInstitutionEventPerformance, getInstitutionStructures } from '../lib/api.js';
import { flattenInstitutionEvents, getInstitutionOverview, sortByDateDesc } from '../lib/metrics.js';
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

function deriveDefaultRange(eventDate) {
  const year = new Date(`${eventDate}T00:00:00`).getFullYear();
  return {
    start: Math.max(year - 4, 2000),
    end: year,
  };
}

export async function render({ params, query, navigate }) {
  const institutionId = Number(params.id);
  const institution = (await getInstitutionStructures([institutionId]))[0];

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
  const events = sortByDateDesc(flattenInstitutionEvents(institution));
  const selectedEvent = events.find((event) => event.id === Number(query.event)) || events[0];
  const defaults = deriveDefaultRange(selectedEvent?.date || new Date().toISOString().slice(0, 10));
  const startYear = query.start ? Number(query.start) : defaults.start;
  const endYear = query.end ? Number(query.end) : defaults.end;
  const performance = selectedEvent
    ? await getInstitutionEventPerformance(institutionId, selectedEvent.id, startYear, endYear)
    : [];

  const html = `
    ${renderPageIntro({
      eyebrow: 'Institution dossier',
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
        <label class="filter-bar__grow">
          <span>Event</span>
          <select name="event">
            ${events
              .map(
                (event) =>
                  `<option value="${event.id}" ${event.id === selectedEvent?.id ? 'selected' : ''}>${escapeHtml(event.competition_name)} · ${escapeHtml(event.name)} (${new Date(`${event.date}T00:00:00`).getFullYear()})</option>`,
              )
              .join('')}
          </select>
        </label>
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
      { label: 'Competitions', value: formatNumber(overview.competitionCount), hint: 'Active pipelines' },
      { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Tracked fixtures' },
      { label: 'Entries', value: formatNumber(overview.teamEntries), hint: 'Team appearances' },
      { label: 'Women tracked', value: formatNumber(overview.femaleParticipants), hint: 'Absolute count' },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Performance over time</span>
            <h2>${escapeHtml(selectedEvent?.name || 'Event trend')}</h2>
          </div>
        </div>
        <p class="card-note">${escapeHtml(selectedEvent ? `${selectedEvent.competition_name} · ${formatDate(selectedEvent.date)}` : 'Select an event to load a trend.')}</p>
        ${renderLineChart(
          performance.map((point) => ({ label: point.year, value: point.medium_performance_rank })),
          { yLabel: 'Average rank' },
        )}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Best runs</span>
            <h2>Top result by season</h2>
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
                  { value: formatNumber(point.best_performance_rank), align: 'right' },
                  { value: point.medium_performance_rank.toFixed(2), align: 'right' },
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
          <span class="eyebrow">Pipeline breakdown</span>
          <h2>Competitions, events and local teams</h2>
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
                    <p>${escapeHtml(String(competition.events.length))} tracked events</p>
                  </div>
                </div>
                ${renderTable({
                  compact: true,
                  columns: [
                    { label: 'Event' },
                    { label: 'Date' },
                    { label: 'Scope' },
                    { label: 'Teams', align: 'right' },
                    { label: 'Women', align: 'right' },
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

import { getEventLocationStats, getEventStats } from '../lib/api.js';
import { getEventMetadata } from '../lib/data-store.js';
import {
  chooseLocationType,
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

function parseLocationTypes(queryValue, fallback = []) {
  if (!queryValue) {
    return fallback;
  }

  return queryValue.split(',').filter(Boolean);
}

export async function render({ params, query, navigate }) {
  const eventId = Number(params.id);
  const metadata = (await getEventMetadata(eventId)) || null;
  const year = query.year
    ? Number(query.year)
    : metadata?.year || new Date(metadata?.date || Date.now()).getFullYear();
  const rawLocationTypes = parseLocationTypes(query.locationTypes, metadata?.locationTypes || []);
  const fallbackLocationType = query.locationType || chooseLocationType(rawLocationTypes);
  const locationChoices = rawLocationTypes.length ? rawLocationTypes : [fallbackLocationType];
  const selectedLocationType = query.locationType || fallbackLocationType;

  const [stats, locationStats] = await Promise.all([
    getEventStats(eventId, year),
    getEventLocationStats(eventId, selectedLocationType, year),
  ]);

  const name = query.name || metadata?.name || `Event #${eventId}`;
  const date = query.date || metadata?.date || null;
  const location = query.location || metadata?.location || 'Location unavailable';
  const competitionId = query.competitionId || metadata?.competitionId || null;
  const competitionName = query.competitionName || metadata?.competitionName || null;

  const html = `
    ${renderPageIntro({
      eyebrow: 'Event dossier',
      title: name,
      blurb:
        'Use the event board to understand roster size, institutional spread and location concentration for a single fixture.',
      meta: [
        date ? renderChip(formatDate(date), 'amber') : '',
        renderChip(location, 'navy'),
        ...locationChoices.map((locationType) => renderChip(locationType, 'slate')),
      ].filter(Boolean),
      actions: competitionId
        ? `<a class="button button--ghost" href="/competitions/${competitionId}${serialiseQuery({ year })}" data-link>${escapeHtml(competitionName || 'Open competition')}</a>`
        : '',
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="event-detail-filters">
        <label>
          <span>Year</span>
          <input name="year" type="number" value="${year}" min="2000" max="2100" />
        </label>
        <label>
          <span>Location tier</span>
          <select name="locationType">
            ${locationChoices
              .map(
                (locationType) =>
                  `<option value="${locationType}" ${locationType === selectedLocationType ? 'selected' : ''}>${locationType}</option>`,
              )
              .join('')}
          </select>
        </label>
        <button class="button" type="submit">Refresh event</button>
      </form>
    </section>

    ${renderStatGrid([
      { label: 'Institutions', value: formatNumber(stats.total_institutions), hint: `Year ${year}` },
      { label: 'Teams', value: formatNumber(stats.total_teams), hint: 'Qualified entries' },
      { label: 'Participants', value: formatNumber(stats.total_participants), hint: 'Roster count' },
      { label: 'Women tracked', value: formatNumber(stats.female_participants), hint: 'Absolute count' },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Location spread</span>
            <h2>${escapeHtml(selectedLocationType)} distribution</h2>
          </div>
        </div>
        ${renderBarList(
          locationStats.map((item) => ({
            label: item.name,
            value: item.total_teams,
          })),
          { valueFormatter: formatNumber },
        )}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Fixture snapshot</span>
            <h2>Quick read</h2>
          </div>
        </div>
        ${renderMetricStrip([
          { label: 'Participants', value: formatNumber(stats.total_participants) },
          { label: 'Women', value: formatNumber(stats.female_participants) },
          { label: 'Location rows', value: formatNumber(locationStats.length) },
        ])}
        <p class="card-note">
          ${escapeHtml(name)} was staged ${date ? `on ${formatDate(date)}` : 'on an unknown date'} in ${escapeHtml(location)}.
          ${competitionName ? ` It belongs to ${escapeHtml(competitionName)}.` : ''}
        </p>
      </article>
    </section>
  `;

  return {
    title: name,
    html,
    afterRender() {
      const form = document.getElementById('event-detail-filters');
      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/events/${eventId}${serialiseQuery({
            year: data.get('year'),
            locationType: data.get('locationType'),
            name,
            date,
            location,
            locationTypes: locationChoices.join(','),
            competitionId,
            competitionName,
          })}`,
        );
      });

      form
        .querySelector('select[name="locationType"]')
        .addEventListener('change', () => form.requestSubmit());
    },
  };
}

import { getOrganizerCompetitionYearStructure, getOrganizerStructures } from '../lib/api.js';
import { getOrganizerOverview, latestYear } from '../lib/metrics.js';
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

export async function render({ params, query, navigate }) {
  const organizerId = Number(params.id);
  const organizer = (await getOrganizerStructures([organizerId]))[0];

  if (!organizer) {
    return {
      title: 'Organizer not found',
      html: renderEmptyState(
        'Organizer not found',
        'The requested organizer is unavailable in the current dataset.',
      ),
    };
  }

  const overview = getOrganizerOverview(organizer);
  const selectedCompetition =
    organizer.competitions.find((competition) => competition.id === Number(query.competition)) ||
    organizer.competitions[0];
  const selectedYear = query.year ? Number(query.year) : latestYear(selectedCompetition?.years || []);
  const seasonStructure = selectedCompetition
    ? await getOrganizerCompetitionYearStructure(selectedCompetition.id, selectedYear)
    : { location_types: [], events: [] };

  const html = `
    ${renderPageIntro({
      eyebrow: 'Organizer dossier',
      title: organizer.name,
      blurb:
        'Use this board to switch between competitions under the same organizer umbrella and inspect one season at a time.',
      meta: [
        renderChip(`${overview.competitionCount} competitions`, 'amber'),
        renderChip(`${overview.eventCount} events`, 'navy'),
      ],
      actions: organizer.website_url
        ? `<a class="button button--ghost" href="${organizer.website_url}" target="_blank" rel="noreferrer">Official site</a>`
        : '',
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="organizer-detail-filters">
        <label class="filter-bar__grow">
          <span>Competition</span>
          <select name="competition">
            ${organizer.competitions
              .map(
                (competition) =>
                  `<option value="${competition.id}" ${competition.id === selectedCompetition?.id ? 'selected' : ''}>${escapeHtml(competition.name)}</option>`,
              )
              .join('')}
          </select>
        </label>
        <label>
          <span>Season</span>
          <select name="year">
            ${[...(selectedCompetition?.years || [])]
              .sort((left, right) => right - left)
              .map(
                (year) =>
                  `<option value="${year}" ${year === selectedYear ? 'selected' : ''}>${year}</option>`,
              )
              .join('')}
          </select>
        </label>
        <button class="button" type="submit">Refresh season</button>
      </form>
    </section>

    ${renderStatGrid([
      { label: 'Competitions', value: formatNumber(overview.competitionCount), hint: 'Under this organizer' },
      { label: 'Events', value: formatNumber(overview.eventCount), hint: 'Across all competitions' },
      { label: 'Participants', value: formatNumber(overview.totalParticipants), hint: 'Total tracked' },
      { label: 'Women tracked', value: formatNumber(overview.femaleParticipants), hint: 'Absolute count' },
    ])}

    ${
      selectedCompetition
        ? `
      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Selected competition</span>
              <h2>${escapeHtml(selectedCompetition.name)} · ${selectedYear}</h2>
            </div>
            <div class="tag-row">
              ${(seasonStructure.location_types || []).map((locationType) => renderChip(locationType, 'slate')).join('')}
            </div>
          </div>
          ${renderMetricStrip([
            { label: 'Tracked seasons', value: escapeHtml(selectedCompetition.years.join(', ')) },
            { label: 'Events this season', value: formatNumber(seasonStructure.events.length) },
            { label: 'Location tiers', value: formatNumber((seasonStructure.location_types || []).length) },
          ])}
          ${
            seasonStructure.events.length
              ? renderTable({
                  columns: [
                    { label: 'Event' },
                    { label: 'Date' },
                    { label: 'Teams', align: 'right' },
                    { label: 'Participants', align: 'right' },
                    { label: 'Women', align: 'right' },
                  ],
                  rows: seasonStructure.events.map((event) => [
                    {
                      value: `<a href="/events/${event.id}${serialiseQuery({
                        year: selectedYear,
                        name: event.name,
                        date: event.date,
                        locationTypes: (event.location_types || []).join(','),
                        competitionId: selectedCompetition.id,
                        competitionName: selectedCompetition.name,
                      })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                    },
                    { value: escapeHtml(formatDate(event.date)) },
                    { value: formatNumber(event.total_teams), align: 'right' },
                    { value: formatNumber(event.total_participants), align: 'right' },
                    { value: formatNumber(event.female_participants), align: 'right' },
                  ]),
                })
              : renderEmptyState(
                  'No event sheet for this season',
                  'Choose another competition or season in the filter bar.',
                )
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Competition portfolio</span>
              <h2>All competitions run by this organizer</h2>
            </div>
          </div>
          <div class="timeline-list">
            ${organizer.competitions
              .map(
                (competition) => `
                  <a class="timeline-item" href="/organizers/${organizer.id}${serialiseQuery({ competition: competition.id, year: latestYear(competition.years) })}" data-link>
                    <span>${escapeHtml(competition.years.join(', '))}</span>
                    <strong>${escapeHtml(competition.name)}</strong>
                    <small>${formatNumber(competition.events.length)} events · ${competition.location_types.join(', ')}</small>
                  </a>
                `,
              )
              .join('')}
          </div>
        </article>
      </section>
    `
        : renderEmptyState(
            'No competitions found',
            'This organizer currently has no competition structures attached.',
          )
    }
  `;

  return {
    title: organizer.name,
    html,
    afterRender() {
      const form = document.getElementById('organizer-detail-filters');
      if (!form) {
        return;
      }

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/organizers/${organizerId}${serialiseQuery({
            competition: data.get('competition'),
            year: data.get('year'),
          })}`,
        );
      });

      const competitionSelect = form.querySelector('select[name="competition"]');
      competitionSelect.addEventListener('change', () => {
        const competition = organizer.competitions.find(
          (item) => item.id === Number(competitionSelect.value),
        );
        navigate(
          `/organizers/${organizerId}${serialiseQuery({
            competition: competitionSelect.value,
            year: latestYear(competition?.years || []),
          })}`,
        );
      });

      form.querySelector('select[name="year"]').addEventListener('change', () => form.requestSubmit());
    },
  };
}

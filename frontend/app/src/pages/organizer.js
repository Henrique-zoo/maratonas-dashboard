import { getOrganizerCompetitionYearStructure, getOrganizerStructures } from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
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

let dropdownAbortController = null;

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
      eyebrow: 'Organizer',
      title: organizer.name,
      blurb: 'Review one competition-year at a time; portfolio totals use each competition’s latest season.',
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
        ${renderDropdown({
          name: 'competition',
          label: 'Competition',
          options: organizer.competitions.map((competition) => ({
            id: String(competition.id),
            name: competition.name,
          })),
          selectedValue: selectedCompetition?.id,
          placeholder: 'Select competition',
          labelClass: 'filter-step filter-bar__grow',
        })}
          ${renderDropdown({
            name: 'year',
            label: 'Year',
            options: [...(selectedCompetition?.years || [])]
              .sort((left, right) => right - left)
              .map((year) => ({ id: String(year), name: String(year) })),
            selectedValue: selectedYear,
            placeholder: 'Select year',
            disabled: !selectedCompetition?.years?.length,
          })}
        <button class="button" type="submit">Apply</button>
      </form>
    </section>

    ${renderStatGrid([
      {
        label: 'Competitions',
        value: formatNumber(overview.competitionCount),
        hint: 'Portfolio',
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
        hint: 'Latest-season contestant entries',
      },
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
          </div>
          ${renderMetricStrip([
            {
              label: 'Years',
              value: escapeHtml(selectedCompetition.years.join(', ')),
            },
            {
              label: 'Events',
              value: formatNumber(seasonStructure.events.length),
            },
            {
              label: 'Location levels',
              value: formatNumber((seasonStructure.location_types || []).length),
            },
          ])}
          ${
            seasonStructure.events.length
              ? renderTable({
                  columns: [
                    { label: 'Event' },
                    { label: 'Date' },
                    { label: 'Teams', align: 'right' },
                    { label: 'Participants', align: 'right' },
                    { label: 'Female participants', align: 'right' },
                  ],
                  rows: seasonStructure.events.map((event) => [
                    {
                      value: `<a href="/events/${event.id}${serialiseQuery({
                        year: selectedYear,
                      })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                    },
                    { value: escapeHtml(formatDate(event.date)) },
                    { value: formatNumber(event.total_teams), align: 'right' },
                    {
                      value: formatNumber(event.total_participants),
                      align: 'right',
                    },
                    {
                      value: formatNumber(event.female_participants),
                      align: 'right',
                    },
                  ]),
                })
              : renderEmptyState(
                  'No events for this year',
                  'Choose another competition or year in the filter bar.',
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
                    <small>Snapshot ${competition.snapshot_year} · ${formatNumber(competition.events.length)} events · ${competition.location_types.join(', ')}</small>
                  </a>
                `,
              )
              .join('')}
          </div>
        </article>
      </section>
    `
        : renderEmptyState('No competitions found', 'This organizer currently has no competitions attached.')
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

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

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

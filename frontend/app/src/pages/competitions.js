import { getCompetitionOptions, getCompetitionStructures, getOrganizerOptions } from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import { getCompetitionOverview } from '../lib/metrics.js';
import {
  escapeHtml,
  formatDate,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
  serialiseQuery,
} from '../lib/ui.js';

let dropdownAbortController = null;

export async function render({ query, navigate }) {
  const selectedOrganizer = query.organizer ? Number(query.organizer) : null;
  const searchTerm = (query.q || '').trim().toLowerCase();

  const [organizers, competitionOptions] = await Promise.all([
    getOrganizerOptions(),
    getCompetitionOptions(selectedOrganizer ? [selectedOrganizer] : null),
  ]);

  const competitionIds = competitionOptions.map((item) => item.id);
  const competitions = competitionIds.length ? await getCompetitionStructures(competitionIds) : [];

  const filteredCompetitions = competitions
    .map((competition) => ({
      competition,
      overview: getCompetitionOverview(competition),
    }))
    .filter(({ competition }) => {
      if (!searchTerm) {
        return true;
      }

      return [
        competition.name,
        competition.gender_category,
        competition.events.map((event) => event.name).join(' '),
      ]
        .join(' ')
        .toLowerCase()
        .includes(searchTerm);
    })
    .sort((left, right) => right.overview.uniqueTeams - left.overview.uniqueTeams);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Competition market',
      title: 'Competitions',
      blurb: 'Compare each competition using its latest available season snapshot.',
      meta: [renderChip(`${filteredCompetitions.length} visible competitions`, 'amber')],
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="competition-filters">
        ${renderDropdown({
          name: 'organizer',
          label: 'Organizer',
          options: organizers,
          selectedValue: selectedOrganizer,
          placeholder: 'All organizers',
        })}
        <label class="filter-bar__grow">
          <span>Search</span>
          <input name="q" type="search" value="${escapeHtml(query.q || '')}" placeholder="Competition, event or category" />
        </label>
        <button class="button" type="submit">Apply filters</button>
      </form>
    </section>

    ${
      filteredCompetitions.length
        ? `
      <section class="card-grid card-grid--two">
        ${filteredCompetitions
          .map(({ competition, overview }) => {
            const latestEvent = [...competition.events].sort(
              (left, right) => new Date(right.date) - new Date(left.date),
            )[0];
            return `
              <article class="list-card list-card--competition">
                <div class="list-card__title">
                  <div>
                    <h2><a href="/competitions/${competition.id}" data-link>${escapeHtml(competition.name)}</a></h2>
                    <p>Snapshot ${overview.snapshotYear} · ${escapeHtml(String(overview.eventCount))} events</p>
                  </div>
                  ${renderChip(competition.gender_category, 'navy')}
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
                  {
                    label: 'Female entries',
                    value: formatNumber(overview.femaleParticipantEntries),
                  },
                ])}
                <div class="tag-row">
                  ${(competition.location_types || []).map((locationType) => renderChip(locationType, 'slate')).join('')}
                </div>
                ${latestEvent ? `<p class="card-note">Latest fixture: ${escapeHtml(latestEvent.name)} on ${escapeHtml(formatDate(latestEvent.date))} in ${escapeHtml(latestEvent.location)}.</p>` : ''}
                <a class="button button--ghost" href="/competitions/${competition.id}${serialiseQuery({ year: overview.snapshotYear })}" data-link>Open competition</a>
              </article>
            `;
          })
          .join('')}
      </section>
    `
        : renderEmptyState(
            'No competitions match this filter',
            'Try removing the organizer filter or using a broader search term.',
          )
    }
  `;

  return {
    title: 'Competitions',
    html,
    afterRender() {
      const form = document.getElementById('competition-filters');
      const organizerSelect = form.querySelector('select[name="organizer"]');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/competitions${serialiseQuery({
            organizer: data.get('organizer'),
            q: data.get('q')?.toString().trim(),
          })}`,
        );
      });

      organizerSelect.addEventListener('change', () => {
        form.requestSubmit();
      });
    },
  };
}

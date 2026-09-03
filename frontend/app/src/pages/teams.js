import {
  getCompetitionOptions,
  getInstitutionOptions,
  getTeamOptions,
  getTeamStructures,
} from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import { getTeamOverview } from '../lib/metrics.js';
import {
  escapeHtml,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
  serialiseQuery,
} from '../lib/ui.js';

let dropdownAbortController = null;

export async function render({ query, navigate }) {
  const selectedCompetition = query.competition ? Number(query.competition) : null;
  const selectedInstitution = query.institution ? Number(query.institution) : null;
  const searchTerm = (query.q || '').trim().toLowerCase();

  const [competitionOptions, institutionOptions, teamOptions] = await Promise.all([
    getCompetitionOptions(),
    getInstitutionOptions(selectedCompetition ? [selectedCompetition] : null),
    getTeamOptions(
      selectedCompetition ? [selectedCompetition] : null,
      selectedInstitution ? [selectedInstitution] : null,
    ),
  ]);

  const teams = teamOptions.length ? await getTeamStructures(teamOptions.map((item) => item.id)) : [];

  const visibleTeams = teams
    .map((team) => ({ team, overview: getTeamOverview(team) }))
    .filter(({ team }) => {
      if (!searchTerm) {
        return true;
      }

      return [team.name, team.competitions.map((competition) => competition.name).join(' ')]
        .join(' ')
        .toLowerCase()
        .includes(searchTerm);
    })
    .sort((left, right) => right.overview.eventCount - left.overview.eventCount);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Team market',
      title: 'Teams',
      blurb:
        'Filter teams by historical participation, then compare their latest result snapshot in each competition.',
      meta: [renderChip(`${visibleTeams.length} visible teams`, 'amber')],
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="team-filters">
        ${renderDropdown({
          name: 'competition',
          label: 'Competition',
          options: competitionOptions,
          selectedValue: selectedCompetition,
          placeholder: 'All competitions',
        })}
        ${renderDropdown({
          name: 'institution',
          label: 'Institution',
          options: institutionOptions,
          selectedValue: selectedInstitution,
          placeholder: 'All institutions',
        })}
        <label class="filter-bar__grow">
          <span>Search</span>
          <input name="q" type="search" value="${escapeHtml(query.q || '')}" placeholder="Team or competition name" />
        </label>
        <button class="button" type="submit">Apply filters</button>
      </form>
    </section>

    ${
      visibleTeams.length
        ? `
      <section class="card-grid card-grid--three">
        ${visibleTeams
          .map(
            ({ team, overview }) => `
            <article class="list-card">
              <div class="list-card__title">
                <div>
                  <h2><a href="/teams/${team.id}${serialiseQuery({ competition: query.competition || team.competitions[0]?.id })}" data-link>${escapeHtml(team.name)}</a></h2>
                  <p>${escapeHtml(String(overview.competitionCount))} competitions · ${escapeHtml(String(overview.eventCount))} events</p>
                </div>
                ${team.competitions[0]?.gender_category ? renderChip(team.competitions[0].gender_category, 'navy') : ''}
              </div>
              ${renderMetricStrip([
                {
                  label: 'Competitions',
                  value: formatNumber(overview.competitionCount),
                },
                {
                  label: 'Snapshot events',
                  value: formatNumber(overview.eventCount),
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
              <p class="card-note">Reference seasons: ${escapeHtml(overview.snapshotYears.join(', ') || '—')}</p>
              <p class="card-note">Current portfolio: ${escapeHtml(
                team.competitions
                  .slice(0, 3)
                  .map((competition) => competition.name)
                  .join(', ') || '—',
              )}</p>
            </article>
          `,
          )
          .join('')}
      </section>
    `
        : renderEmptyState(
            'No teams match the selected filters',
            'Try a different competition, institution or broader search term.',
          )
    }
  `;

  return {
    title: 'Teams',
    html,
    afterRender() {
      const form = document.getElementById('team-filters');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/teams${serialiseQuery({
            competition: data.get('competition'),
            institution: data.get('institution'),
            q: data.get('q')?.toString().trim(),
          })}`,
        );
      });

      form.querySelectorAll('select').forEach((select) => {
        select.addEventListener('change', () => form.requestSubmit());
      });
    },
  };
}

import { getTeamCompetitionYearStructure, getTeamStructures } from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import { getTeamOverview, latestYear } from '../lib/metrics.js';
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

function renderCompetitionRail(team, selectedCompetitionId, selectedYear) {
  return `
    <div class="rail-list">
      ${team.competitions
        .map(
          (competition) => `
            <a class="rail-item ${competition.id === selectedCompetitionId ? 'is-active' : ''}" href="/teams/${team.id}${serialiseQuery({ competition: competition.id, year: selectedYear && competition.id === selectedCompetitionId ? selectedYear : latestYear(competition.years) })}" data-link>
              <strong>${escapeHtml(competition.name)}</strong>
              <small>${escapeHtml(competition.years.join(', '))}</small>
            </a>
          `,
        )
        .join('')}
    </div>
  `;
}

export async function render({ params, query, navigate }) {
  const teamId = Number(params.id);
  const team = (await getTeamStructures([teamId]))[0];

  if (!team) {
    return {
      title: 'Team not found',
      html: renderEmptyState('Team not found', 'The requested team is not present in the current dataset.'),
    };
  }

  const overview = getTeamOverview(team);
  const selectedCompetition =
    team.competitions.find((competition) => competition.id === Number(query.competition)) ||
    team.competitions[0];
  const selectedYear = query.year ? Number(query.year) : latestYear(selectedCompetition?.years || []);
  const seasonStructure = selectedCompetition
    ? await getTeamCompetitionYearStructure(teamId, selectedCompetition.id, selectedYear)
    : { events: [] };

  const html = `
    ${renderPageIntro({
      eyebrow: 'Team',
      title: team.name,
      blurb: 'Follow this team across competitions and years.',
      meta: [
        renderChip(`${overview.competitionCount} competitions`, 'amber'),
        renderChip(`${overview.eventCount} events`, 'navy'),
      ],
    })}

    <section class="panel">
      <div class="section-head">
        <div>
          <span class="eyebrow">Competitions</span>
          <h2>Choose a competition</h2>
        </div>
      </div>
      ${renderCompetitionRail(team, selectedCompetition?.id, selectedYear)}
    </section>

    ${
      selectedCompetition
        ? `
      <section class="panel panel--filters">
        <form class="filter-bar" id="team-detail-filters">
          ${renderDropdown({
            name: 'competition',
            label: 'Competition',
            options: team.competitions.map((competition) => ({
              id: String(competition.id),
              name: competition.name,
            })),
            selectedValue: selectedCompetition.id,
            placeholder: 'Select competition',
          })}
          ${renderDropdown({
            name: 'year',
            label: 'Year',
            options: [...selectedCompetition.years]
              .sort((left, right) => right - left)
              .map((year) => ({ id: String(year), name: String(year) })),
            selectedValue: selectedYear,
            placeholder: 'Select year',
          })}
          <button class="button" type="submit">Apply</button>
        </form>
      </section>

      ${renderStatGrid([
        {
          label: 'Years',
          value: escapeHtml(selectedCompetition.years.join(', ')),
          hint: selectedCompetition.gender_category,
        },
        {
          label: 'Contestant entries',
          value: formatNumber(seasonStructure.total_members),
          hint: `Across events in ${selectedYear}`,
        },
        {
          label: 'Female entries',
          value: formatNumber(seasonStructure.female_participants),
          hint: `Contestant entries in ${selectedYear}`,
        },
        {
          label: 'Events',
          value: formatNumber(seasonStructure.events.length),
          hint: `Year ${selectedYear}`,
        },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Results</span>
              <h2>${escapeHtml(selectedCompetition.name)} · ${selectedYear}</h2>
            </div>
            ${renderChip(selectedCompetition.gender_category, 'slate')}
          </div>
          ${
            seasonStructure.events.length
              ? renderTable({
                  columns: [
                    { label: 'Event' },
                    { label: 'Date' },
                    { label: 'Location' },
                    { label: 'Level' },
                    { label: 'Rank', align: 'right' },
                  ],
                  rows: seasonStructure.events.map((event) => [
                    {
                      value: `<a href="/events/${event.id}${serialiseQuery({
                        year: selectedYear,
                      })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                    },
                    { value: escapeHtml(formatDate(event.date)) },
                    { value: escapeHtml(event.location) },
                    { value: escapeHtml(event.scope) },
                    {
                      value: formatNumber(event.team_event_rank),
                      align: 'right',
                    },
                  ]),
                })
              : renderEmptyState(
                  'No events for this year',
                  'Pick another year or choose a different competition.',
                )
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Overview</span>
              <h2>Latest participation snapshot per competition</h2>
            </div>
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
          <div class="timeline-list">
            ${team.competitions
              .map(
                (competition) => `
                  <a class="timeline-item" href="/teams/${team.id}${serialiseQuery({ competition: competition.id, year: latestYear(competition.years) })}" data-link>
                    <span>${escapeHtml(competition.years.join(', '))}</span>
                    <strong>${escapeHtml(competition.name)}</strong>
                    <small>Snapshot ${competition.snapshot_year} · ${formatNumber(competition.events.length)} events · ${formatNumber(competition.female_participants)} female entries</small>
                  </a>
                `,
              )
              .join('')}
          </div>
        </article>
      </section>
    `
        : renderEmptyState(
            'No competition portfolio',
            'This team currently has no linked competitions in the API response.',
          )
    }
  `;

  return {
    title: team.name,
    html,
    afterRender() {
      const form = document.getElementById('team-detail-filters');

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
          `/teams/${teamId}${serialiseQuery({
            competition: data.get('competition'),
            year: data.get('year'),
          })}`,
        );
      });

      const competitionSelect = form.querySelector('select[name="competition"]');
      competitionSelect.addEventListener('change', () => {
        const competition = team.competitions.find((item) => item.id === Number(competitionSelect.value));
        navigate(
          `/teams/${teamId}${serialiseQuery({
            competition: competitionSelect.value,
            year: latestYear(competition?.years || []),
          })}`,
        );
      });

      form.querySelector('select[name="year"]').addEventListener('change', () => form.requestSubmit());
    },
  };
}

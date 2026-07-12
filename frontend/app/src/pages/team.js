import { getTeamCompetitionYearStructure, getTeamStructures } from '../lib/api.js';
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
      eyebrow: 'Team dossier',
      title: team.name,
      blurb:
        'Follow this team across its competition portfolio and drill into a specific season inside any competition track.',
      meta: [
        renderChip(`${overview.competitionCount} competitions`, 'amber'),
        renderChip(`${overview.eventCount} events`, 'navy'),
      ],
    })}

    <section class="panel">
      <div class="section-head">
        <div>
          <span class="eyebrow">Competition rail</span>
          <h2>Move between competitions</h2>
        </div>
      </div>
      ${renderCompetitionRail(team, selectedCompetition?.id, selectedYear)}
    </section>

    ${
      selectedCompetition
        ? `
      <section class="panel panel--filters">
        <form class="filter-bar" id="team-detail-filters">
          <label>
            <span>Competition</span>
            <select name="competition">
              ${team.competitions
                .map(
                  (competition) =>
                    `<option value="${competition.id}" ${competition.id === selectedCompetition.id ? 'selected' : ''}>${escapeHtml(competition.name)}</option>`,
                )
                .join('')}
            </select>
          </label>
          <label>
            <span>Season</span>
            <select name="year">
              ${[...selectedCompetition.years]
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
        {
          label: 'Competition seasons',
          value: escapeHtml(selectedCompetition.years.join(', ')),
          hint: selectedCompetition.gender_category,
        },
        {
          label: 'Members tracked',
          value: formatNumber(selectedCompetition.total_members),
          hint: 'Across this competition',
        },
        {
          label: 'Women tracked',
          value: formatNumber(selectedCompetition.female_participants),
          hint: 'Across this competition',
        },
        {
          label: 'Events this season',
          value: formatNumber(seasonStructure.events.length),
          hint: `Season ${selectedYear}`,
        },
      ])}

      <section class="content-grid">
        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Season results</span>
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
                    { label: 'Scope' },
                    { label: 'Rank', align: 'right' },
                  ],
                  rows: seasonStructure.events.map((event) => [
                    {
                      value: `<a href="/events/${event.id}${serialiseQuery({
                        year: selectedYear,
                        name: event.name,
                        date: event.date,
                        location: event.location,
                        competitionId: selectedCompetition.id,
                        competitionName: selectedCompetition.name,
                      })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                    },
                    { value: escapeHtml(formatDate(event.date)) },
                    { value: escapeHtml(event.location) },
                    { value: escapeHtml(event.scope) },
                    { value: formatNumber(event.team_event_rank), align: 'right' },
                  ]),
                })
              : renderEmptyState(
                  'No season rows for this competition',
                  'Pick another season or move to a different competition from the rail above.',
                )
          }
        </article>

        <article class="panel">
          <div class="section-head">
            <div>
              <span class="eyebrow">Portfolio snapshot</span>
              <h2>Everything tracked for this team</h2>
            </div>
          </div>
          ${renderMetricStrip([
            { label: 'Competitions', value: formatNumber(overview.competitionCount) },
            { label: 'Events', value: formatNumber(overview.eventCount) },
            { label: 'Members', value: formatNumber(overview.totalMembers) },
            { label: 'Women', value: formatNumber(overview.femaleParticipants) },
          ])}
          <div class="timeline-list">
            ${team.competitions
              .map(
                (competition) => `
                  <a class="timeline-item" href="/teams/${team.id}${serialiseQuery({ competition: competition.id, year: latestYear(competition.years) })}" data-link>
                    <span>${escapeHtml(competition.years.join(', '))}</span>
                    <strong>${escapeHtml(competition.name)}</strong>
                    <small>${formatNumber(competition.events.length)} events · ${formatNumber(competition.female_participants)} women</small>
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

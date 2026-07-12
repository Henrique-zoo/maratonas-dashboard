import {
  getCompetitionLocationStats,
  getCompetitionStats,
  getCompetitionStructures,
  getCompetitionYearStructure,
} from '../lib/api.js';
import {
  chooseLocationType,
  escapeHtml,
  formatDate,
  formatNumber,
  renderBarList,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
  renderStatGrid,
  renderTable,
  serialiseQuery,
} from '../lib/ui.js';
import { getCompetitionOverview, latestYear } from '../lib/metrics.js';

function renderEventCard(event, competitionId, competitionName, year) {
  const rows = event.teams.slice(0, 5).map((team) => [
    { value: `<strong>#${formatNumber(team.rank)}</strong>` },
    {
      value: `<a href="/teams/${team.id}${serialiseQuery({ competition: competitionId, year })}" data-link>${escapeHtml(team.name)}</a><small>${escapeHtml(team.institution_short_name || team.institution_name)}</small>`,
    },
    { value: formatNumber(team.total_members), align: 'right' },
    { value: formatNumber(team.female_participants), align: 'right' },
  ]);

  return `
    <article class="panel">
      <div class="section-head section-head--tight">
        <div>
          <span class="eyebrow">${escapeHtml(formatDate(event.date))}</span>
          <h3>
            <a href="/events/${event.id}${serialiseQuery({
              year,
              name: event.name,
              date: event.date,
              location: event.location,
              locationTypes: (event.location_types || []).join(','),
              competitionId,
              competitionName,
            })}" data-link>${escapeHtml(event.name)}</a>
          </h3>
        </div>
        <div class="tag-row">
          ${(event.location_types || []).map((locationType) => renderChip(locationType, 'slate')).join('')}
        </div>
      </div>
      <p class="card-note">${escapeHtml(event.location)} · ${escapeHtml(String(event.teams.length))} ranked teams</p>
      ${
        rows.length
          ? renderTable({
              compact: true,
              columns: [
                { label: 'Rank' },
                { label: 'Team' },
                { label: 'Members', align: 'right' },
                { label: 'Women', align: 'right' },
              ],
              rows,
            })
          : renderEmptyState('No ranking rows', 'This event has no team rows in the selected season.')
      }
    </article>
  `;
}

export async function render({ params, query, navigate }) {
  const competitionId = Number(params.id);
  const competition = (await getCompetitionStructures([competitionId]))[0];

  if (!competition) {
    return {
      title: 'Competition not found',
      html: renderEmptyState(
        'Competition not found',
        'The requested competition is not available in the current API snapshot.',
      ),
    };
  }

  const overview = getCompetitionOverview(competition);
  const selectedYear = query.year ? Number(query.year) : latestYear(competition.years);
  const yearStructure = await getCompetitionYearStructure(competitionId, selectedYear);
  const stats = await getCompetitionStats(competitionId, selectedYear);
  const rawLocationTypes = yearStructure.location_types?.length
    ? yearStructure.location_types
    : competition.location_types || [];
  const fallbackLocationType = query.locationType || chooseLocationType(rawLocationTypes);
  const locationChoices = rawLocationTypes.length ? rawLocationTypes : [fallbackLocationType];
  const selectedLocationType = query.locationType || fallbackLocationType;
  const locationStats = await getCompetitionLocationStats(competitionId, selectedLocationType, selectedYear);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Competition dossier',
      title: competition.name,
      blurb:
        'Season-by-season fixture sheet, roster volume and geographic spread for the selected competition.',
      meta: [
        renderChip(competition.gender_category, 'amber'),
        renderChip(`${overview.yearSpan} seasons`, 'navy'),
        ...(competition.location_types || []).map((locationType) => renderChip(locationType, 'slate')),
      ],
      actions: competition.website_url
        ? `<a class="button button--ghost" href="${competition.website_url}" target="_blank" rel="noreferrer">Official site</a>`
        : '',
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="competition-detail-filters">
        <label>
          <span>Season</span>
          <select name="year">
            ${[...competition.years]
              .sort((left, right) => right - left)
              .map(
                (year) =>
                  `<option value="${year}" ${year === selectedYear ? 'selected' : ''}>${year}</option>`,
              )
              .join('')}
          </select>
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
        <button class="button" type="submit">Refresh board</button>
      </form>
    </section>

    ${renderStatGrid([
      {
        label: 'Institutions',
        value: formatNumber(stats.total_institutions),
        hint: `Season ${selectedYear}`,
      },
      { label: 'Teams', value: formatNumber(stats.total_teams), hint: 'Qualified team entries' },
      { label: 'Participants', value: formatNumber(stats.total_participants), hint: 'Roster count' },
      { label: 'Women tracked', value: formatNumber(stats.female_participants), hint: 'Absolute count' },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Geographic split</span>
            <h2>${escapeHtml(selectedLocationType)} breakdown</h2>
          </div>
        </div>
        ${renderBarList(
          locationStats.map((item) => ({
            label: item.name,
            value: item.total_teams,
            subtitle: `${formatNumber(item.total_participants)} participants`,
          })),
          { valueFormatter: formatNumber },
        )}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Season summary</span>
            <h2>${selectedYear} fixture scope</h2>
          </div>
        </div>
        ${renderMetricStrip([
          { label: 'Events', value: formatNumber(yearStructure.events.length) },
          { label: 'Tracked years', value: overview.yearSpan },
          { label: 'Unique teams overall', value: formatNumber(overview.uniqueTeams) },
          { label: 'Women overall', value: formatNumber(overview.femaleParticipants) },
        ])}
        ${
          yearStructure.events.length
            ? renderTable({
                compact: true,
                columns: [
                  { label: 'Event' },
                  { label: 'Date' },
                  { label: 'Location' },
                  { label: 'Teams', align: 'right' },
                ],
                rows: yearStructure.events.map((event) => [
                  {
                    value: `<a href="/events/${event.id}${serialiseQuery({
                      year: selectedYear,
                      name: event.name,
                      date: event.date,
                      location: event.location,
                      locationTypes: (event.location_types || []).join(','),
                      competitionId,
                      competitionName: competition.name,
                    })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                  },
                  { value: escapeHtml(formatDate(event.date)) },
                  { value: escapeHtml(event.location) },
                  { value: formatNumber(event.teams.length), align: 'right' },
                ]),
              })
            : renderEmptyState(
                'No events found for this year',
                'Try selecting another season from the filter bar above.',
              )
        }
      </article>
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Event reports</span>
          <h2>Ranking cards inside the selected season</h2>
        </div>
      </div>
      <div class="card-grid card-grid--two">
        ${yearStructure.events.map((event) => renderEventCard(event, competitionId, competition.name, selectedYear)).join('') || renderEmptyState('No event cards available', 'There are no event rows to render in this season.')}
      </div>
    </section>
  `;

  return {
    title: competition.name,
    html,
    afterRender() {
      const form = document.getElementById('competition-detail-filters');
      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/competitions/${competitionId}${serialiseQuery({
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

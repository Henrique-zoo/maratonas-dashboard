import {
  getCompetitionLocationStats,
  getCompetitionStats,
  getCompetitionStructures,
  getCompetitionYearStructure,
} from '../lib/api.js';
import { initCustomDropdowns, renderDropdown } from '../lib/custom-select.js';
import {
  chooseLocationType,
  escapeHtml,
  formatDate,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderLineChart,
  renderMetricStrip,
  renderPageIntro,
  renderParticipationChart,
  renderStatGrid,
  renderTable,
  serialiseQuery,
} from '../lib/ui.js';
import { getCompetitionOverview, latestYear } from '../lib/metrics.js';

let dropdownAbortController = null;

function renderEventCard(event, competitionId, year) {
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
            })}" data-link>${escapeHtml(event.name)}</a>
          </h3>
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
                { label: 'Contestant entries', align: 'right' },
                { label: 'Female entries', align: 'right' },
              ],
              rows,
            })
          : renderEmptyState('No ranking rows', 'This event has no team rows in the selected year.')
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
  const annualOverview = getCompetitionOverview({
    ...competition,
    snapshot_year: selectedYear,
    events: yearStructure.events,
  });
  const stats = await getCompetitionStats(competitionId, selectedYear);
  const rawLocationTypes = yearStructure.location_types?.length
    ? yearStructure.location_types
    : competition.location_types || [];
  const fallbackLocationType = query.locationType || chooseLocationType(rawLocationTypes);
  const locationChoices = rawLocationTypes.length ? rawLocationTypes : [fallbackLocationType];
  const selectedLocationType = query.locationType || fallbackLocationType;
  const locationStats = await getCompetitionLocationStats(competitionId, selectedLocationType, selectedYear);
  const sortedYears = [...competition.years].map(Number).sort((left, right) => left - right);
  const annualStats = await Promise.all(
    sortedYears.map((year) =>
      year === selectedYear ? Promise.resolve(stats) : getCompetitionStats(competitionId, year),
    ),
  );
  const participationTrend = sortedYears.map((year, index) => ({
    label: year,
    participants: annualStats[index].total_participants,
    femaleParticipants: annualStats[index].female_participants,
  }));

  const html = `
    ${renderPageIntro({
      eyebrow: 'Competition',
      title: competition.name,
      blurb: 'Review events, teams and location data by year.',
      meta: [renderChip(competition.gender_category, 'amber'), renderChip(`Year ${selectedYear}`, 'navy')],
      actions: competition.website_url
        ? `<a class="button button--ghost" href="${competition.website_url}" target="_blank" rel="noreferrer">Official site</a>`
        : '',
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="competition-detail-filters">
          ${renderDropdown({
            name: 'year',
            label: 'Year',
            options: [...competition.years]
              .sort((left, right) => right - left)
              .map((year) => ({ id: String(year), name: String(year) })),
            selectedValue: selectedYear,
            placeholder: 'Select year',
          })}
          ${renderDropdown({
            name: 'locationType',
            label: 'Location level',
            options: locationChoices.map((locationType) => ({
              id: locationType,
              name: locationType,
            })),
            selectedValue: selectedLocationType,
            placeholder: 'Select location',
            disabled: !locationChoices.length,
          })}
        <button class="button" type="submit">Apply</button>
      </form>
    </section>

    ${renderStatGrid([
      {
        label: 'Institutions',
        value: formatNumber(stats.total_institutions),
        hint: `Distinct · year ${selectedYear}`,
      },
      {
        label: 'Teams',
        value: formatNumber(stats.total_teams),
        hint: `Distinct · year ${selectedYear}`,
      },
      {
        label: 'Participants',
        value: formatNumber(stats.total_participants),
        hint: `Distinct people · year ${selectedYear}`,
      },
      {
        label: 'Female participants',
        value: formatNumber(stats.female_participants),
        hint: 'Distinct people',
      },
    ])}

    <section class="content-grid">
      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Geographic split</span>
            <h2>${escapeHtml(selectedLocationType)} breakdown</h2>
          </div>
        </div>
        ${renderParticipationChart(
          locationStats.map((item) => ({
            label: item.name,
            total: item.total_participants,
            highlighted: item.female_participants,
            context: `${formatNumber(item.total_teams)} teams`,
          })),
        )}
      </article>

      <article class="panel">
        <div class="section-head">
          <div>
            <span class="eyebrow">Participation over time</span>
            <h2>Annual participant totals</h2>
          </div>
        </div>
        ${renderLineChart(participationTrend, {
          yLabel: 'Distinct participants',
          includeZero: true,
          series: [
            { key: 'participants', label: 'All participants' },
            { key: 'femaleParticipants', label: 'Female participants' },
          ],
        })}
      </article>
    </section>

    <section class="panel section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Year summary</span>
          <h2>${selectedYear} events</h2>
        </div>
      </div>
        ${renderMetricStrip([
          { label: 'Events', value: formatNumber(yearStructure.events.length) },
          { label: 'Available years', value: overview.yearSpan },
          {
            label: 'Unique teams',
            value: formatNumber(annualOverview.uniqueTeams),
          },
          {
            label: 'Female entries',
            value: formatNumber(annualOverview.femaleParticipantEntries),
          },
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
                    })}" data-link><strong>${escapeHtml(event.name)}</strong></a>`,
                  },
                  { value: escapeHtml(formatDate(event.date)) },
                  { value: escapeHtml(event.location) },
                  { value: formatNumber(event.teams.length), align: 'right' },
                ]),
              })
            : renderEmptyState(
                'No events found for this year',
                'Try selecting another year from the filter bar above.',
              )
        }
    </section>

    <section class="section-block">
      <div class="section-head">
        <div>
          <span class="eyebrow">Events</span>
          <h2>Rankings for the selected year</h2>
        </div>
      </div>
      <div class="card-grid card-grid--two">
        ${yearStructure.events.map((event) => renderEventCard(event, competitionId, selectedYear)).join('') || renderEmptyState('No event cards available', 'There are no events to show for this year.')}
      </div>
    </section>
  `;

  return {
    title: competition.name,
    html,
    afterRender() {
      const form = document.getElementById('competition-detail-filters');

      if (dropdownAbortController) {
        dropdownAbortController.abort();
      }

      dropdownAbortController = new AbortController();
      initCustomDropdowns(document, dropdownAbortController.signal);

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

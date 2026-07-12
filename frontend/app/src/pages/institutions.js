import { getCompetitionOptions, getInstitutionOptions, getInstitutionStructures } from '../lib/api.js';
import { getInstitutionOverview } from '../lib/metrics.js';
import {
  escapeHtml,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
  serialiseQuery,
} from '../lib/ui.js';

function optionMarkup(options, selectedValue, placeholder) {
  return `
    <option value="">${escapeHtml(placeholder)}</option>
    ${options
      .map(
        (option) =>
          `<option value="${option.id}" ${Number(selectedValue) === option.id ? 'selected' : ''}>${escapeHtml(option.name)}</option>`,
      )
      .join('')}
  `;
}

export async function render({ query, navigate }) {
  const selectedCompetition = query.competition ? Number(query.competition) : null;
  const searchTerm = (query.q || '').trim().toLowerCase();

  const [competitionOptions, institutionOptions] = await Promise.all([
    getCompetitionOptions(),
    getInstitutionOptions(selectedCompetition ? [selectedCompetition] : null),
  ]);

  const institutions = institutionOptions.length
    ? await getInstitutionStructures(institutionOptions.map((item) => item.id))
    : [];

  const visibleInstitutions = institutions
    .map((institution) => ({ institution, overview: getInstitutionOverview(institution) }))
    .filter(({ institution }) => {
      if (!searchTerm) {
        return true;
      }

      return [institution.name, institution.short_name, institution.location]
        .join(' ')
        .toLowerCase()
        .includes(searchTerm);
    })
    .sort((left, right) => right.overview.teamEntries - left.overview.teamEntries);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Institution market',
      title: 'Institution pipelines',
      blurb:
        'Evaluate universities and schools through event participation depth, competition reach and historical performance threads.',
      meta: [renderChip(`${visibleInstitutions.length} visible institutions`, 'amber')],
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="institution-filters">
        <label>
          <span>Competition</span>
          <select name="competition">
            ${optionMarkup(competitionOptions, selectedCompetition, 'All competitions')}
          </select>
        </label>
        <label class="filter-bar__grow">
          <span>Search</span>
          <input name="q" type="search" value="${escapeHtml(query.q || '')}" placeholder="Institution or location" />
        </label>
        <button class="button" type="submit">Apply filters</button>
      </form>
    </section>

    ${
      visibleInstitutions.length
        ? `
      <section class="card-grid card-grid--three">
        ${visibleInstitutions
          .map(
            ({ institution, overview }) => `
            <article class="list-card">
              <div class="list-card__title">
                <div>
                  <h2><a href="/institutions/${institution.id}" data-link>${escapeHtml(institution.short_name || institution.name)}</a></h2>
                  <p>${escapeHtml(institution.location)}</p>
                </div>
                ${renderChip(`${overview.competitionCount} competitions`, 'navy')}
              </div>
              ${renderMetricStrip([
                { label: 'Events', value: formatNumber(overview.eventCount) },
                { label: 'Entries', value: formatNumber(overview.teamEntries) },
                { label: 'Participants', value: formatNumber(overview.totalParticipants) },
                { label: 'Women', value: formatNumber(overview.femaleParticipants) },
              ])}
              <p class="card-note">Programs tracked: ${escapeHtml(
                institution.competitions
                  .slice(0, 3)
                  .map((competition) => competition.name)
                  .join(', ') || '—',
              )}</p>
              <a class="button button--ghost" href="/institutions/${institution.id}${serialiseQuery({ event: institution.competitions[0]?.events[0]?.id })}" data-link>Open institution</a>
            </article>
          `,
          )
          .join('')}
      </section>
    `
        : renderEmptyState(
            'No institutions match these filters',
            'Try clearing the competition filter or broadening the search term.',
          )
    }
  `;

  return {
    title: 'Institutions',
    html,
    afterRender() {
      const form = document.getElementById('institution-filters');
      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        navigate(
          `/institutions${serialiseQuery({
            competition: data.get('competition'),
            q: data.get('q')?.toString().trim(),
          })}`,
        );
      });

      form.querySelector('select[name="competition"]').addEventListener('change', () => form.requestSubmit());
    },
  };
}

import { getOrganizerOptions, getOrganizerStructures } from '../lib/api.js';
import { getOrganizerOverview } from '../lib/metrics.js';
import {
  escapeHtml,
  formatNumber,
  renderChip,
  renderEmptyState,
  renderMetricStrip,
  renderPageIntro,
} from '../lib/ui.js';

export async function render({ query, navigate }) {
  const searchTerm = (query.q || '').trim().toLowerCase();
  const organizerOptions = await getOrganizerOptions();
  const organizers = organizerOptions.length
    ? await getOrganizerStructures(organizerOptions.map((item) => item.id))
    : [];

  const visibleOrganizers = organizers
    .map((organizer) => ({ organizer, overview: getOrganizerOverview(organizer) }))
    .filter(({ organizer }) => {
      if (!searchTerm) {
        return true;
      }

      return organizer.name.toLowerCase().includes(searchTerm);
    })
    .sort((left, right) => right.overview.totalParticipants - left.overview.totalParticipants);

  const html = `
    ${renderPageIntro({
      eyebrow: 'Organizer market',
      title: 'Organizer network board',
      blurb:
        'See which competition networks operate the widest calendars, the deepest participation pools and the broadest location spread.',
      meta: [renderChip(`${visibleOrganizers.length} visible organizers`, 'amber')],
    })}

    <section class="panel panel--filters">
      <form class="filter-bar" id="organizer-filters">
        <label class="filter-bar__grow">
          <span>Search</span>
          <input name="q" type="search" value="${escapeHtml(query.q || '')}" placeholder="Organizer name" />
        </label>
        <button class="button" type="submit">Search</button>
      </form>
    </section>

    ${
      visibleOrganizers.length
        ? `
      <section class="card-grid card-grid--three">
        ${visibleOrganizers
          .map(
            ({ organizer, overview }) => `
            <article class="list-card">
              <div class="list-card__title">
                <div>
                  <h2><a href="/organizers/${organizer.id}" data-link>${escapeHtml(organizer.name)}</a></h2>
                  <p>${escapeHtml(String(overview.competitionCount))} competitions · ${escapeHtml(String(overview.eventCount))} events</p>
                </div>
                ${renderChip(`${overview.locationTypes.length} location tiers`, 'navy')}
              </div>
              ${renderMetricStrip([
                { label: 'Teams', value: formatNumber(overview.totalTeams) },
                { label: 'Participants', value: formatNumber(overview.totalParticipants) },
                { label: 'Women', value: formatNumber(overview.femaleParticipants) },
              ])}
              <p class="card-note">Reach: ${escapeHtml(overview.locationTypes.join(', ') || 'No location tiers tracked')}</p>
            </article>
          `,
          )
          .join('')}
      </section>
    `
        : renderEmptyState(
            'No organizers match your search',
            'Try a shorter search phrase or clear the search input.',
          )
    }
  `;

  return {
    title: 'Organizers',
    html,
    afterRender() {
      const form = document.getElementById('organizer-filters');
      form.addEventListener('submit', (event) => {
        event.preventDefault();
        const data = new FormData(form);
        const search = data.get('q')?.toString().trim();
        navigate(search ? `/organizers?q=${encodeURIComponent(search)}` : '/organizers');
      });
    },
  };
}

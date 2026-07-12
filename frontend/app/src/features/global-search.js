import { getSearchCatalog } from '../lib/data-store.js';
import { escapeHtml } from '../lib/ui.js';

function renderSearchResult(item) {
  return `
    <a class="search-result" href="${item.href}" data-link>
      <span class="search-result__type">${escapeHtml(item.type)}</span>
      <strong>${escapeHtml(item.name)}</strong>
      <small>${escapeHtml(item.subtitle)}</small>
    </a>
  `;
}

export function initGlobalSearch({ searchInput, searchPanel }) {
  const searchCatalogPromise = getSearchCatalog().catch(() => null);

  function closeSearchPanel() {
    searchPanel.hidden = true;
    searchPanel.innerHTML = '';
  }

  async function updateSearchPanel() {
    const term = searchInput.value.trim().toLowerCase();

    if (term.length < 2) {
      closeSearchPanel();
      return;
    }

    const catalog = await searchCatalogPromise;

    searchPanel.hidden = false;

    if (!catalog) {
      searchPanel.innerHTML = '<p class="search-panel__message">Search index unavailable right now.</p>';
      return;
    }

    const matches = catalog.searchIndex.filter((item) => item.searchText.includes(term)).slice(0, 8);

    searchPanel.innerHTML = matches.length
      ? matches.map((item) => renderSearchResult(item)).join('')
      : '<p class="search-panel__message">No matching entity in the current dataset.</p>';
  }

  searchInput.addEventListener('input', updateSearchPanel);
  searchInput.addEventListener('focus', updateSearchPanel);

  document.addEventListener('click', (event) => {
    if (!event.target.closest('.global-search')) {
      closeSearchPanel();
    }
  });

  return {
    close: closeSearchPanel,
    update: updateSearchPanel,
  };
}

import './styles/index.css';

import { initGlobalSearch } from './features/global-search.js';
import { mountSiteShell } from './layout/site-shell.js';
import { matchRoute } from './lib/router.js';
import { renderErrorState } from './lib/ui.js';

const { app, searchInput, searchPanel } = mountSiteShell();
const globalSearch = initGlobalSearch({ searchInput, searchPanel });

function parseQuery(search) {
  return Object.fromEntries(new URLSearchParams(search).entries());
}

function showLoading() {
  app.innerHTML = `
    <section class="loading-state">
      <span class="loading-state__tag">Loading desk</span>
      <h1>Pulling the latest competition ledger…</h1>
    </section>
  `;
}

export function navigate(href, { replace = false } = {}) {
  if (replace) {
    window.history.replaceState({}, '', href);
  } else {
    window.history.pushState({}, '', href);
  }

  renderCurrentRoute();
}

async function renderCurrentRoute() {
  const url = new URL(window.location.href);
  const route = matchRoute(url.pathname);

  document.querySelectorAll('[data-nav]').forEach((link) => {
    link.classList.toggle('is-active', route?.navKey && link.dataset.nav === route.navKey);
  });

  if (!route) {
    document.title = 'Page Not Found | MD Stack';
    app.innerHTML = renderErrorState('The requested route does not exist in this dashboard.');
    return;
  }

  showLoading();

  try {
    const result = await route.view({
      params: route.params,
      query: parseQuery(url.search),
      navigate,
    });

    document.title = `${result.title} | MD Stack`;
    app.innerHTML = result.html;

    if (typeof result.afterRender === 'function') {
      result.afterRender();
    }

    window.scrollTo({ top: 0, behavior: 'smooth' });
  } catch (error) {
    document.title = 'Something Went Wrong | MD Stack';
    app.innerHTML = renderErrorState(error.message);
  }
}

document.addEventListener('click', (event) => {
  const link = event.target.closest('a[data-link]');

  if (link && link.origin === window.location.origin) {
    event.preventDefault();
    globalSearch.close();
    navigate(`${link.pathname}${link.search}`);
  }
});

window.addEventListener('popstate', renderCurrentRoute);

renderCurrentRoute();

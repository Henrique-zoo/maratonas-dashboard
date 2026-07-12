const NAV_ITEMS = [
  { href: '/', label: 'Home', navKey: '/' },
  { href: '/competitions', label: 'Competitions', navKey: '/competitions' },
  { href: '/teams', label: 'Teams', navKey: '/teams' },
  { href: '/institutions', label: 'Institutions', navKey: '/institutions' },
  { href: '/organizers', label: 'Organizers', navKey: '/organizers' },
];

function renderNavLinks() {
  return NAV_ITEMS.map(
    (item) => `<a href="${item.href}" data-link data-nav="${item.navKey}">${item.label}</a>`,
  ).join('');
}

function renderHeader() {
  return `
    <header class="topbar">
      <div class="brand-cluster">
        <a class="brand-mark" href="/" data-link aria-label="MD Stack home">
          <img src="/assets/logo-mark.svg" alt="" />
        </a>
        <div class="brand-copy">
          <span class="brand-kicker">Maratona Data</span>
          <a class="brand-title" href="/" data-link>MD Stack</a>
        </div>
      </div>

      <nav class="primary-nav" aria-label="Primary">
        ${renderNavLinks()}
      </nav>

      <div class="toolbar">
        <form class="global-search" id="global-search-form" role="search">
          <label class="sr-only" for="global-search-input">Search entities</label>
          <input
            id="global-search-input"
            type="search"
            autocomplete="off"
            placeholder="Search teams, competitions, institutions..."
          />
          <div class="search-panel" id="search-panel" hidden></div>
        </form>
      </div>
    </header>
  `;
}

function renderFooter() {
  return `
    <footer class="site-footer">
      <div>
        <span class="footer-title">MD Stack</span>
        <p>
          A Transfermarkt-inspired view on programming competition ecosystems, rankings, institutions and
          organizer networks.
        </p>
      </div>
      <div class="footer-links">
        ${NAV_ITEMS.filter((item) => item.href !== '/')
          .map((item) => `<a href="${item.href}" data-link>${item.label}</a>`)
          .join('')}
      </div>
    </footer>
  `;
}

function renderSiteShell() {
  return `
    <div class="site-shell">
      ${renderHeader()}
      <main id="app" class="app-stage" aria-live="polite"></main>
      ${renderFooter()}
    </div>
  `;
}

export function mountSiteShell() {
  const root = document.getElementById('root');

  if (!root) {
    throw new Error('Application root element was not found.');
  }

  root.outerHTML = renderSiteShell();

  return {
    app: document.getElementById('app'),
    searchInput: document.getElementById('global-search-input'),
    searchPanel: document.getElementById('search-panel'),
  };
}

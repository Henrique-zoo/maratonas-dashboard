import { chromium } from 'playwright';

const organizerOptions = [
  {
    id: 300,
    name: 'International Alliance for Highly Collaborative Programming Contests and Marathon Data Operations',
  },
];

const competitionOptions = [
  {
    id: 1,
    name: 'South America Invitational Championship for Distributed Systems, Algorithms and Competitive Programming',
  },
];

const institutionOptions = [
  {
    id: 200,
    name: 'Federal Institute of Technology and Applied Computational Sciences of Sao Paulo Metropolitan Campus',
  },
];

const teamOptions = [
  {
    id: 100,
    name: 'Runtime Exception Handlers With Very Long Team Naming Convention',
  },
];

const teamEntry = {
  id: 100,
  name: teamOptions[0].name,
  institution_name: institutionOptions[0].name,
  institution_short_name: 'FITACSSPMC',
  total_members: 123456,
  female_participants: 45678,
  rank: 1234,
  team_event_rank: 1234,
};

const eventEntry = {
  id: 10,
  name: 'Grand Final Round for Multi Regional Programming Marathon and Algorithmic Systems Design',
  date: '2025-05-10',
  location: 'Sao Paulo Metropolitan Innovation District With A Particularly Long Venue Name',
  scope: 'International multi-campus online onsite hybrid regional scope',
  location_types: ['Country', 'Administrative Region', 'Metropolitan Innovation District'],
  total_teams: 98765,
  total_participants: 1234567,
  female_participants: 456789,
  teams: [teamEntry],
};

const competitionStructures = [
  {
    id: 1,
    name: competitionOptions[0].name,
    gender_category: 'Mixed category with extended eligibility wording',
    website_url: 'https://example.test/mock-competition',
    years: [2020, 2021, 2022, 2023, 2024, 2025],
    location_types: eventEntry.location_types,
    events: [eventEntry],
  },
];

const organizerStructures = [
  {
    id: 300,
    name: organizerOptions[0].name,
    website_url: 'https://example.test/mock-organizer',
    competitions: [
      {
        ...competitionStructures[0],
        total_participants: 1234567,
        female_participants: 456789,
      },
    ],
  },
];

const institutionStructures = [
  {
    id: 200,
    name: institutionOptions[0].name,
    short_name: 'FITACSSPMC',
    location: 'Sao Paulo Metropolitan Innovation District With Long Geographic Label',
    competitions: [
      {
        id: 1,
        name: competitionOptions[0].name,
        website_url: 'https://example.test/mock-competition',
        events: [eventEntry],
      },
    ],
  },
];

const teamStructures = [
  {
    id: 100,
    name: teamOptions[0].name,
    institution_name: institutionOptions[0].name,
    institution_short_name: 'FITACSSPMC',
    competitions: [
      {
        id: 1,
        name: competitionOptions[0].name,
        gender_category: 'Mixed category with extended eligibility wording',
        years: [2020, 2021, 2022, 2023, 2024, 2025],
        total_members: 123456,
        female_participants: 45678,
        events: [eventEntry],
      },
    ],
  },
];

const responsesByPath = new Map([
  ['/api/organizers/options', organizerOptions],
  ['/api/competitions/options', competitionOptions],
  ['/api/institutions/options', institutionOptions],
  ['/api/teams/options', teamOptions],
  ['/api/competitions/structures', competitionStructures],
  ['/api/organizers/structures', organizerStructures],
  ['/api/institutions/structures', institutionStructures],
  ['/api/teams/structures', teamStructures],
  ['/api/organizers/competitions/1/structure', competitionStructures[0]],
  ['/api/competitions/1/structure', competitionStructures[0]],
  ['/api/competitions/1/stats', {
    total_institutions: 12345,
    total_teams: 98765,
    total_participants: 1234567,
    female_participants: 456789,
  }],
  ['/api/competitions/1/location_stats', [
    { name: eventEntry.location, total_teams: 98765, total_participants: 1234567 },
  ]],
  ['/api/institutions/200/events/10', [
    {
      year: 2021,
      best_performance_team_name: teamOptions[0].name,
      best_performance_rank: 1234,
      medium_performance_rank: 876.54,
    },
    {
      year: 2025,
      best_performance_team_name: teamOptions[0].name,
      best_performance_rank: 234,
      medium_performance_rank: 321.45,
    },
  ]],
  ['/api/teams/100/competitions/1', { events: [eventEntry] }],
  ['/api/events/10/stats', {
    total_institutions: 12345,
    total_teams: 98765,
    total_participants: 1234567,
    female_participants: 456789,
  }],
  ['/api/events/10/location_stats', [
    { name: eventEntry.location, total_teams: 98765 },
  ]],
]);

const routes = [
  '/',
  '/?organizer=300&competition=1&institution=200&team=100',
  '/competitions',
  '/competitions/1?year=2025',
  '/teams',
  '/teams/100?competition=1&year=2025',
  '/institutions',
  '/institutions/200?event=10',
  '/organizers',
  '/organizers/300?competition=1&year=2025',
  `/events/10?year=2025&name=${encodeURIComponent(eventEntry.name)}&date=${eventEntry.date}&location=${encodeURIComponent(eventEntry.location)}&locationTypes=${eventEntry.location_types.join(',')}&competitionId=1&competitionName=${encodeURIComponent(competitionOptions[0].name)}`,
];

const viewports = [
  { name: 'desktop', width: 1366, height: 900 },
  { name: 'mobile', width: 390, height: 844 },
];

function visibleElementIssues() {
  const ignoredSelectors = ['html', 'body', '.data-table'];

  return Array.from(document.querySelectorAll('*'))
    .filter((element) => {
      if (ignoredSelectors.some((selector) => element.matches(selector))) {
        return false;
      }

      const style = getComputedStyle(element);
      const rect = element.getBoundingClientRect();
      return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
    })
    .map((element) => {
      const rect = element.getBoundingClientRect();
      const horizontalOverflow = Math.ceil(element.scrollWidth - element.clientWidth);
      const viewportOverflow = Math.ceil(Math.max(0, rect.right - window.innerWidth, -rect.left));

      if (horizontalOverflow <= 1 && viewportOverflow <= 1) {
        return null;
      }

      return {
        selector: element.className || element.tagName.toLowerCase(),
        text: element.textContent.trim().replace(/\s+/g, ' ').slice(0, 100),
        horizontalOverflow,
        viewportOverflow,
        width: Math.round(rect.width),
      };
    })
    .filter(Boolean)
    .slice(0, 16);
}

const browser = await chromium.launch();
const page = await browser.newPage();

await page.route('**/api/**', async (route) => {
  const url = new URL(route.request().url());
  const body = responsesByPath.get(url.pathname);

  if (!body) {
    await route.fulfill({ status: 404, json: { error: `Unhandled test API path: ${url.pathname}` } });
    return;
  }

  await route.fulfill({ json: body });
});

for (const viewport of viewports) {
  await page.setViewportSize({ width: viewport.width, height: viewport.height });

  for (const route of routes) {
    await page.goto(`http://127.0.0.1:4173${route}`, { waitUntil: 'networkidle' });
    await page.locator('.loading-state').waitFor({ state: 'detached', timeout: 10000 }).catch(() => {});

    const issues = await page.evaluate(visibleElementIssues);
    const documentOverflow = await page.evaluate(() => ({
      scrollWidth: document.documentElement.scrollWidth,
      clientWidth: document.documentElement.clientWidth,
    }));

    if (issues.length || documentOverflow.scrollWidth > documentOverflow.clientWidth + 1) {
      console.log(`${viewport.name} ${route}`);
      console.log(JSON.stringify({ documentOverflow, issues }, null, 2));
    }
  }
}

await page.setViewportSize({ width: 390, height: 844 });
await page.goto('http://127.0.0.1:4173/?organizer=300&competition=1&institution=200&team=100', {
  waitUntil: 'networkidle',
});
await page.screenshot({ path: '/tmp/md-stack-mobile-audit.png', fullPage: true });

await page.setViewportSize({ width: 1366, height: 900 });
await page.goto('http://127.0.0.1:4173/competitions/1?year=2025', { waitUntil: 'networkidle' });
await page.screenshot({ path: '/tmp/md-stack-desktop-audit.png', fullPage: true });

await browser.close();

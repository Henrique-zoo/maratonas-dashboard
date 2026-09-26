import { expect, test } from '@playwright/test';

const competition = {
  id: 1,
  name: 'Global Algorithm Cup',
  gender_category: 'Mixed',
  website_url: 'https://example.test/competition',
  years: [2023, 2024, 2025],
  snapshot_year: 2025,
  location_types: ['Country'],
  events: [],
};

const competitionStats = new Map([
  [2023, { total_institutions: 4, total_teams: 8, total_participants: 24, female_participants: 5 }],
  [2024, { total_institutions: 5, total_teams: 10, total_participants: 30, female_participants: 8 }],
  [2025, { total_institutions: 6, total_teams: 12, total_participants: 36, female_participants: 12 }],
]);

const institution = {
  id: 200,
  name: 'Mock University',
  short_name: 'MU',
  location: 'Brazil',
  competitions: [],
};

const performance = [
  {
    year: 2022,
    best_performance_team_name: 'Mock Team',
    best_performance_rank: 8,
    average_performance_rank: 10.5,
  },
  {
    year: 2023,
    best_performance_team_name: 'Mock Team',
    best_performance_rank: 5,
    average_performance_rank: 7.25,
  },
  {
    year: 2024,
    best_performance_team_name: 'Mock Team',
    best_performance_rank: 3,
    average_performance_rank: 5.5,
  },
  {
    year: 2025,
    best_performance_team_name: 'Mock Team',
    best_performance_rank: 1,
    average_performance_rank: 3.25,
  },
];

async function mockChartsApi(page) {
  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());
    let body = null;

    if (url.pathname === '/api/competitions/structures') {
      body = [competition];
    } else if (url.pathname === '/api/competitions/1/structure') {
      body = {
        location_types: ['Country'],
        events: [
          {
            id: 10,
            name: 'World Final',
            date: `${url.searchParams.get('year')}-05-10`,
            location: 'Brazil',
            teams: [],
          },
        ],
      };
    } else if (url.pathname === '/api/competitions/1/stats') {
      body = competitionStats.get(Number(url.searchParams.get('year')));
    } else if (url.pathname === '/api/competitions/1/location-stats') {
      body = [
        {
          id: 1,
          name: 'Brazil',
          total_institutions: 4,
          total_teams: 8,
          total_participants: 24,
          female_participants: 8,
        },
        {
          id: 2,
          name: 'Chile',
          total_institutions: 2,
          total_teams: 4,
          total_participants: 12,
          female_participants: 3,
        },
      ];
    } else if (url.pathname === '/api/institutions/structures') {
      body = [institution];
    } else if (url.pathname === '/api/institutions/200/events/options') {
      body = [
        {
          id: 10,
          name: 'World Final',
          competition_name: 'Global Algorithm Cup',
          years: [2022, 2023, 2024, 2025],
        },
      ];
    } else if (url.pathname === '/api/institutions/200/events/10/performance') {
      body = performance;
    }

    if (!body) {
      await route.fulfill({ status: 404, json: { error: `Unhandled path ${url.pathname}` } });
      return;
    }

    await route.fulfill({ json: body });
  });
}

test('competition page plots annual and geographic participation', async ({ page }) => {
  await mockChartsApi(page);
  await page.goto('/competitions/1?year=2025&locationType=Country');

  await expect(page.getByRole('heading', { name: 'Annual participant totals' })).toBeVisible();
  await expect(page.getByRole('img', { name: 'Distinct participants over time' })).toBeVisible();
  await expect(page.getByText('All participants', { exact: true })).toBeVisible();
  await expect(page.getByText('Female participants', { exact: true }).first()).toBeVisible();
  await expect(page.locator('circle[aria-label="2025, All participants: 36"]')).toBeVisible();

  await expect(page.getByRole('figure', { name: 'Participants by location' })).toBeVisible();
  await expect(
    page.getByRole('img', {
      name: 'Brazil: 24 participants, 8 female participants · 8 teams',
    }),
  ).toBeVisible();
});

test('institution page plots best and average rank with the ranking axis inverted', async ({ page }) => {
  await mockChartsApi(page);
  await page.goto('/institutions/200?event=10&start=2022&end=2025');

  const chart = page.getByRole('img', { name: 'Rank (lower is better) over time' });
  const chartFigure = page.getByRole('figure').filter({ has: chart });
  await expect(chart).toBeVisible();
  await expect(chartFigure.getByText('Best rank', { exact: true })).toBeVisible();
  await expect(chartFigure.getByText('Average rank', { exact: true })).toBeVisible();

  const earlierPoint = page.locator('circle[aria-label="2022, Best rank: 8"]');
  const latestPoint = page.locator('circle[aria-label="2025, Best rank: 1"]');
  await expect(earlierPoint).toBeVisible();
  await expect(latestPoint).toBeVisible();

  const earlierY = Number(await earlierPoint.getAttribute('cy'));
  const latestY = Number(await latestPoint.getAttribute('cy'));
  expect(latestY).toBeLessThan(earlierY);
});

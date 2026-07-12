import { expect, test } from '@playwright/test';

const organizerOptions = [{ id: 300, name: 'Mock Organizer' }];
const competitionOptions = [{ id: 1, name: 'Mock Competition' }];
const institutionOptions = [{ id: 200, name: 'Mock University' }];
const teamOptions = [{ id: 100, name: 'Mock Team' }];

const teamEntry = {
  id: 100,
  name: 'Mock Team',
  institution_name: 'Mock University',
  institution_short_name: 'MU',
  total_members: 3,
  female_participants: 1,
};

const eventEntry = {
  id: 10,
  name: 'Mock Event',
  date: '2025-05-10',
  location: 'Sao Paulo',
  location_types: ['onsite'],
  total_teams: 1,
  total_participants: 3,
  female_participants: 1,
  teams: [teamEntry],
};

const competitionStructures = [
  {
    id: 1,
    name: 'Mock Competition',
    gender_category: 'mixed',
    website_url: 'https://example.test/mock-competition',
    years: [2025],
    events: [eventEntry],
  },
];

const organizerStructures = [
  {
    id: 300,
    name: 'Mock Organizer',
    competitions: [
      {
        id: 1,
        name: 'Mock Competition',
        years: [2025],
        location_types: ['onsite'],
        events: [eventEntry],
      },
    ],
  },
];

const institutionStructures = [
  {
    id: 200,
    name: 'Mock University',
    short_name: 'MU',
    location: 'Sao Paulo',
    competitions: [
      {
        id: 1,
        name: 'Mock Competition',
        website_url: 'https://example.test/mock-competition',
        events: [eventEntry],
      },
    ],
  },
];

const teamStructures = [
  {
    id: 100,
    name: 'Mock Team',
    institution_name: 'Mock University',
    institution_short_name: 'MU',
    competitions: [
      {
        id: 1,
        name: 'Mock Competition',
        total_members: 3,
        female_participants: 1,
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
]);

async function mockApi(page) {
  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());
    const body = responsesByPath.get(url.pathname);

    if (!body) {
      await route.fulfill({
        status: 404,
        json: { error: `Unhandled test API path: ${url.pathname}` },
      });
      return;
    }

    await route.fulfill({ json: body });
  });
}

test('home filters update locally and apply through the button', async ({ page }) => {
  const apiRequests = [];

  await mockApi(page);
  page.on('request', (request) => {
    const url = new URL(request.url());

    if (url.pathname.startsWith('/api/')) {
      apiRequests.push(request.url());
    }
  });

  await page.goto('/');
  await expect(
    page.getByRole('heading', { name: 'Track programming ecosystems like a transfer market.' }),
  ).toBeVisible();
  await expect(page.getByRole('button', { name: 'Select organizer', exact: true })).toBeVisible();

  apiRequests.length = 0;
  await page.getByRole('button', { name: 'Select organizer', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Organizer', exact: true }).click();

  await page.getByRole('button', { name: 'Select competition', exact: true }).click();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Mock Competition', exact: true }),
  ).toBeVisible();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Competition', exact: true }).click();

  await page.getByRole('button', { name: 'Select institution', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock University', exact: true }).click();

  await page.getByRole('button', { name: 'Select team', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Team', exact: true }).click();

  await expect(page).toHaveURL(/\/$/);
  expect(apiRequests).toHaveLength(0);

  await page.getByRole('button', { name: 'Apply Filters' }).click();

  await expect(page).toHaveURL(/\?organizer=300&competition=1&institution=200&team=100$/);
  await expect(page.getByText('Organizer: Mock Organizer')).toBeVisible();
  await expect(page.getByText('Showing Team structure for Mock Team.')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Mock Team' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Competitions returned for this team' })).toBeVisible();
  await expect(page.getByRole('link', { name: /Mock Event/ }).first()).toBeVisible();
});

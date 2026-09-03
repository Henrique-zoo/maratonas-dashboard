import { expect, test } from '@playwright/test';

const organizerOptions = [{ id: 300, name: 'Mock Organizer' }];

const eventEntry = {
  id: 10,
  name: 'Mock Event',
  date: '2025-05-10',
  location: 'Sao Paulo',
  location_types: ['Country', 'City'],
  total_institutions: 1,
  total_teams: 2,
  total_participants: 6,
  female_participants: 1,
};

const secondEventEntry = {
  id: 11,
  name: 'Mock Final',
  date: '2025-06-15',
  location: 'Rio de Janeiro',
  location_types: ['Country', 'City'],
  total_institutions: 1,
  total_teams: 1,
  total_participants: 3,
  female_participants: 1,
};

const previousYearEventEntry = {
  id: 12,
  name: 'Mock Previous Regional',
  date: '2024-05-08',
  location: 'Campinas',
  location_types: ['Country', 'City'],
  total_institutions: 1,
  total_teams: 1,
  total_participants: 3,
  female_participants: 1,
};

const organizerStructures = [
  {
    id: 300,
    name: 'Mock Organizer',
    website_url: 'https://example.test/mock-organizer',
    competitions: [
      {
        id: 1,
        name: 'Mock Competition',
        website_url: 'https://example.test/mock-competition',
        gender_category: 'Open',
        years: [2024, 2025],
        snapshot_year: 2025,
        location_types: ['Country', 'City'],
        events: [eventEntry, secondEventEntry],
      },
    ],
  },
];

const competitionYearStructure = {
  location_types: ['Country', 'City'],
  events: [previousYearEventEntry],
};

const competitionStats = {
  total_institutions: 2,
  total_teams: 3,
  total_participants: 6,
  female_participants: 1,
};

const eventStats = {
  total_institutions: 1,
  total_teams: 2,
  total_participants: 6,
  female_participants: 1,
};

const locationStats = [
  {
    id: 1,
    name: 'Brazil',
    total_institutions: 2,
    total_teams: 3,
    total_participants: 6,
    female_participants: 1,
  },
];

const responsesByPath = new Map([
  ['/api/organizers/options', organizerOptions],
  ['/api/organizers/structures', organizerStructures],
  ['/api/organizers/competitions/1/structure', competitionYearStructure],
  ['/api/competitions/1/stats', competitionStats],
  ['/api/competitions/1/location-stats', locationStats],
  ['/api/events/10/stats', eventStats],
  ['/api/events/10/location-stats', locationStats],
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

test('organizer index renders competition cards and drills into year and location filters', async ({
  page,
}) => {
  const apiRequests = [];

  await mockApi(page);
  page.on('request', (request) => {
    const url = new URL(request.url());

    if (url.pathname.startsWith('/api/')) {
      apiRequests.push(url);
    }
  });

  await page.goto('/organizers');

  const organizerCard = page.locator('.organizer-structure-card').filter({ hasText: 'Mock Organizer' });
  const competitionCard = organizerCard
    .locator('.organizer-competition-card')
    .filter({ hasText: 'Mock Competition' });

  await expect(organizerCard).toBeVisible();
  await expect(competitionCard).toBeVisible();
  await expect(
    competitionCard.getByRole('link', {
      name: 'Open Mock Competition website',
    }),
  ).toHaveAttribute('href', 'https://example.test/mock-competition');
  await expect(competitionCard.getByRole('link', { name: /Mock Event/ })).toBeVisible();
  await expect(competitionCard.getByRole('link', { name: /Mock Final/ })).toBeVisible();

  await competitionCard.click({ position: { x: 20, y: 20 } });

  await expect(page).toHaveURL(/\?competition=1&year=2025&locationType=Country$/);
  await expect(page.locator('.organizer-inspector').getByText('Competition details')).toBeVisible();
  await expect(
    page.locator('.organizer-inspector').getByRole('heading', { name: 'Mock Competition' }),
  ).toBeVisible();
  await expect(page.locator('.organizer-inspector').getByText('Participants', { exact: true })).toBeVisible();
  await expect(
    page.locator('.organizer-inspector').getByText('Female participants', { exact: true }),
  ).toBeVisible();
  await expect(
    page.locator('.organizer-inspector').getByText('Contestant entries', { exact: true }),
  ).toHaveCount(0);
  await expect(page.locator('.organizer-inspector').getByText('Country split')).toBeVisible();
  await expect(page.locator('.organizer-inspector').getByText('Brazil')).toBeVisible();

  await competitionCard.getByRole('link', { name: /Mock Event/ }).click();

  await expect(page).toHaveURL(/\?competition=1&event=10&year=2025&locationType=Country$/);
  await expect(page.locator('.organizer-inspector').getByText('Event details')).toBeVisible();
  await expect(
    page.locator('.organizer-inspector').getByRole('heading', { name: 'Mock Event' }),
  ).toBeVisible();

  const yearForm = competitionCard.locator('.organizer-competition-year-form');
  await yearForm.getByRole('button', { name: '2025', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: '2024', exact: true }).click();

  await expect(page).toHaveURL(/\?competition=1&year=2024$/);
  await expect(competitionCard.getByRole('link', { name: /Mock Previous Regional/ })).toBeVisible();
  await expect(competitionCard.getByRole('link', { name: /Mock Final/ })).toHaveCount(0);
  expect(
    apiRequests.some(
      (url) =>
        url.pathname === '/api/organizers/competitions/1/structure' &&
        url.searchParams.get('year') === '2024',
    ),
  ).toBe(true);
});

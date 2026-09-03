import { expect, test } from '@playwright/test';

const organizers = [
  { id: 1, name: 'Organizer A' },
  { id: 2, name: 'Organizer B' },
];
const competitions = [
  { id: 11, name: 'Competition A' },
  { id: 22, name: 'Competition B' },
];
const competitionStructures = competitions.map((competition) => ({
  ...competition,
  gender_category: 'Open',
  website_url: null,
  years: [2025],
  snapshot_year: 2025,
  location_types: ['Country'],
  events: [],
}));
const organizerStructures = organizers.map((organizer, index) => ({
  ...organizer,
  competitions: [competitionStructures[index]],
}));

async function mockCascadeApi(page) {
  const requests = [];

  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());
    requests.push(url);

    if (url.pathname === '/api/organizers/options') {
      await route.fulfill({ json: organizers });
      return;
    }

    if (url.pathname === '/api/competitions/options') {
      const organizerIds = url.searchParams.get('organizer_ids') || '';

      if (organizerIds === '1') {
        await new Promise((resolve) => setTimeout(resolve, 600));
        await route.fulfill({ json: [competitions[0]] });
        return;
      }

      await route.fulfill({
        json: organizerIds === '2' ? [competitions[1]] : competitions,
      });
      return;
    }

    const payloads = new Map([
      ['/api/institutions/options', []],
      ['/api/teams/options', []],
      ['/api/competitions/structures', competitionStructures],
      ['/api/organizers/structures', organizerStructures],
    ]);
    const payload = payloads.get(url.pathname);

    if (payload === undefined) {
      await route.fulfill({
        status: 404,
        json: { error: `Unhandled test API path: ${url.pathname}` },
      });
      return;
    }

    await route.fulfill({ json: payload });
  });

  return requests;
}

async function toggleOrganizer(page, trigger, name) {
  const option = page.getByRole('listbox').getByRole('option', { name });

  if (!(await option.isVisible())) {
    await trigger.click();
  }

  await expect(option).toBeVisible();
  await option.click();
}

test('a slow ancestor response cannot overwrite the latest cascade selection', async ({ page }) => {
  const requests = await mockCascadeApi(page);

  await page.goto('/');
  const filters = page.locator('#home-cascade-filters');
  const organizerTrigger = filters
    .locator('select[name="organizer"]')
    .locator('..')
    .locator('[data-custom-select-trigger]');

  await toggleOrganizer(page, organizerTrigger, 'Organizer A');
  await toggleOrganizer(page, organizerTrigger, 'Organizer A');
  await toggleOrganizer(page, organizerTrigger, 'Organizer B');

  const competitionTrigger = filters
    .locator('select[name="competition"]')
    .locator('..')
    .locator('[data-custom-select-trigger]');
  await expect(competitionTrigger).toBeEnabled();
  await expect(competitionTrigger).toContainText('Select competitions');
  await competitionTrigger.click();

  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Competition B', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Competition A', exact: true }),
  ).toHaveCount(0);

  await page.waitForTimeout(700);

  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Competition B', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Competition A', exact: true }),
  ).toHaveCount(0);
  expect(
    requests.some(
      (request) =>
        request.pathname === '/api/competitions/options' && request.searchParams.get('organizer_ids') === '1',
    ),
  ).toBe(true);
  expect(
    requests.some(
      (request) =>
        request.pathname === '/api/competitions/options' && request.searchParams.get('organizer_ids') === '2',
    ),
  ).toBe(true);
});

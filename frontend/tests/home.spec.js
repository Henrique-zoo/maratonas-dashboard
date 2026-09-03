import { expect, test } from '@playwright/test';

const organizerOptions = [{ id: 300, name: 'Mock Organizer' }];
const competitionOptions = [{ id: 1, name: 'Mock Competition' }];
const institutionOptions = [{ id: 200, name: 'Mock University' }];
const teamOptions = [
  {
    id: 100,
    name: 'Mock Team',
    institution_id: 200,
    institution_name: 'Mock University',
    institution_short_name: 'MU',
  },
];

const teamEntry = {
  id: 100,
  institution_id: 200,
  name: 'Mock Team',
  rank: 1,
  institution_name: 'Mock University',
  institution_short_name: 'MU',
  total_members: 3,
  female_participants: 1,
};

const runnerUpTeamEntry = {
  id: 101,
  institution_id: 200,
  name: 'Runner Up Team',
  rank: 2,
  institution_name: 'Mock University',
  institution_short_name: 'MU',
  total_members: 3,
  female_participants: 0,
};

const eventEntry = {
  id: 10,
  name: 'Mock Event',
  date: '2025-05-10',
  location: 'Sao Paulo',
  location_types: ['Country', 'City'],
  total_teams: 2,
  total_participants: 6,
  female_participants: 1,
  teams: [teamEntry, runnerUpTeamEntry],
};

const secondEventEntry = {
  id: 11,
  name: 'Mock Final',
  date: '2025-06-15',
  location: 'Rio de Janeiro',
  location_types: ['Country', 'City'],
  total_teams: 1,
  total_participants: 3,
  female_participants: 1,
  teams: [teamEntry],
};

const previousYearEventEntry = {
  id: 12,
  name: 'Mock Previous Regional',
  date: '2024-05-08',
  location: 'Campinas',
  location_types: ['Country', 'City'],
  total_teams: 1,
  total_participants: 3,
  female_participants: 1,
  teams: [teamEntry],
};

const independentEventEntry = {
  ...eventEntry,
  id: 20,
  name: 'Independent Event',
  date: '2023-09-20',
};

const competitionStructures = [
  {
    id: 1,
    name: 'Mock Competition',
    gender_category: 'mixed',
    website_url: 'https://example.test/mock-competition',
    years: [2024, 2025],
    snapshot_year: 2025,
    location_types: ['Country', 'City'],
    events: [eventEntry, secondEventEntry],
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

const competitionLocationStats = [
  {
    id: 1,
    name: 'Brazil',
    total_institutions: 1,
    total_teams: 2,
    total_participants: 6,
    female_participants: 1,
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
        gender_category: 'Open',
        website_url: 'https://example.test/mock-competition',
        years: [2024, 2025],
        snapshot_year: 2025,
        location_types: ['Country', 'City'],
        events: [eventEntry, secondEventEntry],
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
        years: [2024, 2025],
        snapshot_year: 2025,
        events: [eventEntry, secondEventEntry],
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
        years: [2024, 2025],
        snapshot_year: 2025,
        total_members: 3,
        female_participants: 1,
        events: [eventEntry],
      },
      {
        id: 2,
        name: 'Independent Competition',
        years: [2023],
        snapshot_year: 2023,
        total_members: 3,
        female_participants: 1,
        events: [independentEventEntry],
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
  ['/api/competitions/1/structure', competitionYearStructure],
  ['/api/competitions/1/stats', competitionStats],
  ['/api/competitions/1/location-stats', competitionLocationStats],
  ['/api/events/10/stats', eventStats],
  ['/api/events/10/location-stats', competitionLocationStats],
  ['/api/organizers/structures', organizerStructures],
  ['/api/organizers/competitions/1/structure', competitionYearStructure],
  ['/api/institutions/structures', institutionStructures],
  ['/api/teams/structures', teamStructures],
]);

async function mockApi(page, overrides = new Map()) {
  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());
    const response = overrides.has(url.pathname)
      ? overrides.get(url.pathname)
      : responsesByPath.get(url.pathname);
    const body = typeof response === 'function' ? response(url) : response;

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

async function expectOptionAtTopLayer(page, optionName) {
  const option = page.getByRole('listbox').getByRole('option', { name: optionName, exact: true });

  await expect(option).toBeVisible();
  const isTopLayer = await option.evaluate((element) => {
    const rect = element.getBoundingClientRect();
    const topElement = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);

    return element === topElement || element.contains(topElement);
  });

  expect(isTopLayer).toBe(true);
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
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Select organizers', exact: true })).toBeVisible();

  apiRequests.length = 0;
  await page.getByRole('button', { name: 'Select organizers', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Organizer', exact: true }).click();

  await page.getByRole('button', { name: 'Select competitions', exact: true }).click();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Mock Competition', exact: true }),
  ).toBeVisible();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Competition', exact: true }).click();

  await page.getByRole('button', { name: 'Select institutions', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock University', exact: true }).click();

  await page.getByRole('button', { name: 'Select teams', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Team', exact: true }).click();

  await expect(page).toHaveURL(/\/$/);
  expect(apiRequests.some((url) => new URL(url).pathname === '/api/institutions/options')).toBe(true);
  expect(apiRequests.some((url) => new URL(url).pathname === '/api/teams/options')).toBe(true);

  await page.getByRole('button', { name: 'Apply Filters' }).click();

  await expect(page).toHaveURL(/\?organizer=300&competition=1&institution=200&team=100$/);
  await expect(page.getByText('Organizer: Mock Organizer')).toBeVisible();
  await expect(page.getByText('Showing Mock Team.')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Mock Team' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Competitions' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Independent Competition', exact: true })).toBeVisible();
  await expect(page.getByRole('link', { name: /Mock Event/ }).first()).toBeVisible();
});

test('competition filter renders year controls, location drawer and event rankings', async ({ page }) => {
  const apiRequests = [];

  await mockApi(page);
  page.on('request', (request) => {
    const url = new URL(request.url());

    if (url.pathname.startsWith('/api/')) {
      apiRequests.push(url);
    }
  });

  await page.goto('/');

  await page.getByRole('button', { name: 'Select organizers', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Organizer', exact: true }).click();
  await page.getByRole('button', { name: 'Select competitions', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Competition', exact: true }).click();
  await page.getByRole('button', { name: 'Apply Filters' }).click();

  await expect(page).toHaveURL(/\?organizer=300&competition=1$/);
  await expect(page.getByText('Showing Mock Competition in 2025.')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Events' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Team rankings inside each event' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /Mock Event/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /Mock Final/ })).toBeVisible();
  await expect(page.getByRole('cell', { name: 'Runner Up Team' })).toHaveCount(0);

  await page.getByRole('button', { name: /Mock Event/ }).click();

  await expect(page.locator('#competition-event-ranking-10').getByText('#1').first()).toBeVisible();
  await expect(
    page.locator('#competition-event-ranking-10').getByRole('cell', { name: 'Runner Up Team', exact: true }),
  ).toBeVisible();
  await expect(page.getByText('Locations', { exact: true })).toBeVisible();
  await expect(page.getByText('Breakdown')).toHaveCount(0);

  const yearForm = page.locator('#home-competition-year-filter');
  await expect(yearForm.getByRole('button', { name: 'Year 2025', exact: true })).toBeVisible();
  await yearForm.getByRole('button', { name: 'Year 2025', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Year 2024', exact: true }).click();
  await yearForm.getByRole('button', { name: 'Apply' }).click();

  await expect(page).toHaveURL(/\?organizer=300&competition=1&year=2024$/);
  await expect(page.getByRole('button', { name: /Mock Previous Regional/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /Mock Final/ })).toHaveCount(0);
  expect(
    apiRequests.some(
      (url) => url.pathname === '/api/competitions/1/structure' && url.searchParams.get('year') === '2024',
    ),
  ).toBe(true);
});

test('organizer filter renders organizer data on the home index', async ({ page }) => {
  const apiRequests = [];

  await mockApi(page);
  page.on('request', (request) => {
    const url = new URL(request.url());

    if (url.pathname.startsWith('/api/')) {
      apiRequests.push(url);
    }
  });

  await page.goto('/');

  await page.getByRole('button', { name: 'Select organizers', exact: true }).click();
  await page.getByRole('listbox').getByRole('option', { name: 'Mock Organizer', exact: true }).click();
  await page.getByRole('button', { name: 'Apply Filters' }).click();

  await expect(page).toHaveURL(/\?organizer=300$/);
  await expect(page.getByText('Showing Mock Organizer.')).toBeVisible();
  await expect(page.locator('.organizer-inspector')).toHaveCount(0);

  const organizerResult = page.locator('[data-structure-result]').filter({ hasText: 'Mock Organizer' });
  const competitionCard = organizerResult
    .locator('.organizer-competition-card')
    .filter({ hasText: 'Mock Competition' });

  await expect(competitionCard).toBeVisible();
  await expect(
    competitionCard.getByRole('link', {
      name: 'Open Mock Competition website',
    }),
  ).toHaveAttribute('href', 'https://example.test/mock-competition');
  const websiteIconBox = await competitionCard
    .getByRole('link', { name: 'Open Mock Competition website' })
    .boundingBox();

  expect(websiteIconBox?.width).toBeGreaterThan(20);
  expect(websiteIconBox?.height).toBeGreaterThan(20);
  await expect(competitionCard.getByRole('link', { name: /Mock Event/ })).toBeVisible();
  await expect(competitionCard.getByRole('link', { name: /Mock Final/ })).toBeVisible();

  apiRequests.length = 0;
  await competitionCard.evaluate((element) => element.scrollIntoView({ block: 'center', inline: 'nearest' }));
  const cardBox = await competitionCard.boundingBox();
  const headerBox = await page.locator('.topbar').boundingBox();

  if (!cardBox) {
    throw new Error('Competition card is not visible');
  }

  await page.mouse.click(
    cardBox.x + 20,
    Math.min(
      Math.max(cardBox.y + 120, (headerBox?.y || 0) + (headerBox?.height || 0) + 24),
      cardBox.y + cardBox.height - 24,
    ),
  );

  await expect(page).toHaveURL(/\?organizer=300$/);
  expect(apiRequests.some((url) => new URL(url).pathname === '/api/competitions/1/stats')).toBe(true);
  await expect(page.getByText('Showing Mock Organizer.')).toBeVisible();
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
  await expect(page.locator('.organizer-inspector__drawer > summary strong')).toHaveText('Annual totals');

  apiRequests.length = 0;
  await competitionCard.getByRole('link', { name: /Mock Event/ }).click();

  await expect(page).toHaveURL(/\?organizer=300$/);
  expect(apiRequests.some((url) => new URL(url).pathname === '/api/events/10/stats')).toBe(true);
  await expect(page.locator('.organizer-inspector').getByText('Event details')).toBeVisible();
  await expect(
    page.locator('.organizer-inspector').getByRole('heading', { name: 'Mock Event' }),
  ).toBeVisible();

  await page
    .locator('.organizer-inspector')
    .getByRole('button', { name: 'Select location', exact: true })
    .click();
  await page.getByRole('listbox').getByRole('option', { name: 'Country', exact: true }).click();

  await expect(page).toHaveURL(/\?organizer=300$/);
  await expect(page.locator('.organizer-inspector').getByText('Country split')).toBeVisible();
  await expect(page.locator('.organizer-inspector').getByText('Brazil')).toBeVisible();
  expect(
    apiRequests.some(
      (url) =>
        url.pathname === '/api/events/10/location-stats' &&
        url.searchParams.get('location_type') === 'Country',
    ),
  ).toBe(true);

  const yearForm = competitionCard.locator('.organizer-competition-year-form');
  apiRequests.length = 0;
  await yearForm.getByRole('button', { name: '2025', exact: true }).click();
  await expectOptionAtTopLayer(page, '2024');
  await page.getByRole('listbox').getByRole('option', { name: '2024', exact: true }).click();

  await expect(page).toHaveURL(/\?organizer=300$/);
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

test('a full home deep link survives reload and browser history', async ({ page }) => {
  let organizerOptionRequests = 0;

  await mockApi(page);
  page.on('request', (request) => {
    if (new URL(request.url()).pathname === '/api/organizers/options') {
      organizerOptionRequests += 1;
    }
  });

  const deepLink = '/?organizer=300&competition=1&institution=200&team=100';
  await page.goto(deepLink);

  await expect(page).toHaveURL(new RegExp(`${deepLink.replaceAll('?', '\\?')}$`));
  await expect(page.getByText('Showing Mock Team.')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Mock Team' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'Independent Competition', exact: true })).toBeVisible();

  await page.reload();

  await expect(page).toHaveURL(new RegExp(`${deepLink.replaceAll('?', '\\?')}$`));
  await expect(page.getByText('Showing Mock Team.')).toBeVisible();
  await expect(page.getByRole('link', { name: 'Independent Competition', exact: true })).toBeVisible();

  await page.locator('#home-cascade-filters').getByRole('link', { name: 'Clear' }).click();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();

  await page.goBack();
  await expect(page).toHaveURL(new RegExp(`${deepLink.replaceAll('?', '\\?')}$`));
  await expect(page.getByText('Showing Mock Team.')).toBeVisible();

  await page.goForward();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(organizerOptionRequests).toBe(2);
});

test('changing an ancestor filter clears every dependent local selection', async ({ page }) => {
  await mockApi(page);
  await page.goto('/');

  const filters = page.locator('#home-cascade-filters');
  const triggerFor = (name) =>
    filters.locator(`select[name="${name}"]`).locator('..').locator('[data-custom-select-trigger]');
  const toggleOption = async (name, optionName) => {
    const trigger = triggerFor(name);
    const option = page.getByRole('listbox').getByRole('option', { name: optionName });

    if (!(await option.isVisible())) {
      await trigger.click();
    }

    await expect(option).toBeVisible();
    await option.click();
  };

  await toggleOption('organizer', 'Mock Organizer');
  await expect(triggerFor('competition')).toBeEnabled();
  await toggleOption('competition', 'Mock Competition');
  await expect(triggerFor('institution')).toBeEnabled();
  await toggleOption('institution', 'Mock University');
  await expect(triggerFor('team')).toBeEnabled();
  await toggleOption('team', 'Mock Team');

  await toggleOption('competition', 'Mock Competition');

  await expect(triggerFor('competition')).toContainText('Select competitions');
  await expect(triggerFor('institution')).toContainText('Select competition first');
  await expect(triggerFor('institution')).toBeDisabled();
  await expect(triggerFor('team')).toContainText('Select institution first');
  await expect(triggerFor('team')).toBeDisabled();

  await toggleOption('competition', 'Mock Competition');
  await toggleOption('institution', 'Mock University');
  await toggleOption('team', 'Mock Team');
  await toggleOption('organizer', 'Mock Organizer');

  await expect(triggerFor('competition')).toContainText('Select organizer first');
  await expect(triggerFor('competition')).toBeDisabled();
  await expect(triggerFor('institution')).toContainText('Select competition first');
  await expect(triggerFor('institution')).toBeDisabled();
  await expect(triggerFor('team')).toContainText('Select institution first');
  await expect(triggerFor('team')).toBeDisabled();

  await filters.getByRole('button', { name: 'Apply Filters' }).click();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
});

test('team names include the institution acronym only when multiple institutions are selected', async ({
  page,
}) => {
  const duplicateInstitutionOptions = [...institutionOptions, { id: 201, name: 'Second University' }];
  const duplicateTeamOptions = [
    {
      id: 100,
      name: 'Shared Team',
      institution_id: 200,
      institution_name: 'Mock University',
      institution_short_name: 'MU',
    },
    {
      id: 101,
      name: 'Shared Team',
      institution_id: 201,
      institution_name: 'Second University',
      institution_short_name: 'SU',
    },
  ];
  const overrides = new Map([
    ['/api/institutions/options', duplicateInstitutionOptions],
    [
      '/api/teams/options',
      (url) => {
        const institutionIds = (url.searchParams.get('institution_ids') || '')
          .split(',')
          .filter(Boolean)
          .map(Number);

        return institutionIds.length
          ? duplicateTeamOptions.filter((option) => institutionIds.includes(option.institution_id))
          : duplicateTeamOptions;
      },
    ],
  ]);

  await mockApi(page, overrides);
  await page.goto('/?organizer=300&competition=1&institution=200,201');

  const filters = page.locator('#home-cascade-filters');
  const teamTrigger = filters
    .locator('select[name="team"]')
    .locator('..')
    .locator('[data-custom-select-trigger]');
  const institutionTrigger = filters
    .locator('select[name="institution"]')
    .locator('..')
    .locator('[data-custom-select-trigger]');

  await teamTrigger.click();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Shared Team (MU)', exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Shared Team (SU)', exact: true }),
  ).toBeVisible();

  await page.keyboard.press('Escape');
  await institutionTrigger.click();
  await page.getByRole('listbox').getByRole('option', { name: 'Second University' }).click();
  await page.keyboard.press('Escape');
  await expect(filters.locator('select[name="team"] option')).toHaveText(['Shared Team']);

  await teamTrigger.click();
  await expect(
    page.getByRole('listbox').getByRole('option', { name: 'Shared Team', exact: true }),
  ).toBeVisible();
  await expect(page.getByRole('listbox').getByRole('option', { name: /Shared Team \(/ })).toHaveCount(0);
});

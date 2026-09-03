import { expect, test } from '@playwright/test';

const emptyOptionPaths = new Set([
  '/api/organizers/options',
  '/api/competitions/options',
  '/api/institutions/options',
  '/api/teams/options',
]);

async function mockEmptyUniverse(page, { organizerGate = null, failOrganizerOnce = null } = {}) {
  const requests = [];
  let organizerAttempts = 0;

  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());
    requests.push(url);

    if (!emptyOptionPaths.has(url.pathname)) {
      await route.fulfill({
        status: 404,
        json: { error: `Unhandled test API path: ${url.pathname}` },
      });
      return;
    }

    if (url.pathname === '/api/organizers/options') {
      organizerAttempts += 1;

      if (organizerGate) {
        await organizerGate;
      }

      if (failOrganizerOnce && organizerAttempts === 1) {
        await route.fulfill({ status: failOrganizerOnce.status, json: failOrganizerOnce.body });
        return;
      }
    }

    await route.fulfill({ json: [] });
  });

  return {
    count(pathname) {
      return requests.filter((request) => request.pathname === pathname).length;
    },
  };
}

test('shows a loading state until the initial catalog is available', async ({ page }) => {
  let releaseOrganizer;
  const organizerGate = new Promise((resolve) => {
    releaseOrganizer = resolve;
  });
  await mockEmptyUniverse(page, { organizerGate });

  await page.goto('/');

  await expect(page.locator('.loading-state')).toBeVisible();
  await expect(page.getByText('Pulling the latest competition ledger…')).toBeVisible();

  releaseOrganizer();
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
});

test('renders escaped API errors and retries a rejected snapshot', async ({ page }) => {
  const api = await mockEmptyUniverse(page, {
    failOrganizerOnce: {
      status: 503,
      body: { error: '<img src=x onerror=alert(1)> service unavailable' },
    },
  });

  await page.goto('/');

  await expect(page).toHaveTitle('Something Went Wrong | MD Stack');
  await expect(page.locator('.empty-state--error')).toContainText(
    '<img src=x onerror=alert(1)> service unavailable',
  );
  await expect(page.locator('.empty-state--error img')).toHaveCount(0);

  await page.getByRole('link', { name: 'Return home' }).click();

  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(api.count('/api/organizers/options')).toBe(2);
});

test('renders an empty universe without enabling dependent filters', async ({ page }) => {
  await mockEmptyUniverse(page);

  await page.goto('/');

  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  await expect(page.getByText('0 organizers', { exact: true })).toBeVisible();
  await expect(page.getByText('No competitions', { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Select organizer first', exact: true })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Select competition first', exact: true })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Select institution first', exact: true })).toBeDisabled();
});

test('handles an unknown route without invoking a page-specific endpoint', async ({ page }) => {
  const apiRequests = [];
  await page.route('**/api/**', async (route) => {
    apiRequests.push(new URL(route.request().url()).pathname);
    await route.fulfill({ json: [] });
  });

  await page.goto('/route-that-does-not-exist');

  await expect(page).toHaveTitle('Page Not Found | MD Stack');
  await expect(page.locator('.empty-state--error')).toContainText(
    'The requested route does not exist in this dashboard.',
  );
  expect(apiRequests.sort()).toEqual(
    [
      '/api/competitions/options',
      '/api/institutions/options',
      '/api/organizers/options',
      '/api/teams/options',
    ].sort(),
  );
});

test('reuses the home snapshot during SPA history and rebuilds it after reload', async ({ page }) => {
  const api = await mockEmptyUniverse(page);

  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(api.count('/api/organizers/options')).toBe(1);

  await page.locator('#home-cascade-filters').getByRole('link', { name: 'Clear' }).click();
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(api.count('/api/organizers/options')).toBe(1);

  await page.goBack();
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(api.count('/api/organizers/options')).toBe(1);

  await page.reload();
  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();
  expect(api.count('/api/organizers/options')).toBe(2);
});

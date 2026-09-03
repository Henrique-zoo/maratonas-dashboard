import { expect, test } from '@playwright/test';

test('a slow route cannot overwrite a newer SPA navigation', async ({ page }) => {
  await page.route('**/api/**', async (route) => {
    const url = new URL(route.request().url());

    if (url.pathname === '/api/organizers/options') {
      await new Promise((resolve) => setTimeout(resolve, 600));
      await route.fulfill({ json: [] });
      return;
    }

    if (
      ['/api/competitions/options', '/api/institutions/options', '/api/teams/options'].includes(url.pathname)
    ) {
      await route.fulfill({ json: [] });
      return;
    }

    await route.fulfill({
      status: 404,
      json: { error: `Unhandled test API path: ${url.pathname}` },
    });
  });

  await page.goto('/organizers');
  await expect(page.locator('.loading-state')).toBeVisible();

  await page.locator('.primary-nav').getByRole('link', { name: 'Teams', exact: true }).click();

  await expect(page).toHaveURL(/\/teams$/);
  await expect(page.getByRole('heading', { name: 'Teams', exact: true })).toBeVisible();
  await page.waitForTimeout(700);
  await expect(page).toHaveURL(/\/teams$/);
  await expect(page.getByRole('heading', { name: 'Teams', exact: true })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Organizers', exact: true })).toHaveCount(0);
});

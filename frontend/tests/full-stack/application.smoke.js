import { expect, test } from '@playwright/test';

async function selectOption(page, triggerName, optionName) {
  const trigger = page.getByRole('button', { name: triggerName, exact: true });

  await expect(trigger).toBeEnabled();
  await trigger.click();

  const option = page.getByRole('listbox').getByRole('option', {
    name: optionName,
    exact: true,
  });

  await expect(option).toBeVisible();
  await option.click();
}

test('the browser reaches the real API and PostgreSQL through the production frontend', async ({ page }) => {
  await page.goto('/');

  await expect(page.getByRole('heading', { name: 'Competition overview' })).toBeVisible();

  await selectOption(page, 'Select organizers', 'Algorithm League');
  await selectOption(page, 'Select competitions', 'Global Algorithm Cup');
  await selectOption(page, 'Select institutions', 'Alpha University');
  await selectOption(page, 'Select teams', 'Alpha Coders');

  await page.getByRole('button', { name: 'Apply Filters' }).click();

  await expect(page).toHaveURL(/\?organizer=1&competition=10&institution=1&team=1$/);

  const result = page.locator('[data-structure-result]');
  await expect(result.getByRole('heading', { name: 'Alpha Coders', exact: true })).toBeVisible();
  const competitionLink = result.getByRole('link', { name: 'Global Algorithm Cup', exact: true });
  await expect(competitionLink).toBeVisible();
  await expect(result.getByRole('link', { name: 'Regional Code League', exact: true })).toBeVisible();

  await page.goto('/competitions/10');

  await expect(page).toHaveURL(/\/competitions\/10/);
  await expect(page.getByRole('heading', { name: 'Annual participant totals' })).toBeVisible();
  await expect(page.getByRole('img', { name: 'Distinct participants over time' })).toBeVisible();
  await expect(page.getByRole('figure', { name: 'Participants by location' })).toBeVisible();
});

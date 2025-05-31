import { expect, test } from '@playwright/test'
import { resetDb } from '../../helpers'

test.beforeEach(async () => {
  await resetDb()
})

test('logs in', async ({ page }) => {
  page.on('console', (msg) => console.log(`Console: ${msg.type()} - ${msg.text()}`))
  await page.goto('/')

  await page.locator('.login-button-wrap').click()

  await expect(page.locator('.auth-title')).toContainText('Log In')

  await page.getByPlaceholder('Email').fill('user1@samatech.tw')
  await page.getByPlaceholder('Password').fill('password1')

  await page.locator('.login-button').click()

  await expect(page.locator('.user-menu .avatar-image')).toBeVisible()
})

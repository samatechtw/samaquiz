import { expect, Page } from '@playwright/test'

export interface IE2EUser {
  email: string
  password: string
}

export const user1: IE2EUser = {
  email: 'user1@samatech.tw',
  password: 'password1',
}

export const doLogin = async (page: Page, user: IE2EUser): Promise<void> => {
  await page.goto('/')
  await page.locator('.login-button-wrap').click()
  await expect(page.locator('.auth-title')).toContainText('Log In')
  await page.getByPlaceholder('Email').fill(user.email)
  await page.getByPlaceholder('Password').fill(user.password)
  await page.locator('.login-button').click()
  await expect(page.locator('.user-menu .avatar-image')).toBeVisible()
}

export const doSignUp = async (
  page: Page,
  email: string,
  password: string,
): Promise<void> => {
  await page.goto('/')

  await page.locator('.sign-up-button-wrap').click()

  const submitButton = page.locator('.submit-button')
  await expect(page.locator('.auth-title')).toContainText('Sign Up')
  await expect(submitButton).toBeDisabled()

  await page.getByPlaceholder('Email').fill(email)
  await page.getByPlaceholder('Password').fill(password)
  await page.locator('.terms-checkbox-wrap .checkbox').click()
  await expect(submitButton).not.toBeDisabled()

  await submitButton.click()
  await expect(submitButton).toBeDisabled()
  await expect(submitButton.locator('.p-button-animate')).toBeVisible()
}

import { expect, Page } from '@playwright/test'

export interface ITestAnswer {
  text: string
  correct?: boolean
}

export const joinQuiz = async (page: Page, quiz: string, name: string): Promise<void> => {
  await page.goto(`/q/${quiz}`)
  await page.getByPlaceholder('Name').fill(name)
  await page.locator('.cat-wrap').nth(3).click()
  await expect(page.locator('.cat-wrap').nth(3)).toHaveClass('cat-wrap f-center selected')
  await page.locator('.name-wrap .ready ').click()
  await expect(page.locator('.question-countdown')).toBeVisible()
}

export const answerQuestion = async (page: Page, answer: ITestAnswer): Promise<void> => {
  const answerLoc = page.locator('.answers').getByText(answer.text)
  await answerLoc.click()
  if (answer.correct) {
    await expect(answerLoc).toHaveClass(/ correct /)
  } else {
    await expect(answerLoc).toHaveClass(/ incorrect /)
  }
  await expect(page.locator('.quiz-question .wait-host')).toHaveText(
    'Waiting for Host...',
  )
}

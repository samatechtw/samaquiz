import { type Page, expect } from '@playwright/test'
import {
  doLogin,
  resetDb,
  test4,
  user1,
  IMultiParticipant,
  joinQuiz,
  answerQuestion,
} from '../../helpers'

test4.beforeEach(async () => {
  await resetDb()
})

const assertLeaders = async (page: Page, leaders: string[]): Promise<void> => {
  for (let i = 0; i < leaders.length; i += 1) {
    const leader = leaders[i]
    const indexLoc = page
      .locator('.leaders-wrap')
      .getByText(leader)
      .locator('..')
      .locator('.leader-index')
    await expect(indexLoc).toHaveText(`${i + 1}.`)
  }
}

const questionComplete = async (page: Page, leaders: string[]) => {
  await expect(page.locator('.countdown-wrap .count')).toHaveText('3 / 4')
  await page.locator('.countdown-wrap .results-button').click()
  await assertLeaders(page, leaders)
  await page.locator('.question-results .results-next').click()
}

test4('completes quiz with 3 participants', async ({ page, page2, page3, page4 }) => {
  const quiz = 'activequiz'
  await doLogin(page, user1)
  await page.goto(`/q/${quiz}`)

  const u2: IMultiParticipant = { page: page2, name: 'User2' }
  const u3: IMultiParticipant = { page: page3, name: 'User3' }
  const u4: IMultiParticipant = { page: page4, name: 'User4' }
  const participants = [u2, u3, u4]
  for (const p of participants) {
    await joinQuiz(p.page, quiz, p.name)
  }
  // Answer first question (all correct)
  const q1a1 = { text: 'Q1 Answer 1', correct: true }
  const q1a2 = { text: 'Q1 Answer 2', correct: false }
  const q1a3 = { text: 'Q1 Answer 3', correct: false }
  await answerQuestion(u2.page, q1a1)
  await answerQuestion(u3.page, q1a2)
  await answerQuestion(u4.page, q1a3)

  await questionComplete(page, [u2.name])

  // Answer second question
  const q2a1 = { text: 'Q2 Answer 1', correct: true }
  const q2a2 = { text: 'Q2 Answer 2', correct: false }
  await answerQuestion(u2.page, q2a1)
  await answerQuestion(u3.page, q2a1)
  await answerQuestion(u4.page, q2a2)

  await questionComplete(page, [u2.name, u3.name])

  // Answer third question
  const q3a1 = { text: 'Q3 Answer 1', correct: false }
  const q3a2 = { text: 'Q3 Answer 2', correct: true }
  await answerQuestion(u2.page, q3a1)
  await answerQuestion(u3.page, q3a1)
  await answerQuestion(u4.page, q3a2)

  await questionComplete(page, [u2.name, u3.name, u4.name])

  // Answer fourth question
  const q4a1 = { text: 'Q4 Answer 1', correct: true }
  const q4a2 = { text: 'Q4 Answer 2', correct: false }
  await answerQuestion(u2.page, q4a2)
  await answerQuestion(u3.page, q4a2)
  await answerQuestion(u4.page, q4a1)

  await questionComplete(page, [u2.name, u4.name, u3.name])

  await expect(page.locator('.quiz-complete .title')).toHaveText('Quiz Complete!')
  await assertLeaders(page, [u2.name, u4.name, u3.name])
})

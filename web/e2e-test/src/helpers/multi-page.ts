import { Browser, type Page, test as base } from '@playwright/test'

export interface IMultiParticipant {
  page: Page
  name: string
}

interface Test2Fixture {
  page2: Page
}

interface Test3Fixture extends Test2Fixture {
  page3: Page
}

interface Test4Fixture extends Test3Fixture {
  page4: Page
}

async function makeIsolatedPage(browser: Browser, use: (page: Page) => Promise<void>) {
  const ctx = await browser.newContext()
  const page = await ctx.newPage()
  try {
    await use(page)
  } finally {
    await ctx.close()
  }
}

export const test2 = base.extend<Test2Fixture>({
  page2: async ({ browser }, use) => {
    await makeIsolatedPage(browser, use)
  },
})

export const test3 = test2.extend<Test3Fixture>({
  page3: async ({ browser }, use) => {
    await makeIsolatedPage(browser, use)
  },
})

export const test4 = test3.extend<Test4Fixture>({
  page4: async ({ browser }, use) => {
    await makeIsolatedPage(browser, use)
  },
})

import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './src/test',
  // Run tests in files in parallel
  fullyParallel: true,
  // Fail the build on CI if test.only is detected
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : 1,
  reporter: 'html',
  outputDir: './web/e2e-test/dist',
  // Shared test settings. See https://playwright.dev/docs/api/class-testoptions
  timeout: process.env.CI ? 30000 : 20000,
  expect: {
    timeout: process.env.CI ? 5000 : 3000,
  },
  use: {
    actionTimeout: process.env.CI ? 5000 : 3000,
    // Web app is served in preview mode in CI environment
    baseURL: process.env.CI ? 'http://127.0.0.1:8081' : 'http://127.0.0.1:8080',

    // Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
})

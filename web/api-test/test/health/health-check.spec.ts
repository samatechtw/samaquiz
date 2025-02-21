import { testagent, TestAgent } from '../helpers'
import { testConfig } from '../test.config'
import { describe, test, beforeAll } from 'vitest'

describe('Health check endpoint', () => {
  let api: TestAgent

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
  })

  test('returns 200 status code and message', () => {
    return api.get('/api/healthz').expect(200, 'Endpoint is healthy')
  })
})

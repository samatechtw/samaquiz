import supertest from 'supertest'

/* eslint-disable @typescript-eslint/no-explicit-any */

declare module 'supertest' {
  interface Test {
    _assert(this: supertest.Test, resError: Error, res: supertest.Response, fn: any): void
  }
}

const sup = supertest as any
if (!sup.Test.prototype.__assert_updated) {
  sup.Test.prototype.__assert_updated = true

  Object.defineProperties(sup.Test.prototype, {
    _assert: {
      value: sup.Test.prototype.assert,
    },
    assert: {
      value: function (
        this: supertest.Test,
        resError: Error,
        res: supertest.Response,
        fn: any,
      ) {
        this._assert(resError, res, (err: Error, res: supertest.Response) => {
          if (err) {
            const originalMessage = err.message
            err.message = `${err.message}\nbody: ${JSON.stringify(res.body, null, 2)}`
            // Must update the stack trace as what supertest prints is the stacktrace
            err.stack = err.stack?.replace(originalMessage, err.message)
          }
          fn.call(this, err, res)
        })
      },
    },
  })
}

/* eslint-enable @typescript-eslint/no-explicit-any */

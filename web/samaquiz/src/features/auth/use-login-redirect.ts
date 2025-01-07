import { store } from '@frontend/store'
import { watch } from 'vue'
import { LocationQuery, RouteLocation, useRoute, useRouter } from 'vue-router'

export interface UseLoginRedirectOptions {
  defaultRedirect?: string
  loginPage?: string
}

interface RouteQueryAndHash {
  query?: LocationQuery
  hash?: string
}

export interface RedirectToLoginOptions {
  replace?: boolean
  // Route to redirect to after login. Defaults to current path.
  destination?: RouteQueryAndHash
}

export interface LoginRedirectInterface {
  redirectToLogin: (options?: RedirectToLoginOptions) => void
  currentRedirect(): string | null
  redirectAfterLogin: () => void
  checkAuthRedirect: (routeObj?: RouteLocation) => void
  watchAuthRedirect: () => void
}

export const useLoginRedirect = (
  options?: UseLoginRedirectOptions,
): LoginRedirectInterface => {
  const route = useRoute()
  const router = useRouter()
  const { defaultRedirect = 'Profile', loginPage = 'Login' } = options ?? {}

  // Store the current URL and go to the login page
  let redirect: string
  const redirectToLogin = (options?: RedirectToLoginOptions) => {
    if (options?.destination) {
      redirect = JSON.stringify(options.destination)
      store.auth.setLoginRedirect(redirect)
    } else {
      const { hash, name, params, query } = route
      redirect = JSON.stringify({ hash, name, params, query })
      store.auth.setLoginRedirect(redirect)
    }
    const query = {
      ...options?.destination?.query,
      from: btoa(redirect),
    }
    if (options?.replace) {
      router.replace({ name: loginPage, query })
    } else {
      router.push({ name: loginPage, query })
    }
  }

  const currentRedirect = (): string | null => {
    // Precedence goes to `from` query param
    const from = route.query.from as string | undefined
    return from ? atob(from) : store.auth.loginRedirect.value
  }

  // Return to the stored URL after authenticating
  const redirectAfterLogin = () => {
    const redirect = currentRedirect()

    store.auth.setLoginRedirect(null)
    if (redirect) {
      if (redirect.startsWith('/')) {
        // A raw path redirect was used
        const newRoute = router.resolve(redirect)
        // Make sure we don't remain on the Login page
        if (newRoute?.name === 'Login') {
          return router.push({ name: defaultRedirect ?? 'Profile' })
        } else {
          return router.push({ path: redirect })
        }
      } else {
        try {
          return router.push(JSON.parse(redirect))
        } catch (e) {
          console.log('Redirect error:', e)
          // If something went wrong, go Home
          return router.push({ name: defaultRedirect ?? 'Home' })
        }
      }
    } else {
      return router.push({ name: defaultRedirect ?? 'Profile' })
    }
  }

  // Check if auth redirect is required when route or auth status change
  const watchAuthRedirect = () => {
    watch(
      route,
      (newRoute) => {
        checkAuthRedirect(newRoute)
      },
      { immediate: true },
    )
    watch(store.auth.loggedIn, () => {
      checkAuthRedirect()
    })
  }

  // Redirect to login if the current page requires it, and the user is not authenticated
  const checkAuthRedirect = (routeObj?: RouteLocation) => {
    const targetRoute = routeObj ?? route
    if (store.auth.loggedIn.value) {
      return
    }
    const routeRequiresAuth = targetRoute?.meta?.requiresAuth
    const parentRequiresAuth = targetRoute.matched.some(
      (record) => !!record.meta.requiresAuth,
    )
    const requiresAuth = routeRequiresAuth || parentRequiresAuth
    if (requiresAuth) {
      redirectToLogin({ replace: true })
    }
  }

  return {
    redirectToLogin,
    currentRedirect,
    redirectAfterLogin,
    checkAuthRedirect,
    watchAuthRedirect,
  }
}

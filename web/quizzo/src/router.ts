import { createRouter, createWebHistory } from 'vue-router'

declare module 'vue-router' {
  interface RouteMeta {
    title?: string
    noScroll?: boolean
    scrollAnchor?: string
    requiresAuth?: boolean
  }
}

const metaAuth = (title: string) => ({
  title,
  requiresAuth: true,
})

const router = createRouter({
  history: createWebHistory(),
  scrollBehavior(to, from, savedPosition) {
    if (to?.hash) {
      return { el: to.hash }
    }
    if (savedPosition) {
      return savedPosition
    }
    if (to?.meta?.noScroll && from?.meta?.noScroll) {
      return {}
    }
    return { top: 0 }
  },
  routes: [
    {
      path: '',
      name: 'Home',
      component: () => import('./views/HomePage.vue'),
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('./views/LoginPage.vue'),
      meta: { title: 'Login' },
    },
    {
      path: '/signup',
      name: 'SignUp',
      component: () => import('./views/SignUpPage.vue'),
      meta: { title: 'Sign Up' },
    },
    {
      path: '/auth/forgot',
      name: 'ForgotPassword',
      component: () => import('./views/ForgotPasswordPage.vue'),
      meta: { title: 'Forgot Password' },
    },
    {
      path: '/reset-password',
      name: 'ResetPassword',
      component: () => import('./views/ResetPasswordPage.vue'),
      meta: { title: 'Reset Password' },
    },
    {
      path: '/confirm-email',
      name: 'ConfirmEmail',
      component: () => import('./views/ConfirmEmailPage.vue'),
      meta: { title: 'Confirm Email' },
    },
    {
      path: '/profile',
      name: 'Profile',
      component: () => import('./views/ProfilePage.vue'),
      redirect: '/profile/settings',
      meta: metaAuth('Profile'),
      children: [
        {
          path: 'settings',
          name: 'Settings',
          component: () => import('./components/profile/Settings.vue'),
        },
      ],
    },
    {
      path: '/terms',
      name: 'Terms',
      component: () => import('./views/TermsPage.vue'),
      meta: { title: 'Terms of Service' },
    },
    {
      path: '/privacy',
      name: 'Privacy',
      component: () => import('./views/PrivacyPage.vue'),
      meta: { title: 'Privacy Policy' },
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: import('./views/NotFoundPage.vue'),
    },
  ],
})

router.afterEach((to, _from) => {
  const parent = to.matched.find((record) => record.meta.title)
  const parentTitle = parent ? parent.meta.title : null
  document.title = to.meta.title || parentTitle || 'Quizzo'
})

export default router

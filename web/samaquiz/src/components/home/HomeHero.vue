<template>
  <div class="hero-wrap">
    <div class="home-hero container f-center-col">
      <h1 class="hero-title">
        {{ ts('home.hero') }}
      </h1>
      <div class="hero-text">
        {{ ts('home.hero_text') }}
      </div>
      <div v-if="loggedIn" class="links-wrap">
        <router-link :to="{ name: 'Profile' }" class="sign-up-button-wrap">
          {{ ts('home.my') }}
        </router-link>
      </div>
      <div v-else class="links-wrap">
        <router-link :to="{ name: 'Login' }" class="login-button-wrap">
          {{ ts('auth.login') }}
        </router-link>
        <router-link :to="{ name: 'SignUp' }" class="sign-up-button-wrap">
          {{ ts('auth.sign_up') }}
        </router-link>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { loggedIn } from '@frontend/features'
import { ts } from '../../i18n'
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.hero-wrap {
  background: $color4;
}
.home-hero {
  min-height: calc(100vh - $header-height);
}
.hero-text {
  @mixin title-thin 22px;
}
.links-wrap {
  margin-top: 40px;
}
@media (max-width: 680px) {
  .home-hero {
    text-align: center;
  }
  .hero-title {
    font-size: 48px;
  }
}
</style>

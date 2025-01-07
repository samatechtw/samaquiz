<template>
  <header>
    <div class="header container">
      <router-link :to="{ name: 'Home' }">
        <div class="logo-wrap">
          <Logo class="header-logo" />
          <div class="company">
            {{ ts('company') }}
          </div>
        </div>
      </router-link>
      <div class="header-links">
        <!--
        <HeaderLink to="Page1" :text="ts('page1.title')" />
        <HeaderLink to="Page2" :text="ts('page2.title')" />
        <HeaderLink to="Page3" :text="ts('page3.title')" />
      --></div>
      <UserMenu v-if="loggedIn" class="user-menu" />
      <div v-else class="auth-links-wrap">
        <router-link :to="{ name: 'Login' }" class="login-button-wrap">
          {{ ts('auth.login') }}
        </router-link>
        <router-link :to="{ name: 'SignUp' }" class="sign-up-button-wrap">
          {{ ts('auth.sign_up') }}
        </router-link>
      </div>
      <Burger @click="showDrawer(true)" />
      <Drawer :active="drawerActive" @close="showDrawer(false)" class="header-drawer">
        <!--
        <DrawerLink
          to="Page1"
          :text="ts('page1.title')"
          @click="showDrawer(false)"
        />
        <DrawerLink
          to="Page2"
          :text="ts('page2.title')"
          @click="showDrawer(false)"
        />
        <DrawerLink to="Page3" :text="ts('page3.title')" @click="showDrawer(false)" />
      -->
      </Drawer>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { drawerActive, showDrawer, loggedIn } from '@frontend/features'
import { Burger } from '@frontend/components/widgets'
import { Logo } from '@frontend/components/svg'
import Drawer from './Drawer.vue'
import DrawerLink from './DrawerLink.vue'
import HeaderLink from './HeaderLink.vue'
import { ts } from '../../i18n'
import UserMenu from './UserMenu.vue'
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

header {
  width: 100%;
  position: absolute;
  z-index: 999;
  top: 0;
  height: $header-height;
  background-color: rgba(255, 255, 255, 0.2);
}
.header {
  @mixin text 16px;
  height: 100%;
  display: flex;
  align-items: center;
  margin: 0 auto;
  position: relative;
}
.logo-wrap {
  display: flex;
  align-items: center;
}
.company {
  @mixin title 24px;
  margin-left: 8px;
}
.header-logo {
  width: 64px;
  height: 64px;
}
.header-links {
  display: flex;
  margin-left: auto;
  align-items: center;
}
.burger {
  display: none;
  margin-left: 16px;
}
.header-drawer {
  display: none;
}
.login-button-wrap {
  @mixin title-regular 15px;
  margin-right: 16px;
  padding: 8px 16px;
}
.sign-up-button-wrap {
  @mixin title-regular 15px;
  border: 1px solid $color3;
  border-radius: 8px;
  padding: 8px 16px;
  transition: opacity 0.2s ease;
  &:hover {
    opacity: 0.7;
  }
}
@media (max-width: 860px) {
  .company {
    @mixin title 32px;
  }
  .links a {
    margin: 0 12px;
  }
}
@media (max-width: 680px) {
  .links {
    display: none;
  }
  .header-drawer {
    display: block;
  }
  .burger {
    display: flex;
  }
}
</style>

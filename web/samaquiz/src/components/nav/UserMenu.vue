<template>
  <AppMenu
    :items="menuItems"
    :extraHeight="40"
    clickawaySelector=".user-menu"
    class="user-menu"
  >
    <template #toggle>
      <Avatar :url="avatar" size="40" class="avatar-image" />
      <div v-if="hasNotifications" class="notification" />
    </template>
    <template #before>
      <router-link :to="{ name: 'Settings' }">
        <div class="user-email">
          <div class="email-text">
            {{ email }}
          </div>
          <Settings class="settings" />
        </div>
      </router-link>
    </template>
  </AppMenu>
</template>

<script lang="ts" setup>
import { store } from '@frontend/store'
import { useLoginRedirect } from '@frontend/features'
import { IDropdownMenuItem } from '@frontend/types'
import { AppMenu, Avatar } from '@frontend/components/widgets'
import Settings from '../svg/Settings.vue'

const { avatar, email } = store.user
const { checkAuthRedirect } = useLoginRedirect()

const hasNotifications = false

const logout = () => {
  store.auth.logOut()
  checkAuthRedirect()
}

const menuItems: IDropdownMenuItem[] = [
  {
    class: 'quizzes',
    labelKey: 'quizzes.title',
    to: {
      name: 'Profile',
    },
  },
  {
    class: 'logout-button',
    labelKey: 'logout',
    click: logout,
  },
]
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.avatar-image {
  cursor: pointer;
}
:deep(.logout-button) {
  color: $red;
  cursor: pointer;
}
.user-email {
  display: flex;
  align-items: center;
  height: 40px;
  background-color: $bg-light1;
  font-size: 14px;
  padding: 0 16px;
}
.email-text {
  @mixin title-regular 14px;
  @mixin truncate;
  color: $text2;
  margin-right: 8px;
  max-width: 141px;
}
.settings {
  @mixin size 26px;
  margin-left: auto;
  cursor: pointer;
  :deep(path) {
    fill: $text2;
    transition: fill ease 0.2s;
  }
  &:hover {
    :deep(path) {
      fill: $primary;
    }
  }
}
</style>

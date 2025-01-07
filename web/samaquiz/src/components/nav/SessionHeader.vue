<template>
  <header v-if="!quizActive">
    <div class="header container">
      <router-link :to="{ name: 'Home' }">
        <div class="logo-wrap">
          <div class="company">
            {{ ts('company') }}
          </div>
        </div>
      </router-link>
      <UserMenu v-if="loggedIn" class="user-menu" />
    </div>
  </header>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { loggedIn, quizSession } from '@frontend/features'
import { QuizSessionStatus } from '@frontend/types'
import { ts } from '../../i18n'
import UserMenu from './UserMenu.vue'

const quizActive = computed(() => {
  return quizSession.value?.status === QuizSessionStatus.Active
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

header {
  width: 100%;
  position: absolute;
  z-index: 999;
  top: 0;
  height: 54px;
  background-color: rgba(255, 255, 255, 0.2);
}
.header {
  height: 100%;
  display: flex;
  align-items: center;
  margin: 0 auto;
  position: relative;
  justify-content: space-between;
}
.company {
  @mixin title 21px;
  margin-left: 8px;
}
@media (max-width: 860px) {
  .company {
    @mixin title 19px;
  }
}
</style>

<template>
  <div class="quiz-waiting f-center-col">
    <div class="wait-host">
      {{ ts('session.wait_host') }}
    </div>
    <div class="avatar-wrap">
      <Avatar :url="localParticipant.avatar" size="32" />
      <div class="name">
        {{ localParticipant.name }}
      </div>
    </div>
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { IQuizParticipant } from '@frontend/types'
import { Avatar } from '@frontend/components/widgets'
import { ts } from '../../i18n'

defineProps<{
  localParticipant: IQuizParticipant
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-waiting {
  @mixin quiz-bubble;
  margin-top: 20%;
}
.wait-host {
  @mixin title 24px;
}
.avatar-wrap {
  @mixin title 16px;
  display: flex;
  align-items: center;
  margin-top: 16px;
}
.name {
  margin-left: 8px;
}
</style>

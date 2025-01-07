<template>
  <div class="leaders">
    <div class="leaders-title">
      {{ ts('leaders') }}
    </div>
    <Spinner v-if="loadingLeaders" :size="24" />
    <div v-else-if="!leaders?.length">
      {{ ts('no_leaders') }}
    </div>
    <div v-else class="leaders-wrap">
      <div v-for="(leader, index) in leaders" class="leader-view">
        <div class="leader-index">{{ `${index + 1}.` }}</div>
        <Avatar :url="leader.avatar" size="40" class="leader-avatar" />
        <div class="leader-name">{{ leader.name }}</div>
        <!--<div class="leader-points">{{ leader.points }}</div>-->
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ISessionLeader } from '@frontend/types'
import { Spinner, Avatar } from '@frontend/components/widgets'
import { ts } from '../../i18n'

defineProps<{
  leaders: ISessionLeader[] | undefined
  loadingLeaders: boolean
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.leaders {
  width: 100%;
}
.leaders-title {
  @mixin title 20px;
  margin-top: 24px;
}
.leaders-wrap {
  display: flex;
  justify-content: space-around;
  width: 100%;
  flex-wrap: wrap;
  margin-top: 12px;
}
.leader-view {
  display: flex;
  align-items: center;
  width: 50%;
}
.leader-name {
  @mixin title 20px;
  margin: 0 12px 0 8px;
}
.leader-index {
  @mixin title 24px;
}
.leader-avatar {
  margin-left: 8px;
}
.leader-points {
  @mixin title 28px;
  color: $color3;
}
</style>

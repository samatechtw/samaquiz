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
      <div class="leaders-left">
        <div v-for="(leader, index) in leaders1" class="leader-view">
          <div class="leader-index">{{ `${index + 1}.` }}</div>
          <Avatar :url="leader.avatar" size="40" class="leader-avatar" />
          <div class="leader-name">{{ leader.name }}</div>
          <!--<div class="leader-points">{{ leader.points }}</div>-->
        </div>
      </div>
      <div class="leaders-right">
        <div v-for="(leader, index) in leaders2" class="leader-view">
          <div class="leader-index">{{ `${leaders1.length + index + 1}.` }}</div>
          <Avatar :url="leader.avatar" size="40" class="leader-avatar" />
          <div class="leader-name">{{ leader.name }}</div>
          <!--<div class="leader-points">{{ leader.points }}</div>-->
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ISessionLeader } from '@frontend/types'
import { Spinner, Avatar } from '@frontend/components/widgets'
import { ts } from '../../i18n'
import { computed } from 'vue'

const { leaders } = defineProps<{
  leaders: ISessionLeader[] | undefined
  loadingLeaders: boolean
}>()

const leaders1 = computed(() => {
  if (!leaders || leaders.length <= 1) {
    return leaders
  }
  return leaders.slice(0, Math.round(leaders.length / 2))
})

const leaders2 = computed(() => {
  if (!leaders || leaders.length <= 1) {
    return []
  }
  return leaders.slice(Math.round(leaders.length / 2), leaders.length)
})
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
.leaders-left {
  width: 50%;
}
.leaders-right {
  width: 50%;
}
.leaders-wrap {
  display: flex;
  justify-content: space-around;
  width: 100%;
  flex-wrap: wrap;
  margin-top: 12px;
}
.leader-view {
  @mixin truncate;
  display: flex;
  align-items: center;
  width: 100%;
  margin-top: 12px;
  justify-content: center;
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
  flex-shrink: 0;
}
.leader-points {
  @mixin title 28px;
  color: $color3;
}
</style>

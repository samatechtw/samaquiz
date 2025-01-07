<template>
  <div class="router-tab-view f-col">
    <div v-if="!hideTabs" class="router-tab-view-tabs">
      <router-link
        v-for="tabRoute in routes"
        :key="tabRoute.name"
        :to="{ name: tabRoute.name }"
        class="router-tab-view-tab"
        :class="{
          [`router-tab-view-tab--${variant}`]: true,
          [tabRoute.labelClass ?? '']: true,
        }"
        :replace="tabRoute.replace"
      >
        {{ tabRoute.label }}
      </router-link>
    </div>
    <slot name="tab-body" />
    <router-view v-bind="routerViewProps" />
  </div>
</template>

<script lang="ts" setup>
import { computed, toRefs } from 'vue'
import { useRoute } from 'vue-router'
import { IRoute } from '@frontend/types'

const props = withDefaults(
  defineProps<{
    variant?: 'button' | 'link'
    hideTabs?: boolean
    routes: Array<IRoute>
  }>(),
  {
    hideTabs: false,
    variant: 'button',
  },
)

const { routes } = toRefs(props)

const route = useRoute()

const routerViewProps = computed(() => {
  const name = route.name ?? {}
  return routes.value.find((x) => x.name === name)?.props ?? {}
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.router-tab-view {
  .router-tab-view-tabs {
    display: flex;
    flex-direction: row;
    border-bottom: 1px solid $border1;
    overflow-x: auto;
    column-gap: 8px;
    margin-bottom: 55px;
    white-space: nowrap;
    flex-shrink: 0;
  }
}
.router-tab-view-tab {
  @mixin title 16px;
  transition: background-color 0.3;
  &.router-tab-view-tab--link {
    color: $text3;
    margin-right: 32px;
    padding-bottom: 8px;
  }
  &.router-tab-view-tab--button {
    display: flex;
    align-items: center;
    padding: 0 16px;
    height: 40px;
    color: $text3;
    background-color: white;
    border: 1px solid rgba(11, 11, 13, 0.1);
    border-radius: 16px;
  }
  &.router-link-active {
    &.router-tab-view-tab--link {
      color: $text2;
      z-index: 2;
    }
    &.router-tab-view-tab--button {
      @mixin title-regular 14px;
      color: $text1;
      color: $text-light2;
    }
  }
}
</style>

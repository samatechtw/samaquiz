<template>
  <div class="dropdown-menu-items" :class="{ opened }">
    <slot name="before"></slot>
    <template v-for="(item, index) in items" :key="index.toString()">
      <component
        :is="itemTag(item)"
        class="app-dropdown-item"
        :class="{ [item.class ?? '']: item.class }"
        v-bind="itemProps(item)"
        :disabled="item.loading"
        @click="itemClick($event, item)"
      >
        <Spinner v-if="item.loading" class="dropdown-menu-item-spinner" color="#868692" />
        <template v-else>
          {{ computeItemLabel(item) }}
        </template>
      </component>
    </template>
  </div>
</template>

<script lang="ts" setup>
import { IDropdownMenuItem } from '@frontend/types'
import { toRefs } from 'vue'
import Spinner from './Spinner.vue'
import { ts } from '../../i18n'

const props = withDefaults(
  defineProps<{
    opened: boolean
    items: IDropdownMenuItem[]
    // Extra height used by slots is needed for the height animation to work correctly
    extraHeight?: number
  }>(),
  {
    extraHeight: 0,
  },
)
const { items } = toRefs(props)

const emit = defineEmits<{
  (e: 'itemClick', item: IDropdownMenuItem): void
  (e: 'closeMenu'): void
}>()

const itemTag = (item: IDropdownMenuItem) => {
  const { to } = item
  if (to) {
    return typeof to === 'string' ? 'a' : 'router-link'
  } else {
    return 'div'
  }
}

const itemProps = (item: IDropdownMenuItem) => {
  const { to, linkTarget } = item
  if (!to) {
    // Not a link
    return {}
  } else {
    // Is a link
    if (typeof to === 'string') {
      return {
        href: to,
        target: linkTarget,
      }
    } else {
      return {
        to,
        target: linkTarget,
      }
    }
  }
}

const itemClick = (e: MouseEvent, item: IDropdownMenuItem) => {
  e.stopPropagation()
  if (!item.loading) {
    item.click?.()
    emit('itemClick', item)
    emit('closeMenu')
  }
}

const computeItemLabel = (item: IDropdownMenuItem) => {
  if (item.labelKey) {
    return ts(item.labelKey)
  } else {
    return item.label
  }
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.dropdown-menu-items {
  filter: drop-shadow(0 8px 16px rgba(0, 0, 0, 0.25));
  position: absolute;
  min-width: 216px;
  height: 0;
  max-height: 192px;
  overflow: hidden;
  transition: height 0.2s;
  z-index: 1;
  &.opened {
    height: v-bind(extraHeight + (items.length * 48) + 'px');
    overflow-y: auto;
  }
}
.app-dropdown-item {
  @mixin title-regular 15px;
  background-color: white;
  display: block;
  color: black;
  padding: 12px 16px;
  &:not(:first-child) {
    border-top: 1px solid $border1;
  }
  &[disabled='true'] {
    color: $disabled1;
    cursor: default;
  }
  &:not([disabled='true']):hover {
    background-color: $border1;
  }
}
.dropdown-menu-item-spinner {
  margin-left: 4px;
}
</style>

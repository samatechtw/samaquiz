<template>
  <div class="settings-row">
    <div class="row-left">
      <div class="row-title">
        {{ title }}
      </div>
      <div v-if="text" class="row-text">
        {{ text }}
      </div>
    </div>
    <div class="row-right f-col">
      <slot name="right" />
    </div>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  title: string
  text?: string
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.settings-row {
  display: flex;
  padding: 24px 0 32px;
  border-bottom: 1px solid $border1;
}
.row-left {
  padding-right: 24px;
  max-width: 60%;
}
.row-right {
  margin-left: auto;
  justify-content: center;
}
.row-title {
  @mixin title 17px;
  color: $text1;
}
.row-text {
  @mixin text 16px;
  color: $text2;
  margin-top: 8px;
}
</style>

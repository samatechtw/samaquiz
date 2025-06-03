<template>
  <div class="avatar-select">
    <div
      v-for="cat in cats"
      class="cat-wrap f-center"
      :class="{ selected: cat === selected }"
      @click="selected = cat"
    >
      <img :src="cat" draggable="false" />
    </div>
  </div>
</template>

<script lang="ts" setup>
const cats = Array.from(
  { length: 12 },
  (_, i) => `http://quiz-web.pubstudioassets.com/cat${i + 1}.png`,
)

const selected = defineModel<string>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.avatar-select {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  width: 280px;
}
.cat-wrap {
  width: 60px;
  border: 2px solid transparent;
  transition: all 0.25s ease;
  border-radius: 50%;
  cursor: pointer;
  user-select: none;
  &.selected {
    border-color: $color5;
    background-color: $color6;
  }
  img {
    width: 52px;
    height: 52px;
    user-select: none;
  }
}
</style>

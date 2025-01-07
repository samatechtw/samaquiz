<template>
  <div
    class="quiz-answer f-center"
    :class="{ dim, correct, incorrect, [`a${index}`]: true }"
  >
    {{ text }}
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  text: string
  index: number
  dim?: boolean
  correct?: boolean
  incorrect?: boolean
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.a0 {
  background-color: $answer1;
}
.a1 {
  background-color: $answer2;
}
.a2 {
  background-color: $answer3;
}
.a3 {
  background-color: $answer4;
}
.quiz-answer {
  @mixin title-regular 20px;
  width: calc(50% - 8px);
  margin-top: 16px;
  height: 140px;
  padding: 0 16px;
  border: 8px solid transparent;
  border-radius: 8px;
  box-sizing: border-box;
  color: #ffffff;
  cursor: pointer;
  line-height: 130%;
  overflow: visible;
  text-align: center;
  touch-action: manipulation;
  transition: all 0.2s;
  user-select: none;
  &:hover:not(:disabled) {
    filter: brightness(1.1);
  }
}
.dim {
  opacity: 0.3;
}
.correct {
  border-color: $color3;
  filter: brightness(1.5);
  opacity: 1;
}
.incorrect {
  background-color: black;
  opacity: 0.8;
}

@media (max-width: 680px) {
  .quiz-answer {
    height: 180px;
  }
}
</style>

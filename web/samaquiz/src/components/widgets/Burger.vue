<template>
  <div class="burger f-col">
    <div class="patty"></div>
    <div class="patty"></div>
    <div class="patty"></div>
  </div>
</template>

<style>
@import '@theme/css/defines.postcss';

.burger {
  height: 18px;
  width: 24px;
  justify-content: space-between;
}
.patty {
  width: 100%;
  height: 100%;
  height: 2px;
  border-radius: 1px;
  background-color: $color2;
}
</style>

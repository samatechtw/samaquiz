<template>
  <router-view />
</template>

<style lang="postcss">
@import '@theme/css/font.postcss';
@import '@theme/css/defines.postcss';
@import '@theme/css/app.postcss';
@import '@theme/css/auth.postcss';

html,
body {
  padding: 0;
  margin: 0;
  width: 100%;
  color: black;
  background: $color5;
  * {
    box-sizing: border-box;
  }
}

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  position: relative;
  overflow: hidden;
}
</style>

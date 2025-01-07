<template>
  <div class="details-wrap">
    <div class="home-details container">
      <div class="details-left f-col">
        <h2 class="details-title">
          {{ ts('home.details') }}
        </h2>
        <div class="details-text">
          {{ ts('home.details_text') }}
        </div>
      </div>
      <div class="details-image"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.details-wrap {
  background-color: $bg-dark2;
}
.home-details {
  display: flex;
  min-height: 680px;
  padding-top: 96px;
  padding-bottom: 120px;
  color: $text-light1;
}
.details-left {
  width: 50%;
  justify-content: center;
}
.details-text {
  @mixin title-thin 22px;
  max-width: 400px;
}
.details-image {
  width: 50%;
  background-color: #aaa;
}
@media (max-width: 900px) {
  .home-details {
    flex-direction: column;
  }
  .details-left {
    width: 100%;
    text-align: center;
    align-items: center;
  }
  .details-image {
    width: 100%;
    height: 300px;
    max-width: 400px;
    margin: 64px auto 0;
  }
}
</style>

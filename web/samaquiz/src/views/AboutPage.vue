<template>
  <div class="about-wrap blog-wrap">
    <div class="blog-content">
      <h1 class="privacy-title blog-title">About</h1>
      <div class="blog-subtitle">Last Updated: Feb. 22, 2025</div>
      <div class="blog-text">
        SamaQuiz is an open source live quiz app that is, for now, completely free. Quiz
        participants are currently uncapped, but heavy usage may cause issues with the
        server - no guarantees! If you need to support hundreds or thousands of users,
        please consider self-hosting, or reach out to
        <a href="mailto:support@samatech.tw" target="_blank">support@samatech.tw</a>
      </div>
      <div class="blog-text">
        The SamaQuiz source code is available on GitHub, see
        <a href="https://github.com/samatechtw/samaquiz" target="_blank">
          github.com/samatechtw/samaquiz
        </a>
        for instructions on how to build and deploy your own instance. It is completely
        free to use as long as you are not re-packaging it for sale in any way.
      </div>
      <div class="blog-text">
        If you want to support the project, there may be paid plans in the future. For
        now, you could buy me a coffee!
      </div>
      <KofiButton class="kofi" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import { KofiButton } from '@frontend/components/widgets'

onMounted(() => {})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.about-wrap {
  background-color: $color6;
  .blog-content {
    max-width: 760px;
  }
}
.kofi {
  margin-top: 40px;
  text-align: center;
}
</style>

<template>
  <div class="kofi-container" v-html="kofiHtml"></div>
</template>

<script lang="ts" setup>
import { kofiHtml, useKofiButton } from '@frontend/util/ui'

useKofiButton()
</script>

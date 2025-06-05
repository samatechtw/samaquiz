<template>
  <div class="app-asset f-center" :class="{ isVideo }">
    <!--
    <video
      v-if="isVideo"
      ref="videoPlayer"
      :class="{ 'hide-play-button': !showVideoControls }"
      class="app-asset-video video-js vjs-theme-fantasy"
    >
      <source :src="url" type="video/mp4" />
    </video>
    -->
    <img v-if="asImg && url" :src="url" class="app-asset-image" />
    <div
      v-else-if="url"
      :style="{ backgroundImage: `url(${url})` }"
      class="app-asset-image"
    />
    <img v-else :src="EditPhoto" class="app-asset-default" />
  </div>
</template>

<script lang="ts" setup>
// import videojs, { VideoJsPlayer, VideoJsPlayerOptions } from 'video.js'
import { computed, ref, toRefs, onMounted, onUnmounted } from 'vue'
import EditPhoto from '@theme/img/edit_photo.svg'

/*
let player: VideoJsPlayer
*/
const videoPlayer = ref<Element>()

const props = withDefaults(
  defineProps<{
    asset?: string | null
    asImg?: boolean
    contentHash?: string | number
    canPlayVideo?: boolean
  }>(),
  {
    asset: null,
    contentHash: undefined,
    canPlayVideo: true,
  },
)
const { asset, canPlayVideo, contentHash } = toRefs(props)

const isVideo = computed(() => asset.value?.endsWith('.mp4') === true)

const url = computed(() => {
  const hash = contentHash.value
  return (asset.value ?? '') + (hash ? `?${hash}` : '')
})

// Don't show controls if the asset is an image, or they are explicitly disabled
const showVideoControls = computed(() => isVideo.value && canPlayVideo.value)

onMounted(() => {
  if (isVideo.value) {
    /*
    const options: VideoJsPlayerOptions = {
      loop: true,
      controls: showVideoControls.value,
    }
    player = videojs(videoPlayer.value || '', options)
    */
  }
})
onUnmounted(() => {
  /*
  if (player) {
    player.dispose()
  }
  */
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.app-asset {
  width: 100%;
  height: 100%;
  padding: 4px;
  position: relative;
  background-color: $color4;
  &.isVideo {
    background-color: unset;
    align-items: flex-start;
  }
}
.app-asset-image {
  width: 100%;
  height: 100%;
  background-size: cover;
  background-position: center;
  object-fit: contain;
  object-position: center;
}
.app-asset-video {
  width: 100%;
  height: 100%;
  &.hide-play-button {
    .vjs-big-play-button {
      display: none;
    }
  }
  .vjs-control-bar {
    border-bottom-left-radius: 24px;
    border-bottom-right-radius: 24px;
  }
}
.app-asset-default {
  width: 32px;
  height: 32px;
}
</style>

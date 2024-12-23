<template>
  <div class="profile-top">
    <div class="top-left f-center-col">
      <div class="profile-avatar f-center">
        <div class="avatar-image" v-html="identiconAvatar" />
      </div>
      <div class="profile-name">
        {{ name }}
      </div>
    </div>
    <div class="top-right">
      <div class="profile-title-wrap">
        <div class="profile-title">
          {{ ts('profile.title') }}
        </div>
        <div class="profile-edit" @click="emit('edit')">
          {{ ts('edit') }}
        </div>
      </div>
      <div class="profile-description">
        {{ description }}
      </div>
      <div class="profile-link">
        <a :href="link" target="_blank">{{ link }}</a>
      </div>
      <div v-if="location" class="profile-location profile-count count">
        <Location class="location-icon count-icon" />
        {{ location }}
      </div>
      <div class="profile-joined">
        {{ joined }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatDistance } from 'date-fns'
import { store } from '@frontend/store'
import { Location } from '@frontend/components/svg'
import { ts } from '../../i18n'

const emit = defineEmits<{
  (e: 'edit'): void
}>()

const { identiconAvatar } = store.user

const name = computed(() => {
  return store.user.name.value || 'No Name'
})

const location = computed(() => {
  return store.user.location.value
})

const description = computed(() => {
  return store.user.description.value || 'No description'
})

const link = computed(() => {
  return store.user.link.value
})

const joined = computed(() => {
  let date = 'recently'
  const created = store.user.createdAt.value
  if (created) {
    formatDistance(created, new Date(), {
      addSuffix: true,
    })
  }
  return `Joined ${date}`
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.profile-top {
  display: flex;
  width: 100%;
  padding: 40px 0;
  margin: 0 auto;
  max-width: 1080px;
}
.top-left {
  width: 300px;
}
.profile-avatar {
  @mixin size 240px;
  border-radius: 50%;
  overflow: hidden;
  background-color: $border1;
}
.avatar-image {
  @mixin size 100%;
  top: -12px;
  position: relative;
}
.profile-name {
  @mixin title 24px;
  text-align: center;
  margin-top: 20px;
  color: $text2;
}
.top-right {
  padding: 0 0 0 64px;
}
.profile-title-wrap {
  display: flex;
  align-items: center;
}
.profile-title {
  @mixin title 40px;
  color: $text2;
}
.profile-edit {
  @mixin title 16px;
  margin: 4px 0 0 40px;
  cursor: pointer;
  color: $text3;
  user-select: none;
}
.profile-count {
  @mixin title-regular 14px;
  display: flex;
  margin-top: 16px;
  color: $primary;
}
.count {
  display: flex;
  align-items: center;
}
.count-icon {
  @mixin size 18px;
  margin-right: 6px;
}
.profile-description {
  @mixin text 15px;
  margin-top: 24px;
  color: $text2;
}
.profile-link {
  @mixin title-regular 14px;
  color: $primary;
  margin-top: 24px;
}
.profile-location {
  margin-top: 8px;
  color: $text3;
}
.profile-joined {
  @mixin title-regular 13px;
  color: $text-light2;
  margin-top: 8px;
}

@media (max-width: 600px) {
  .profile-avatar {
    @mixin size 200px;
  }
  .profile-title {
    display: none;
  }
  .profile-top {
    flex-direction: column;
    align-items: center;
  }
  .top-left {
    width: 100%;
    padding: 0 24px;
  }
  .top-right {
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 0 24px;
    align-items: center;
  }
  .profile-edit {
    margin: 16px 0 0 0;
  }
}
</style>

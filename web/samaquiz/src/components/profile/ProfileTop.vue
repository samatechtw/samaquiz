<template>
  <div class="profile-top">
    <div class="top-left f-center-col">
      <Avatar :url="avatar" size="90" class="avatar-image">
        <div class="avatar-edit overlay f-center" @click="showAvatarModal = true">
          <Edit color="white" class="edit-icon" />
        </div>
      </Avatar>
      <div class="name-wrap" @click="emit('edit')">
        <div class="name">
          {{ name }}
        </div>
        <Edit class="edit-icon" />
      </div>
    </div>
    <div class="top-right">
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
    <ProfileAvatarSelect :show="showAvatarModal" @cancel="showAvatarModal = false" />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { formatDistance } from 'date-fns'
import { store } from '@frontend/store'
import { Edit, Location } from '@frontend/components/svg'
import { Avatar } from '@frontend/components/widgets'
import ProfileAvatarSelect from './ProfileAvatarSelect.vue'

const emit = defineEmits<{
  (e: 'edit'): void
}>()

const showAvatarModal = ref(false)

const name = computed(() => {
  return store.user.name.value || 'No Name'
})

const avatar = computed(() => {
  return store.user.avatar.value
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
  padding: 32px 0;
  margin: 0 auto;
  max-width: 800px;
}
.top-left {
  padding: 0 40px;
}
.avatar-edit {
  transition: opacity 0.2s ease;
  background-color: rgba(0, 0, 0, 0.15);
  opacity: 0;
  cursor: pointer;
  &:hover {
    opacity: 1;
  }
  .edit-icon {
    @mixin size 24px;
  }
}
.name-wrap {
  @mixin title 16px;
  text-align: center;
  margin-top: 12px;
  color: $text2;
  display: flex;
  user-select: none;
  cursor: pointer;
}
.top-right {
  padding: 0 0 0 64px;
}
.edit-icon {
  @mixin size 18px;
  margin-left: 6px;
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
  color: $text2;
}
.profile-link {
  @mixin title-regular 14px;
  color: $primary;
  margin-top: 12px;
}
.profile-location {
  margin-top: 12px;
  color: $text3;
}
.profile-joined {
  @mixin title-regular 13px;
  color: $text-light2;
  margin-top: 12px;
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

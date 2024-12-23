<template>
  <footer>
    <div class="footer-wrap">
      <div class="footer-content">
        <div class="footer-left f-col">
          <router-link :to="{ name: 'Home' }">
            <Logo class="footer-logo" />
          </router-link>
          <div class="footer-title">
            {{ ts('footer.title') }}
          </div>
          <div class="footer-text">
            {{ ts('footer.text') }}
          </div>
          <div class="socials">
            <a href="https://x.com/quizzo" target="_blank">
              <div class="social">
                <Twitter />
              </div>
            </a>
            <a href="https://medium.com/@quizzo" target="_blank">
              <div class="social">
                <Medium />
              </div>
            </a>
          </div>
        </div>
        <div class="footer-right f-col">
          <div class="footer-cols">
            <div class="footer-col">
              <div class="label">{{ ts('footer.col1') }}</div>
              <router-link
                :to="{ name: 'Home' }"
                class="link"
                :class="{ active: route.name === 'Home' }"
              >
              </router-link>
              <!--
                {{ ts('home') }}
              </router-link>
              <router-link
                :to="{ name: 'Page4' }"
                class="link"
                :class="{ active: route.name === 'About' }"
              >
                {{ ts('footer.about') }}
              </router-link>
              <router-link
                :to="{ name: 'Faq' }"
                class="link"
                :class="{ active: route.name === 'Faq' }"
              >
                {{ ts('footer.faq') }}
              </router-link>
              -->
            </div>
            <div class="footer-col">
              <div class="label">{{ ts('footer.col2') }}</div>
              <!--
              <router-link
                :to="{ name: 'Page1' }"
                class="link"
                :class="{ active: route.name === 'Page1' }"
              >
                {{ ts('integration.title') }}
              </router-link>
              <router-link
                :to="{ name: 'Page2' }"
                class="link"
                :class="{ active: route.name === 'Page2' }"
              >
                {{ ts('features.title') }}
              </router-link>
              <router-link
                :to="{ name: 'Page3' }"
                class="link"
                :class="{ active: route.name === 'Page3' }"
              >
                {{ ts('connect.start') }}
              </router-link>
            --></div>
            <div class="footer-col"></div>
          </div>
          <div class="footer-br f-col">
            <div class="bottom-links">
              <router-link
                :to="{ name: 'Terms' }"
                class="bottom-link"
                :class="{ active: route.name === 'Terms' }"
              >
                {{ ts('footer.terms') }}
              </router-link>
              <div class="bottom-divider"></div>
              <router-link
                :to="{ name: 'Privacy' }"
                class="bottom-link"
                :class="{ active: route.name === 'Privacy' }"
              >
                {{ ts('footer.privacy') }}
              </router-link>
            </div>
            <div class="copyright">
              {{ ts('footer.copyright') }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { ts } from '../../i18n'
import { Logo, Medium, Twitter } from '@frontend/components/svg'

const route = useRoute()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

$text-color: $text4;

.footer {
  position: relative;
}
.bottom-links {
  display: flex;
  margin-top: 8px;
}
.bottom-divider {
  width: 1px;
  height: 15px;
  background-color: $border1;
  margin: 0 20px;
}
.bottom-link {
  @mixin text 12px;
  letter-spacing: 1.2px;
  color: white;
  &:hover {
    color: $text-color;
  }
}
a.active {
  color: $text-color;
}
.footer-wrap {
  position: relative;
  background-color: $color1;
  color: white;
}
.footer-content {
  display: flex;
  position: relative;
  min-height: 220px;
  padding: 64px 40px 40px;
  width: 100%;
  max-width: 1080px;
  align-items: stretch;
  margin: 0 auto;
}
.footer-top {
  display: flex;
  justify-content: space-between;
  flex-direction: row;
}
.footer-left {
  max-width: 300px;
}
.footer-title {
  @mixin title 20px;
  color: $text-color;
}
.footer-text {
  @mixin text 15px;
  margin-top: 8px;
}
.footer-logo {
  width: 180px;
}
.footer-right {
  margin-left: auto;
  align-items: center;
  justify-content: space-between;
}
.footer-br {
  margin-top: 48px;
  align-items: flex-end;
  position: relative;
}
.copyright {
  @mixin title 13px;
  letter-spacing: 0.03em;
  margin-top: 8px;
}
.footer-cols {
  display: flex;
  flex-direction: row;
  margin-left: auto;
}
.footer-col {
  display: flex;
  flex-direction: column;
  margin-left: 64px;
}
.label {
  @mixin title 15px;
  font-weight: 800;
  color: $text3;
  letter-spacing: 0.03em;
  margin-bottom: 12px;
}
.link {
  @mixin text 15px;
  padding: 6px 0;
  color: white;
  &:not(:last-child) {
    border-bottom: 1px solid rgba(181, 216, 205, 0.25);
  }
}
.socials {
  display: flex;
  margin-top: 4px;
}
.social {
  width: 28px;
  height: 28px;
  padding: 4px;
  cursor: pointer;
  margin-right: 8px;
  :deep(svg) {
    fill: white;
    transition: fill 0.25s ease;
    width: 24px;
    height: 24px;
  }
  &:hover {
    :deep(svg) {
      fill: $text-color;
    }
  }
}
@media (max-width: 820px) {
  .footer-content {
    padding: 48px 40px 48px;
  }
  .footer-top {
    flex-direction: column;
    align-items: center;
  }
  .footer-cols {
    margin: 48px 0 0 0;
  }
  .footer-col {
    &:first-child {
      margin-left: 0;
    }
  }
  .footer-left {
    max-width: 600px;
    text-align: center;
  }
  .footer-br {
    align-items: center;
    margin-top: 24px;
  }
  .footer-wrap {
    overflow: hidden;
  }
  .footer-bg {
    object-position: 49.9% 8.7%;
    transform: scale(2);
  }
}
@media (max-width: 700px) {
  .footer-bg {
    object-position: 49.9% 0%;
    transform: scale(2);
  }
}
@media (max-width: 700px) {
  .footer-bg {
    object-position: 49.9% -10%;
    transform: scale(2);
  }
}
@media (max-width: 510px) {
  .footer-bg {
    object-position: 49.9% 140px;
    transform: scale(3);
  }
  .footer-cols {
    flex-direction: column;
    margin-top: 24px;
  }
  .footer-col {
    margin-left: 0;
    margin-top: 0;
    text-align: center;
    &:last-child {
      margin-top: 0;
    }
  }
  .label {
    margin-top: 16px;
  }
  .empty-label {
    display: none;
  }
  .footer-left {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .bottom-link {
    text-align: center;
  }
  .bottom-divider {
    margin: 0 10px;
  }
}
@media (max-width: 460px) {
  .footer-bg {
    object-position: 49.9% 220px;
    transform: scale(3);
  }
}
</style>

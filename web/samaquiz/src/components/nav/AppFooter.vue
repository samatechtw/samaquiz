<template>
  <footer>
    <div class="footer-wrap">
      <div class="footer-content">
        <div class="footer-left f-col">
          <div class="footer-title">
            {{ ts('footer.title') }}
          </div>
          <div class="footer-text">
            {{ ts('footer.text') }}
          </div>
          <div class="socials">
            <a href="https://x.com/samatechtw" target="_blank">
              <div class="social">
                <Twitter />
              </div>
            </a>
          </div>
          <div class="pets desktop">
            <a target="_blank" href="https://www.vecteezy.com/free-vector/pets">
              Pets Vectors by Vecteezy
            </a>
          </div>
          <div class="copyright desktop">
            {{ ts('footer.copyright') }}
          </div>
        </div>
        <div class="footer-right f-col">
          <div class="f-col">
            <div class="footer-links">
              <router-link
                :to="{ name: 'Home' }"
                class="link"
                :class="{ active: route.name === 'Home' }"
              >
                {{ ts('footer.home') }}
              </router-link>
              <div class="bottom-divider"></div>
              <router-link
                :to="{ name: 'About' }"
                class="link"
                :class="{ active: route.name === 'About' }"
              >
                {{ ts('footer.about') }}
              </router-link>
              <!--
              <router-link
                :to="{ name: 'Faq' }"
                class="link"
                :class="{ active: route.name === 'Faq' }"
              >
                {{ ts('footer.faq') }}
              </router-link>
              -->
            </div>
            <KofiButton class="kofi" />
          </div>
          <div class="f-col">
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
            <div class="copyright mobile">
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
import { Medium, Twitter } from '@frontend/components/svg'
import { KofiButton } from '@frontend/components/widgets'
import { ts } from '../../i18n'

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
  justify-content: center;
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
.footer-links {
  display: flex;
  align-items: center;
  justify-content: center;
}
.kofi {
  margin-top: 12px;
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
  padding: 40px 40px 32px;
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
.footer-right {
  margin-left: auto;
  align-items: center;
  justify-content: space-between;
}
.pets {
  @mixin title 13px;
  letter-spacing: 0.03em;
  margin-top: auto;
  padding-top: 6px;
}
.copyright {
  @mixin title 13px;
  letter-spacing: 0.03em;
  margin-top: 4px;
}
.link {
  @mixin title 17px;
  padding: 6px 0;
  color: white;
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
  .footer-top {
    flex-direction: column;
    align-items: center;
  }
  .footer-left {
    max-width: 600px;
    text-align: center;
  }
  .footer-wrap {
    overflow: hidden;
  }
  .footer-text {
    max-width: 240px;
  }
}
@media (max-width: 560px) {
  .footer-left {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .footer-content {
    flex-direction: column;
    align-items: center;
  }
  .footer-right {
    margin: 24px 0 0 0;
  }
  .bottom-links {
    margin: 16px 0;
  }
  .bottom-link {
    text-align: center;
  }
  .bottom-divider {
    margin: 0 10px;
  }
  .desktop {
    display: none;
  }
  .mobile {
    display: block;
  }
}
</style>

import Vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vitest/config'
import { tsconfigBaseAliases } from '../samaquiz/tsconfig-base-aliases'

export default defineConfig({
  assetsInclude: /\.(pdf|jpg|png|webm|mp4|svg|wasm)$/,
  plugins: [Vue()],
  resolve: {
    preserveSymlinks: true,
    alias: {
      ...tsconfigBaseAliases(__dirname),
    },
  },
  test: {
    setupFiles: ['./vitest-setup.ts'],
    isolate: false,
    fileParallelism: false,
  },
})

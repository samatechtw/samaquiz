import Vue from '@vitejs/plugin-vue'
import terser from '@rollup/plugin-terser'
import { defineConfig } from 'vite'
import path from 'path'
import { tsconfigBaseAliases } from './tsconfig-base-aliases'

const resolve = (p: string): string => path.resolve(__dirname, p)

export default defineConfig({
  assetsInclude: /\.(pdf|jpg|png|webm|mp4|svg|wasm)$/,
  plugins: [Vue()],
  resolve: {
    preserveSymlinks: true,
    alias: {
      '@theme/': `${resolve('./src/theme')}/`,
      ...tsconfigBaseAliases(__dirname),
    },
  },
  server: {
    port: 8080,
  },
  build: {
    outDir: './dist',
    emptyOutDir: true,
    sourcemap: true,
    minify: true,
    rollupOptions: {
      plugins: [terser()],
      output: {
        format: 'es',
        dir: 'dist',
      },
    },
  },
})

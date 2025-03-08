import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
  ],
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'WenfoxComponentLib',
      fileName: (format) => `wenfox_components.${format}.js`
    },
    rollupOptions: {
      external: ['vue', 'pinia'],
      output: {
      globals: {
        vue: 'Vue',
        pinia: 'Pinia'
      }
    }
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})

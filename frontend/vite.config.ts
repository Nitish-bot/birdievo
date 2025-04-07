import { defineConfig } from 'vite'
import { sveltePreprocess } from 'svelte-preprocess'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({
    preprocess: [sveltePreprocess()],
    onwarn(warning, defaultHandler) {
      defaultHandler?.(warning)
    }
  })],
  build: {
    target: 'esnext',
  },
  optimizeDeps: {
    exclude: ['simulation-wasm'],
  },
})

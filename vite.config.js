import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: 'es2021',
  },
  plugins: [svelte()],
  server: {
    port: 3000,
  },
  preview: {
    port: 8080,
  },
});

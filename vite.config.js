import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: "chrome65",
  },
  plugins: [vue()],
  preview: {
    port: 8080,
  },
});

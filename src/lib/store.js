import { writable } from 'svelte/store';
import { resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/api/fs';

export const store = writable({
  config: {
    loaded: false,
  },
});

resolveResource('resources/config.json')
  .then((resourceConfigPath) => readTextFile(resourceConfigPath))
  .then((data) => JSON.parse(data))
  .then((config) => {
    config.loaded = true;
    store.set({ config: config });
  })
  .catch((error) => window.console.error(error));

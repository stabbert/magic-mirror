import { writable } from 'svelte/store';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/api/fs';

export const store = writable({
  config: {
    loaded: false,
  },
});

appConfigDir()
  .then((appConfigDir) => readTextFile(appConfigDir + "config.json"))
  .then((data) => JSON.parse(data))
  .then((config) => {
    config.loaded = true;
    store.set({ config: config });
  })
  .catch((error) => window.console.error(error));

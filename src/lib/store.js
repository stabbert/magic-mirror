import { writable } from 'svelte/store';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/plugin-fs';

export const store = writable({
  config: {
    loaded: false,
  },
});

appConfigDir()
  .then((appConfigDir) => readTextFile(appConfigDir + '/config.json'))
  .then((data) => JSON.parse(data))
  .then((config) => {
    config.loaded = true;
    config.common = {
      language: 'de'
    }
    store.set({ config: config });
  })
  .catch((error) => window.console.error(error));

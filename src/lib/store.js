import { writable } from 'svelte/store';

export const store = writable({
  config: {
    loaded: false,
  },
});

fetch('config.json')
  .then((response) => response.json())
  .then((config) => {
    config.loaded = true;
    store.set({ config: config });
  })
  .catch((error) => window.console.error(error));

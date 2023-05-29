module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: ['eslint:recommended', 'plugin:svelte/recommended', 'prettier'],
  parserOptions: {
    sourceType: 'module',
  },
  root: true,
  rules: {
    'svelte/no-at-html-tags': 'off',
  },
};

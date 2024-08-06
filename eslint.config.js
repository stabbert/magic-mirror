import globals from 'globals';
import js from '@eslint/js';
import eslintPluginSvelte from 'eslint-plugin-svelte';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';

export default [
  js.configs.recommended,
  ...eslintPluginSvelte.configs['flat/recommended'],
  eslintPluginPrettierRecommended,
  {
    files: ['**/*.js', '**/*.svelte'],
    languageOptions: {
      ecmaVersion: 2021,
      globals: {
        ...globals.browser,
      },
      sourceType: 'module',
    },
    rules: {
      'svelte/no-at-html-tags': 'off',
    },
  },
];

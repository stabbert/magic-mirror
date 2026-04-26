export default {
  plugins: ['prettier-plugin-svelte'],
  overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }],
  printWidth: 120,
  semi: true,
  singleQuote: true,
  trailingComma: 'all',
};

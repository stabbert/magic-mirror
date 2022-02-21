module.exports = {
  root: true,
  env: {
    es2021: true,
  },
  extends: ["eslint:recommended", "plugin:vue/vue3-recommended", "prettier"],
  rules: {
    "vue/multi-word-component-names": "off",
  },
};

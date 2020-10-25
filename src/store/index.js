import { createStore } from "vuex";

const LOAD_CONFIG = "LOAD_CONFIG";

export default createStore({
  state: {
    config: {
      loaded: false
    }
  },
  mutations: {
    [LOAD_CONFIG](state, config) {
      config.loaded = true;
      state.config = config;
    }
  },
  actions: {
    loadConfig({ commit }) {
      return fetch("config.json")
        .then(response => response.json())
        .then(jsonConfig => commit(LOAD_CONFIG, jsonConfig))
        .catch(error => window.console.error(error));
    }
  }
});

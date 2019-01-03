import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

const LOAD_CONFIG = "LOAD_CONFIG";

export default new Vuex.Store({
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

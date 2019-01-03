import Vue from "vue";
import App from "./App.vue";
import store from "./store";
import "./assets/css/main.css";
import "./assets/fonts/roboto.css";

Vue.config.productionTip = false;

new Vue({
  store,
  render: h => h(App),
  created: function() {
    this.$store.dispatch("loadConfig");
  }
}).$mount("#app");

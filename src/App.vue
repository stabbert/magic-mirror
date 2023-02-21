<template>
  <div class="region top bar">
    <div class="region top left">
      <div class="container">
        <clock />
      </div>
    </div>
    <div class="region top right">
      <div class="container">
        <weather v-if="$store.state.config.loaded" />
      </div>
    </div>
  </div>
  <div class="region upper third">
    <div class="container">
      <weather-forecast v-if="$store.state.config.loaded" />
    </div>
  </div>
  <div class="region lower third">
    <div class="container">
      <calendar v-if="$store.state.config.loaded" />
    </div>
  </div>
</template>

<script>
import Calendar from "./components/Calendar.vue";
import Clock from "./components/Clock.vue";
import Weather from "./components/Weather.vue";
import WeatherForecast from "./components/WeatherForecast.vue";

export default {
  name: "App",
  components: {
    Calendar,
    Clock,
    Weather,
    WeatherForecast,
  },
  created() {
    this.$store.dispatch("loadConfig");
  },
};
</script>

<style>
body {
  zoom: 220%;
}
</style>

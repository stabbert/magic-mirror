<template>
  <div id="app">
    <div class="region fullscreen below"><div class="container"></div></div>
    <!-- <HelloWorld msg="Welcome to Your Vue.js App"/> -->
    <div class="region top bar">
		  <div class="container"></div>
		  <div class="region top left">
        <div class="container">
          <Clock/>
        </div>
      </div>
		  <div class="region top right">
        <div class="container">
          <Weather v-if="$store.state.config.loaded"/>
        </div>
      </div>
	  </div>
	  <div class="region upper third">
      <div class="container">
        <WeatherForecast v-if="$store.state.config.loaded"/>
      </div>
    </div>
	  <div class="region lower third">
      <div class="container">
        <Calendar v-if="$store.state.config.loaded"/>
      </div>
    </div>
	  <div class="region bottom bar">
		  <div class="container"></div>
		  <div class="region bottom left"><div class="container"></div></div>
		  <div class="region bottom center"><div class="container"></div></div>
		  <div class="region bottom right"><div class="container"></div></div>
	  </div>
  </div>
</template>

<script>
import Calendar from "./components/Calendar.vue";
import Clock from "./components/Clock.vue";
import Weather from "./components/Weather.vue";
import WeatherForecast from "./components/WeatherForecast.vue";

export default {
  name: "app",
  components: {
    Calendar,
    Clock,
    Weather,
    WeatherForecast
  }
};
</script>

<style>
body {
  zoom: 220%;
}
</style>

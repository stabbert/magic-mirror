<script>
  import Calendar from './lib/components/Calendar.svelte';
  import Clock from './lib/components/Clock.svelte';
  import Weather from './lib/components/Weather.svelte';
  import WeatherForecast from './lib/components/WeatherForecast.svelte';
  import { store } from './lib/store';
</script>

{#if $store.config.loaded}
  <div class="container">
    <div class="clock-item">
      <Clock />
    </div>
    <div class="weather-item">
      <Weather />
    </div>
    <div class="weatherforecast-item">
      <WeatherForecast />
    </div>
    <div class="calendar-item">
      <Calendar />
    </div>
  </div>
{/if}

<style>
  .container {
    display: grid;
    grid-template-rows: auto auto auto;
    grid-template-columns: 50% 50%;
    grid-template-areas:
      'clock weather'
      'weather-forecast weather-forecast'
      'calendar calendar';
    row-gap: 3rem;
  }

  .calendar-item {
    grid-area: calendar;
  }

  .clock-item {
    grid-area: clock;
  }

  .weather-item {
    grid-area: weather;
    text-align: right;
  }

  .weatherforecast-item {
    grid-area: weather-forecast;
  }
</style>

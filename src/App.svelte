<script>
  import Calendar from './lib/components/Calendar.svelte';
  import Clock from './lib/components/Clock.svelte';
  import Weather from './lib/components/Weather.svelte';
  import WeatherForecast from './lib/components/WeatherForecast.svelte';
  import { store } from './lib/store';
</script>

{#if $store.config.loaded}
  <div class="region top bar">
    <div class="region top left">
      <div class="container">
        <Clock />
      </div>
    </div>
    <div class="region top right">
      <div class="container">
        <Weather />
      </div>
    </div>
  </div>
  <div class="region upper third">
    <div class="container">
      <WeatherForecast />
    </div>
  </div>
  <div class="region lower third">
    <div class="container">
      <Calendar />
    </div>
  </div>
{/if}

<style>
  :global(body) {
    zoom: 220%;
  }
</style>

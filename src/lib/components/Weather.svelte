<script>
  import '../../../node_modules/@fortawesome/fontawesome-free/css/fontawesome.min.css';
  import '../../../node_modules/@fortawesome/fontawesome-free/css/solid.min.css';
  import '../../../node_modules/weathericons/css/weather-icons.min.css';
  import { onMount } from 'svelte';
  import { store } from '../store';
  import { fetch } from '@tauri-apps/plugin-http';

  const config = $store.config.weather;
  const language = $store.config.common.language;

  const weather = $state({
    sunriseSunsetIcon: '',
    sunriseSunsetTime: '',
    temperature: '',
    weatherType: '',
    windDeg: 0,
    windSpeed: '',
  });

  const ICON_TABLE = {
    '01d': 'wi-day-sunny',
    '02d': 'wi-day-cloudy',
    '03d': 'wi-cloudy',
    '04d': 'wi-cloudy-windy',
    '09d': 'wi-showers',
    '10d': 'wi-rain',
    '11d': 'wi-thunderstorm',
    '13d': 'wi-snow',
    '50d': 'wi-fog',
    '01n': 'wi-night-clear',
    '02n': 'wi-night-cloudy',
    '03n': 'wi-night-cloudy',
    '04n': 'wi-night-cloudy',
    '09n': 'wi-night-showers',
    '10n': 'wi-night-rain',
    '11n': 'wi-night-thunderstorm',
    '13n': 'wi-night-snow',
    '50n': 'wi-night-alt-cloudy-windy',
  };

  const SUNSET_ICON = 'wi-sunset';
  const SUNRISE_ICON = 'wi-sunrise';

  const TIME_FORMAT = new Intl.DateTimeFormat(language, {
    hour: '2-digit',
    minute: '2-digit',
  });

  const OPENWEATHER_URL =
    'https://api.openweathermap.org/data/2.5/weather?id=' +
    config.locationID +
    '&q=' +
    config.location +
    '&APPID=' +
    config.appid +
    '&units=metric&lang=de';

  function updateWeather() {
    fetch(OPENWEATHER_URL)
      .then((response) => response.json())
      .then((data) => {
        const now = Date.now();
        const zero = 0;
        const one = 1;
        const thousand = 1000;

        const sunrise = data.sys.sunrise * thousand;
        const sunset = data.sys.sunset * thousand;

        const isSunset = sunrise < now && sunset > now;

        const sunriseSunsetDate = isSunset ? sunset : sunrise;
        const sunriseSunsetTime = TIME_FORMAT.format(sunriseSunsetDate);

        weather.sunriseSunsetIcon = isSunset ? SUNSET_ICON : SUNRISE_ICON;
        weather.sunriseSunsetTime = sunriseSunsetTime;
        weather.temperature = parseFloat(data.main.temp).toFixed(one);
        weather.weatherType = ICON_TABLE[data.weather[zero].icon];
        weather.windDeg = data.wind.deg;
        weather.windSpeed = parseFloat(data.wind.speed).toFixed(zero);
      })
      .catch((error) => window.console.error(error));
  }

  onMount(() => {
    updateWeather();

    const updateIntervalInMs = config.updateIntervalInMinutes * 60 * 1000;
    const intervalId = setInterval(updateWeather, updateIntervalInMs);

    return () => clearInterval(intervalId);
  });
</script>

<div class="normal medium">
  <i class="wi wi-strong-wind dimmed"></i>
  <span>{weather.windSpeed}</span>
  <i class="fa-solid fa-location-arrow" style="transform:rotate({weather.windDeg - 225}deg);"></i>
  <i class="wi dimmed {weather.sunriseSunsetIcon}"></i>
  <span>{weather.sunriseSunsetTime}</span>
</div>
<div class="large light">
  <i class="wi weathericon {weather.weatherType}"></i>
  <span class="bright">{weather.temperature}&deg;C</span>
</div>

<style>
  .weathericon {
    font-size: 75%;
    transform: translate(0, -6px);
  }
</style>

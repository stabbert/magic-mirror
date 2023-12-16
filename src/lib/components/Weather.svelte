<script>
  import '../../../node_modules/@fortawesome/fontawesome-free/css/fontawesome.min.css';
  import '../../../node_modules/@fortawesome/fontawesome-free/css/solid.min.css';
  import '../../../node_modules/weathericons/css/weather-icons.min.css';
  import moment from 'moment/min/moment-with-locales';
  import { onMount } from 'svelte';
  import { store } from '../store';
  import { fetch } from '@tauri-apps/api/http';

  let weather = {
    windSpeed: '',
    windDeg: 0,
    sunriseSunsetIcon: '',
    sunriseSunsetTime: '',
    weatherType: '',
    temperature: '',
  };

  const iconTable = {
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

  const config = $store.config.weather;

  let openweatherUrl =
    'https://api.openweathermap.org/data/2.5/weather?id=' +
    config.locationID +
    '&q=' +
    config.location +
    '&APPID=' +
    config.appid +
    '&units=metric&lang=de';

  function updateWeather() {
    fetch(openweatherUrl, { method: 'GET', responseType: 1 })
      .then((response) => response.data)
      .then((data) => {
        let now = new Date();
        let sunrise = new Date(data.sys.sunrise * 1000);
        let sunset = new Date(data.sys.sunset * 1000);

        let sunriseSunsetDateObject = sunrise < now && sunset > now ? sunset : sunrise;
        let sunriseSunsetTimeString = moment(sunriseSunsetDateObject).format('HH:mm');

        weather.windSpeed = parseFloat(data.wind.speed).toFixed(0);
        weather.windDeg = data.wind.deg;
        weather.sunriseSunsetIcon = sunrise < now && sunset > now ? 'wi-sunset' : 'wi-sunrise';
        weather.sunriseSunsetTime = sunriseSunsetTimeString;
        weather.weatherType = iconTable[data.weather[0].icon];
        weather.temperature = parseFloat(data.main.temp).toFixed(1);
      })
      .catch((error) => window.console.error(error));
  }

  onMount(() => {
    updateWeather();

    const interval = setInterval(updateWeather, config.updateIntervall);

    return () => clearInterval(interval);
  });
</script>

<div class="normal medium">
  <i class="wi wi-strong-wind dimmed" />
  <span>{weather.windSpeed}</span>
  <i class="fa-solid fa-location-arrow" style="transform:rotate({weather.windDeg - 225}deg);" />
  <i class="wi dimmed {weather.sunriseSunsetIcon}" />
  <span>{weather.sunriseSunsetTime}</span>
</div>
<div class="large light">
  <i class="wi weathericon {weather.weatherType}" />
  <span class="bright">{weather.temperature}&deg;C</span>
</div>

<style>
  .weathericon {
    font-size: 75%;
    transform: translate(0, -3px);
  }
</style>

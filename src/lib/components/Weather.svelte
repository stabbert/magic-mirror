<script>
  import '../../../node_modules/weathericons/css/weather-icons.min.css';
  import moment from 'moment/min/moment-with-locales';
  import { onMount } from 'svelte';
  import { store } from '../store';

  let weather = {
    windSpeed: '',
    windDirection: '',
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

  function deg2Cardinal(deg) {
    if (deg > 11.25 && deg <= 33.75) {
      return 'NNO';
    } else if (deg > 33.75 && deg <= 56.25) {
      return 'NO';
    } else if (deg > 56.25 && deg <= 78.75) {
      return 'ONO';
    } else if (deg > 78.75 && deg <= 101.25) {
      return 'O';
    } else if (deg > 101.25 && deg <= 123.75) {
      return 'OSO';
    } else if (deg > 123.75 && deg <= 146.25) {
      return 'SO';
    } else if (deg > 146.25 && deg <= 168.75) {
      return 'SSO';
    } else if (deg > 168.75 && deg <= 191.25) {
      return 'S';
    } else if (deg > 191.25 && deg <= 213.75) {
      return 'SSW';
    } else if (deg > 213.75 && deg <= 236.25) {
      return 'SW';
    } else if (deg > 236.25 && deg <= 258.75) {
      return 'WSW';
    } else if (deg > 258.75 && deg <= 281.25) {
      return 'W';
    } else if (deg > 281.25 && deg <= 303.75) {
      return 'WNW';
    } else if (deg > 303.75 && deg <= 326.25) {
      return 'NW';
    } else if (deg > 326.25 && deg <= 348.75) {
      return 'NNW';
    } else {
      return 'N';
    }
  }

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
    fetch(openweatherUrl)
      .then((response) => response.json())
      .then((data) => {
        let now = new Date();
        let sunrise = new Date(data.sys.sunrise * 1000);
        let sunset = new Date(data.sys.sunset * 1000);

        let sunriseSunsetDateObject = sunrise < now && sunset > now ? sunset : sunrise;
        let sunriseSunsetTimeString = moment(sunriseSunsetDateObject).format('HH:mm');

        weather.windSpeed = parseFloat(data.wind.speed).toFixed(0);
        weather.windDirection = deg2Cardinal(data.wind.deg);
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
  <span class="wi wi-strong-wind dimmed" />
  <span>{weather.windSpeed}</span>
  <sup>{weather.windDirection}</sup>
  <span class="wi dimmed {weather.sunriseSunsetIcon}" />
  <span>{weather.sunriseSunsetTime}</span>
</div>
<div class="large light">
  <span class="wi weathericon {weather.weatherType}" />
  <span class="bright">{weather.temperature}&deg;C</span>
</div>

<style>
  .weathericon {
    font-size: 75%;
    transform: translate(0, -3px);
  }
</style>

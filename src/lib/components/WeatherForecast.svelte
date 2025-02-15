<script>
  import '../../../node_modules/weathericons/css/weather-icons.min.css';
  import { onMount } from 'svelte';
  import { store } from '../store';
  import { fetch } from '@tauri-apps/plugin-http';

  const config = $store.config.weatherForecast;
  const language = $store.config.common.language;

  const weatherForecast = $state({
    header: '',
    forecasts: [],
  });

  weatherForecast.header = config.header;

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

  /* Use the parse to keep the same struct between daily and forecast Endpoint from Openweather */
  function parserDataWeather(data) {
    if (Object.prototype.hasOwnProperty.call(data, 'main')) {
      data['temp'] = { min: data.main.temp_min, max: data.main.temp_max };
    }
    return data;
  }

  const DATE_FORMAT = new Intl.DateTimeFormat(language, {
    weekday: 'short',
  });

  const OPENWEATHER_URL =
    'https://api.openweathermap.org/data/2.5/forecast?id=' +
    config.locationID +
    '&APPID=' +
    config.appid +
    '&units=metric&lang=de';

  function updateWeatherForecast() {
    fetch(OPENWEATHER_URL)
      .then((response) => response.json())
      .then((data) => {
        const zero = 0;
        const one = 1;
        const thousand = 1000;
        const newForecasts = [];
        let forecastData = {};
        let lastDay = null;
        let duplicateIcons = {};
        let highestOccurrenceIconCount = zero;

        for (let index in data.list) {
          let forecast = data.list[index];

          parserDataWeather(forecast); // hack issue #1017

          const forecastDate = new Date(forecast.dt * thousand)
          const day = DATE_FORMAT.format(forecastDate);

          if (day === lastDay) {
            //Log.log("Compare max: ", forecast.temp.max, parseFloat(forecastData.maxTemp));
            forecastData.maxTemp =
              forecast.temp.max > parseFloat(forecastData.maxTemp)
                ? parseFloat(forecast.temp.max).toFixed(one)
                : forecastData.maxTemp;
            //Log.log("Compare min: ", forecast.temp.min, parseFloat(forecastData.minTemp));
            forecastData.minTemp =
              forecast.temp.min < parseFloat(forecastData.minTemp)
                ? parseFloat(forecast.temp.min).toFixed(one)
                : forecastData.minTemp;

            // Since we don't want an icon from the start of the day (in the middle of the night)
            // we update the icon as long as it's somewhere during the day with the highest occurrence.
            const hour = forecastDate.getHours();
            if (hour >= 8 && hour <= 18) {
              const icon = ICON_TABLE[forecast.weather[zero].icon];
              const duplicateIconCount = duplicateIcons[icon] + one || one;
              duplicateIcons[icon] = duplicateIconCount;
              if (highestOccurrenceIconCount <= duplicateIconCount) {
                forecastData.icon = icon;
                highestOccurrenceIconCount = duplicateIconCount;
              }
            }
          } else {
            // Stop processing when maxNumberOfDays is reached
            if (forecast.length === config.maxNumberOfDays) {
              break;
            }

            const icon = ICON_TABLE[forecast.weather[zero].icon];

            forecastData = {
              day: day,
              icon: icon,
              maxTemp: parseFloat(forecast.temp.max).toFixed(one),
              minTemp: parseFloat(forecast.temp.min).toFixed(one),
            };

            newForecasts.push(forecastData);

            lastDay = day;

            duplicateIcons = {};
            duplicateIcons[icon] = one;
            highestOccurrenceIconCount = one;
          }
        }

        const newForecastsSize = newForecasts.length;
        for (let index = zero; index < newForecastsSize; index++) {
          let newForecast = newForecasts[index];
          let opacity = one;

          if (config.fade && config.fadePoint < one) {
            if (config.fadePoint < zero) {
              config.fadePoint = zero;
            }
            let startingPoint = newForecastsSize * config.fadePoint;
            let steps = newForecastsSize - startingPoint;
            if (index >= startingPoint) {
              let currentStep = index - startingPoint;
              opacity = one - (one / steps) * currentStep;
            }
          }

          newForecast.opacity = opacity;
        }

        weatherForecast.forecasts = newForecasts;
      })
      .catch((error) => window.console.error(error));
  }

  onMount(() => {
    updateWeatherForecast();

    const interval = setInterval(updateWeatherForecast, config.updateIntervall);

    return () => clearInterval(interval);
  });
</script>

<header class="normal">{weatherForecast.header}</header>
<table class="small">
  <tbody>
    {#each weatherForecast.forecasts as forecast}
      <tr class="normal" style:opacity={forecast.opacity}>
        <td class="bright day">{forecast.day}</td>
        <td class="bright weather-icon">
          <i class="wi {forecast.icon}"></i>
        </td>
        <td class="bright temperatur">
          {forecast.maxTemp}&deg;C
        </td>
        <td class="temperatur">{forecast.minTemp}&deg;C</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .day {
    padding-left: 0;
    text-align: left;
  }

  .weather-icon {
    text-align: center;
  }

  .temperatur {
    padding: 0;
    text-align: right;
  }
</style>

<script>
  import '../../../node_modules/weathericons/css/weather-icons.min.css';
  import { onMount } from 'svelte';
  import { store } from '../store';
  import { fetch } from '@tauri-apps/plugin-http';

  const config = $store.config.weatherForecast;
  const language = $store.config.common.language;

  const header = config.header;
  let weatherForecasts = $state([]);

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
  function parseWeatherForecast(data) {
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
        const eight = 8;
        const eighteen = 18;
        const thousand = 1000;
        const newWeatherForecasts = [];
        let weatherForecastData = {};
        let lastDay = null;
        let duplicateIcons = {};
        let highestOccurrenceIconCount = zero;

        const dataListSize = data.list.length;
        for (let index = 0; index < dataListSize; index++) {
          let weatherForecast = data.list[index];

          parseWeatherForecast(weatherForecast);

          const weatherForecastDate = new Date(weatherForecast.dt * thousand);
          const day = DATE_FORMAT.format(weatherForecastDate);

          if (day === lastDay) {
            weatherForecastData.maxTemp =
              weatherForecast.temp.max > parseFloat(weatherForecastData.maxTemp)
                ? parseFloat(weatherForecast.temp.max).toFixed(one)
                : weatherForecastData.maxTemp;
            weatherForecastData.minTemp =
              weatherForecast.temp.min < parseFloat(weatherForecastData.minTemp)
                ? parseFloat(weatherForecast.temp.min).toFixed(one)
                : weatherForecastData.minTemp;

            // Since we don't want an icon from the start of the day (in the middle of the night)
            // we update the icon as long as it's somewhere during the day with the highest occurrence.
            const hour = weatherForecastDate.getHours();
            if (hour >= eight && hour <= eighteen) {
              const icon = ICON_TABLE[weatherForecast.weather[zero].icon];
              const duplicateIconCount = duplicateIcons[icon] + one || one;
              duplicateIcons[icon] = duplicateIconCount;
              if (highestOccurrenceIconCount <= duplicateIconCount) {
                weatherForecastData.icon = icon;
                highestOccurrenceIconCount = duplicateIconCount;
              }
            }
          } else {
            // Stop processing when maxNumberOfDays is reached
            if (weatherForecast.length === config.maxNumberOfDays) {
              break;
            }

            const icon = ICON_TABLE[weatherForecast.weather[zero].icon];

            weatherForecastData = {
              id: weatherForecast.dt,
              day: day,
              icon: icon,
              maxTemp: parseFloat(weatherForecast.temp.max).toFixed(one),
              minTemp: parseFloat(weatherForecast.temp.min).toFixed(one),
            };

            newWeatherForecasts.push(weatherForecastData);

            lastDay = day;

            duplicateIcons = {};
            duplicateIcons[icon] = one;
            highestOccurrenceIconCount = one;
          }
        }

        const newWeatherForecastsSize = newWeatherForecasts.length;
        for (let index = zero; index < newWeatherForecastsSize; index++) {
          const newWeatherForecast = newWeatherForecasts[index];
          let opacity = one;

          if (config.fade && config.fadePoint < one) {
            if (config.fadePoint < zero) {
              config.fadePoint = zero;
            }
            let startingPoint = newWeatherForecastsSize * config.fadePoint;
            let steps = newWeatherForecastsSize - startingPoint;
            if (index >= startingPoint) {
              let currentStep = index - startingPoint;
              opacity = one - (one / steps) * currentStep;
            }
          }

          newWeatherForecast.opacity = opacity;
        }

        weatherForecasts = newWeatherForecasts;
      })
      .catch((error) => window.console.error(error));
  }

  onMount(() => {
    updateWeatherForecast();

    const updateIntervalInMs = config.updateIntervalInMinutes * 60 * 1000;
    const intervalId = setInterval(updateWeatherForecast, updateIntervalInMs);

    return () => clearInterval(intervalId);
  });
</script>

<header class="normal">{header}</header>
<table class="small">
  <tbody>
    {#each weatherForecasts as weatherForecast (weatherForecast.id)}
      <tr class="normal" style:opacity={weatherForecast.opacity}>
        <td class="bright day">{weatherForecast.day}</td>
        <td class="bright weather-icon">
          <i class="wi {weatherForecast.icon}"></i>
        </td>
        <td class="bright temperatur">
          {weatherForecast.maxTemp}&deg;C
        </td>
        <td class="temperatur">{weatherForecast.minTemp}&deg;C</td>
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

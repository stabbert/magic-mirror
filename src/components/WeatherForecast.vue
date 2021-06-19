<template>
  <div class="weatherforecast">
    <header>{{ header }}</header>
    <table class="small">
      <tr
        class="normal"
        v-bind:style="{ opacity: forecast.opacity }"
        v-for="forecast in forecasts"
        :key="forecast.$index"
      >
        <td class="align-left bright day">{{ forecast.day }}</td>
        <td class="bright weather-icon">
          <span class="wi" v-bind:class="forecast.icon"></span>
        </td>
        <td class="align-right bright max-temp">
          {{ forecast.maxTemp }}&deg;C
        </td>
        <td class="align-right min-temp">{{ forecast.minTemp }}&deg;C</td>
      </tr>
    </table>
  </div>
</template>

<script>
import moment from "moment";

const iconTable = {
  "01d": "wi-day-sunny",
  "02d": "wi-day-cloudy",
  "03d": "wi-cloudy",
  "04d": "wi-cloudy-windy",
  "09d": "wi-showers",
  "10d": "wi-rain",
  "11d": "wi-thunderstorm",
  "13d": "wi-snow",
  "50d": "wi-fog",
  "01n": "wi-night-clear",
  "02n": "wi-night-cloudy",
  "03n": "wi-night-cloudy",
  "04n": "wi-night-cloudy",
  "09n": "wi-night-showers",
  "10n": "wi-night-rain",
  "11n": "wi-night-thunderstorm",
  "13n": "wi-night-snow",
  "50n": "wi-night-alt-cloudy-windy"
};

/* parserDataWeather(data)
 *
 * Use the parse to keep the same struct between daily and forecast Endpoint
 * from Openweather
 *
 */
function parserDataWeather(data) {
  if (Object.prototype.hasOwnProperty.call(data, "main")) {
    data["temp"] = { min: data.main.temp_min, max: data.main.temp_max };
  }
  return data;
}

export default {
  name: "Weather-Forecast",
  data() {
    return {
      header: "",
      forecasts: []
    };
  },
  created() {
    const config = this.$store.state.config.weatherForecast;
    let self = this;
    self.header = config.header;

    let openweatherUrl =
      "https://api.openweathermap.org/data/2.5/forecast?id=" +
      config.locationID +
      "&APPID=" +
      config.appid +
      "&units=metric&lang=de";

    function updateWeatherForecast() {
      fetch(openweatherUrl)
        .then(response => response.json())
        .then(data => {
          let newForecasts = [];
          let lastDay = null;
          let forecastData = {};

          let duplicateIcons = {};
          let highestOccurrenceIconCount = 0;

          for (let index in data.list) {
            let forecast = data.list[index];

            parserDataWeather(forecast); // hack issue #1017

            const forecastDate = moment(forecast.dt, "X");
            const day = forecastDate.format("ddd");

            if (day === lastDay) {
              //Log.log("Compare max: ", forecast.temp.max, parseFloat(forecastData.maxTemp));
              forecastData.maxTemp =
                forecast.temp.max > parseFloat(forecastData.maxTemp)
                  ? parseFloat(forecast.temp.max).toFixed(1)
                  : forecastData.maxTemp;
              //Log.log("Compare min: ", forecast.temp.min, parseFloat(forecastData.minTemp));
              forecastData.minTemp =
                forecast.temp.min < parseFloat(forecastData.minTemp)
                  ? parseFloat(forecast.temp.min).toFixed(1)
                  : forecastData.minTemp;

              // Since we don't want an icon from the start of the day (in the middle of the night)
              // we update the icon as long as it's somewhere during the day with the highest occurrence.
              const hour = forecastDate.hour();
              if (hour >= 8 && hour <= 18) {
                const icon = iconTable[forecast.weather[0].icon];
                const duplicateIconCount = duplicateIcons[icon] + 1 || 1;
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

              const icon = iconTable[forecast.weather[0].icon];

              forecastData = {
                day: day,
                icon: icon,
                maxTemp: parseFloat(forecast.temp.max).toFixed(1),
                minTemp: parseFloat(forecast.temp.min).toFixed(1)
              };

              newForecasts.push(forecastData);

              lastDay = day;

              duplicateIcons = {};
              duplicateIcons[icon] = 1;
              highestOccurrenceIconCount = 1;
            }
          }

          newForecasts.forEach(function(newForecast, index) {
            let opacity = 1;

            if (config.fade && config.fadePoint < 1) {
              if (config.fadePoint < 0) {
                config.fadePoint = 0;
              }
              let startingPoint = newForecasts.length * config.fadePoint;
              let steps = newForecasts.length - startingPoint;
              if (index >= startingPoint) {
                let currentStep = index - startingPoint;
                opacity = 1 - (1 / steps) * currentStep;
              }
            }

            newForecast.opacity = opacity;
          });

          self.forecasts = newForecasts;
        })
        .catch(error => window.console.error(error));
    }

    setTimeout(updateWeatherForecast);
    setInterval(updateWeatherForecast, config.updateIntervall);
  }
};
</script>

<style scoped>
@import "../../node_modules/weathericons/css/weather-icons.min.css";

.weatherforecast .day {
  padding-left: 0;
}

.weatherforecast .weather-icon {
  text-align: center;
}

.weatherforecast .min-temp {
  padding: 0;
}

.weatherforecast .max-temp {
  padding: 0;
}
</style>

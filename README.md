# Magic Mirror implementation using Svelte and Tauri

## Restrictions

- supports only German language

# Modules

- calendar
- time
- weather 
- weather forecast.

## Release

The released Tauri application is created for Debian Trixi.

## Configuration

The application can be configured in the `~/.config/com.tauri.magicmirror/config.json` file for the various modules.

```json
{
  "calendar": {
    "header": "Termine",
    "fade": true,
    "fadePoint": 0.25,
    "maximumEntries": 15,
    "maximumNumberOfDays": 365,
    "updateIntervalInMinutes": 60,
    "calendars": [
      {
        "url": null,
        "auth": {
          "user": "username",
          "pass": "superstrongpassword"
        }
      }
    ]
  },
  "weather": {
    "appid": "YOUR_OPENWEATHER_API_KEY",
    "location": "YOUR_LOCATION",
    "locationID": "YOUR_LOCATION_ID",
    "updateIntervalInMinutes": 10
  },
  "weatherForecast": {
    "header": "Wettervorhersage",
    "fade": true,
    "fadePoint": 0.25,
    "appid": "YOUR_OPENWEATHER_API_KEY",
    "location": "YOUR_LOCATION",
    "locationID": "YOUR_LOCATION_ID",
    "maxNumberOfDays": 7,
    "updateIntervalInMinutes": 120
  }
}
```
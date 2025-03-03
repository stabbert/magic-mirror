<script>
  import '../../../node_modules/@fortawesome/fontawesome-free/css/fontawesome.min.css';
  import '../../../node_modules/@fortawesome/fontawesome-free/css/regular.min.css';
  import '../../../node_modules/@fortawesome/fontawesome-free/css/solid.min.css';
  import ICAL from 'ical.js';
  import { onMount } from 'svelte';
  import { store } from '../store';
  import { fetch } from '@tauri-apps/plugin-http';

  const config = $store.config.calendar;
  const language = $store.config.language;
  const header = config.header;

  let calendarEvents = $state([]);

  const RELATIVE_TIME_FORMAT_ALWAYS = new Intl.RelativeTimeFormat(language, {
    numeric: 'always',
    style: 'long',
  });

  const RELATIVE_TIME_FORMAT_AUTO = new Intl.RelativeTimeFormat(language, {
    numeric: 'auto',
    style: 'long',
  });

  const HOUR_AND_MINUTE_FORMAT = new Intl.DateTimeFormat(language, {
    hour: '2-digit',
    minute: '2-digit',
  });

  const WEEKDAY_FORMAT = new Intl.DateTimeFormat(language, {
    weekday: 'long',
  });

  const RELATIVE_TIME_FORMAT_MINUTE_UNIT = 'minute';
  const RELATIVE_TIME_FORMAT_HOUR_UNIT = 'hour';
  const RELATIVE_TIME_FORMAT_DAY_UNIT = 'day';
  const RELATIVE_TIME_FORMAT_MONTH_UNIT = 'month';

  const ONE_SECOUND_IN_MS = 1000; // 1,000 milliseconds
  const ONE_MINUTE_IN_MS = ONE_SECOUND_IN_MS * 60;
  const ONE_HOUR_IN_MS = ONE_MINUTE_IN_MS * 60;
  const ONE_DAY_IN_MS = ONE_HOUR_IN_MS * 24;
  const TWO_DAY_IN_MS = ONE_DAY_IN_MS * 2;

  const ONE_HOUR_IN_MINUTES = 60;
  const ONE_DAY_IN_MINUTES = 1440;
  const ONE_MONTH_IN_MINUTES = 43200;

  const sanitizeUnsafeXssCharacterReplacements = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#x27;',
    '/': '&#x2F;',
  };

  const sanitizeUnsafeXssCharactersRegExp = /[&<>"'/]/gi;

  function sanitize(string) {
    return string.replace(sanitizeUnsafeXssCharactersRegExp, (match) => sanitizeUnsafeXssCharacterReplacements[match]);
  }

  function capitalizeFirstLetter(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
  }

  function isFullDayEvent(event) {
    return event.duration.days > 0 ? true : false;
  }

  function relativeTime(startTimeInMs, nowTimeInMs) {
    const diffInMinutes = (startTimeInMs - nowTimeInMs) / ONE_MINUTE_IN_MS;
    const diffInMinutesAbs = Math.abs(diffInMinutes);

    if (diffInMinutesAbs < ONE_HOUR_IN_MINUTES) {
      return RELATIVE_TIME_FORMAT_ALWAYS.format(Math.round(diffInMinutes), RELATIVE_TIME_FORMAT_MINUTE_UNIT);
    }

    if (diffInMinutes < ONE_DAY_IN_MINUTES) {
      return RELATIVE_TIME_FORMAT_ALWAYS.format(
        Math.round(diffInMinutes / ONE_HOUR_IN_MINUTES),
        RELATIVE_TIME_FORMAT_HOUR_UNIT,
      );
    }

    if (diffInMinutes < ONE_MONTH_IN_MINUTES) {
      return RELATIVE_TIME_FORMAT_ALWAYS.format(
        Math.round(diffInMinutes / ONE_DAY_IN_MINUTES),
        RELATIVE_TIME_FORMAT_DAY_UNIT,
      );
    }

    return RELATIVE_TIME_FORMAT_ALWAYS.format(
      Math.round(diffInMinutes / ONE_MONTH_IN_MINUTES),
      RELATIVE_TIME_FORMAT_MONTH_UNIT,
    );
  }

  function formatAtTime(startTimeInMs) {
    return 'um ' + HOUR_AND_MINUTE_FORMAT.format(startTimeInMs) + ' Uhr';
  }

  function calculateTimeTitle(nowTimeInMs, newEvent) {
    const startTimeInMs = newEvent.startTimeInMs;
    const endTimeInMs = newEvent.endTimeInMs;

    if (newEvent.fullDayEvent) {
      const diffInMs = startTimeInMs - nowTimeInMs;

      if (diffInMs < TWO_DAY_IN_MS) {
        const days = Math.round(diffInMs / ONE_DAY_IN_MS);
        return capitalizeFirstLetter(RELATIVE_TIME_FORMAT_AUTO.format(days, RELATIVE_TIME_FORMAT_DAY_UNIT));
      }
    }

    if (isAfter(startTimeInMs, nowTimeInMs)) {
      const startTimeInMsOnlyDate = startOfDayAsTimeInMs(startTimeInMs);
      const nowTimeInMsOnlyDate = startOfDayAsTimeInMs(nowTimeInMs);
      const diffInMs = startTimeInMsOnlyDate - nowTimeInMsOnlyDate;

      if (diffInMs === 0) {
        return 'Heute ' + formatAtTime(startTimeInMs);
      } else if (diffInMs === ONE_DAY_IN_MS) {
        return 'Morgen ' + formatAtTime(startTimeInMs);
      } else if (diffInMs === TWO_DAY_IN_MS) {
        return WEEKDAY_FORMAT.format(startTimeInMs) + ' ' + formatAtTime(startTimeInMs);
      } else {
        return capitalizeFirstLetter(relativeTime(startTimeInMsOnlyDate, nowTimeInMsOnlyDate));
      }
    } else if (isAfter(endTimeInMs, nowTimeInMs)) {
      const relativeTimeInMinutes = relativeTime(endTimeInMs, nowTimeInMs);
      const relativeTimeInMinutesWithoutSuffix = relativeTimeInMinutes.substring(relativeTimeInMinutes.indexOf(' ') + 1);
      return 'Noch ' + relativeTimeInMinutesWithoutSuffix;
    } else {
      return capitalizeFirstLetter(relativeTime(endTimeInMs, nowTimeInMs));
    }
  }

  function isBefore(timeInMsOne, timeInMsTwo) {
    return timeInMsOne < timeInMsTwo;
  }

  function isAfter(timeInMsOne, timeInMsTwo) {
    return timeInMsOne > timeInMsTwo;
  }

  function isSame(timeInMsOne, timeInMsTwo) {
    return timeInMsOne === timeInMsTwo;
  }

  function isBetween(timeInMs, fromTimeInMs, toTimeInMs) {
    return timeInMs >= fromTimeInMs && timeInMs <= toTimeInMs;
  }

  function startOfDayAsTimeInMs(timeInMs) {
    const date = new Date(timeInMs);
    date.setHours(0, 0, 0, 0);
    return date.getTime();
  }

  function addDaysAsTimeInMs(timeInMs, days) {
    const date = new Date(timeInMs);
    date.setDate(date.getDate() + days);
    return date.getTime();
  }

  function toTimeInMs(icaltime) {
    return icaltime.toJSDate().getTime();
  }

  function fromRuleNextTimeInMs(rule) {
    const next = rule.next();
    return next && toTimeInMs(next);
  }

  function updateCalendar() {
    const nowTimeInMs = Date.now();
    const todayTimeInMs = startOfDayAsTimeInMs(nowTimeInMs);
    const futureTimeInMs = addDaysAsTimeInMs(nowTimeInMs, config.maximumNumberOfDays) - 1000; // Subtract 1 second so that events that start on the middle of the night will not repeat.

    const calendarFetches = [];

    for (let calendar of config.calendars) {
      const url = calendar.url.replace('webcal://', 'https://');
      const auth = calendar.auth;

      const opts = {
        headers: {
          'Access-Control-Allow-Origin': url,
          'Content-Type': 'text/calendar; charset=UTF-8',
        },
        method: 'GET',
      };

      if (auth) {
        if (auth.method === 'bearer') {
          opts.headers['Authorization'] = 'Bearer ' + auth.pass;
        } else {
          opts.headers['Authorization'] = 'Basic ' + btoa(auth.user + ':' + auth.pass);
        }
      }

      calendarFetches.push(
        fetch(new Request(url, opts))
          .then((response) => response.text())
          .then((data) => {
            const parsedICS = ICAL.parse(data);
            const component = new ICAL.Component(parsedICS);
            const allSubcomponents = component.getAllSubcomponents();
            const newEvents = [];
            const shiftedRecurrenceEventMap = {};

            const allSubcomponentsSize = allSubcomponents.length;
            for (let index = 0; index < allSubcomponentsSize; index++) {
              let vevent = allSubcomponents[index];
              let event = new ICAL.Event(vevent);

              if (event.recurrenceId) {
                shiftedRecurrenceEventMap[event.uid] = event;
              }

              if (!event.startDate) {
                continue;
              }

              let startTimeInMs = toTimeInMs(event.startDate);
              let endTimeInMs;

              if (typeof event.endDate !== 'undefined') {
                endTimeInMs = toTimeInMs(event.endDate);
              } else {
                endTimeInMs = startTimeInMs;
              }

              let title = 'Termin';
              if (event.summary) {
                title = typeof event.summary.val !== 'undefined' ? event.summary.val : event.summary;
              } else if (event.description) {
                title = event.description;
              }

              // Replaying event
              if (event.isRecurring()) {
                let timesInMs = [];
                let rule = event.iterator();
                let exceptionTimesInMs = rule.exDates.map((exceptionDate) => toTimeInMs(exceptionDate));
                let shiftedRecurrenceEvent = shiftedRecurrenceEventMap[event.uid];

                let shiftedTimeInMs = shiftedRecurrenceEvent ? toTimeInMs(shiftedRecurrenceEvent.recurrenceId) : null;

                let nextTimeInMs = fromRuleNextTimeInMs(rule);
                while (nextTimeInMs) {
                  if (shiftedTimeInMs && isSame(nextTimeInMs, shiftedTimeInMs)) {
                    nextTimeInMs = fromRuleNextTimeInMs(rule);
                    continue;
                  }
                  if (
                    isBetween(nextTimeInMs, todayTimeInMs, futureTimeInMs) &&
                    exceptionTimesInMs.indexOf(nextTimeInMs) === -1
                  ) {
                    timesInMs.push(nextTimeInMs);
                  }
                  if (timesInMs.length <= config.maximumEntries && isBefore(nextTimeInMs, futureTimeInMs)) {
                    nextTimeInMs = fromRuleNextTimeInMs(rule);
                  } else {
                    nextTimeInMs = false;
                  }
                }

                // calculate the duration of the event for use with recurring events.
                let durationTimeInMs = endTimeInMs - startTimeInMs;

                for (let timeInMs of timesInMs) {
                  startTimeInMs = timeInMs;
                  endTimeInMs = startTimeInMs + durationTimeInMs;

                  if (isAfter(endTimeInMs, nowTimeInMs)) {
                    newEvents.push({
                      title: title,
                      startTimeInMs: startTimeInMs,
                      endTimeInMs: endTimeInMs,
                      fullDayEvent: isFullDayEvent(event),
                    });
                  }
                }
              } else {
                if (isBefore(endTimeInMs, nowTimeInMs)) {
                  continue;
                }

                if (isAfter(startTimeInMs, futureTimeInMs)) {
                  continue;
                }

                // Every thing is good. Add it to the list.
                newEvents.push({
                  title: title,
                  startTimeInMs: startTimeInMs,
                  endTimeInMs: endTimeInMs,
                  fullDayEvent: isFullDayEvent(event),
                });
              }
            }

            newEvents.sort(function (a, b) {
              return a.startTimeInMs - b.startTimeInMs;
            });

            newEvents.splice(config.maximumEntries);

            return newEvents;
          })
          .catch((error) => window.console.error(error)),
      );
    }

    Promise.all(calendarFetches).then((allCalendarNewEvents) => {
      const allNewEvents = [];

      const allCalendarNewEventsSize = allCalendarNewEvents.length;
      for (let allIndex = 0; allIndex < allCalendarNewEventsSize; allIndex++) {
        const calendarNewEvents = allCalendarNewEvents[allIndex];

        const calendarNewEventsSize = calendarNewEvents.length;
        for (let index = 0; index < calendarNewEventsSize; index++) {
          const calendarNewEvent = calendarNewEvents[index];
          allNewEvents.push(calendarNewEvent);
        }
      }

      allCalendarNewEvents = null;

      allNewEvents.sort(function (a, b) {
        return a.startTimeInMs - b.startTimeInMs;
      });

      allNewEvents.splice(config.maximumEntries);

      const allNewEventsSize = allNewEvents.length;
      for (let index = 0; index < allNewEventsSize; index++) {
        const newEvent = allNewEvents[index];

        let opacity = 1;

        if (config.fade && config.fadePoint < 1) {
          if (config.fadePoint < 0) {
            config.fadePoint = 0;
          }
          let startingPoint = allNewEventsSize * config.fadePoint;
          let steps = allNewEventsSize - startingPoint;
          if (index >= startingPoint) {
            let currentStep = index - startingPoint;
            opacity = 1 - (1 / steps) * currentStep;
          }
        }

        newEvent.title = sanitize(newEvent.title);
        newEvent.time = calculateTimeTitle(nowTimeInMs, newEvent);
        newEvent.symbol =
          newEvent.title.indexOf('Geburtstag') === -1 ? 'fa-regular fa-calendar-check' : 'fa-solid fa-birthday-cake';
        newEvent.opacity = opacity;
      }

      calendarEvents = allNewEvents;
    });
  }

  function updateTimeInAllEvents() {
    const nowTimeInMs = Date.now();
    const calendarEventsSize = calendarEvents.length;
    for (let index = 0; index < calendarEventsSize; index++) {
      const calendarEvent = calendarEvents[index];
      calendarEvents[index].time = calculateTimeTitle(nowTimeInMs, calendarEvent);
    }
  }

  onMount(() => {
    updateCalendar();

    const updateCalendarIntervalId = setInterval(updateCalendar, config.updateIntervall);
    const updateTimeInAllEventsIntervalId = setInterval(updateTimeInAllEvents, ONE_MINUTE_IN_MS);

    return () => {
      clearInterval(updateCalendarIntervalId);
      clearInterval(updateTimeInAllEventsIntervalId);
    };
  });
</script>

<header class="normal">{header}</header>
<table class="small">
  <tbody>
    {#each calendarEvents as event}
      <tr class="normal bright" style:opacity={event.opacity}>
        <td class="symbol">
          <i class={event.symbol}></i>
        </td>
        <td class="title">{@html event.title}</td>
        <td class="time light">{event.time}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .symbol {
    font-size: 90%;
    vertical-align: top;
  }

  .title {
    display: -webkit-box;
    margin-left: 0.5rem;
    margin-right: 0.5rem;
    overflow: hidden;
    text-align: center;
    word-break: break-word;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .time {
    text-align: right;
    vertical-align: top;
    white-space: nowrap;
  }
</style>

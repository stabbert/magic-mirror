<script>
  import '../../../node_modules/font-awesome/css/font-awesome.min.css';
  import ICAL from 'ical.js';
  import moment from 'moment/min/moment-with-locales';
  import { onMount } from 'svelte';
  import { store } from '../store';

  let calendar = {
    header: '',
    events: [],
  };

  // Define second, minute, hour, and day variables
  const oneSecondInMs = 1000; // 1,000 milliseconds
  const oneMinuteInMs = oneSecondInMs * 60;
  const oneHourInMs = oneMinuteInMs * 60;
  const oneDayInMs = oneHourInMs * 24;
  const twoDayInMs = oneDayInMs * 2;

  function shorten(string, maxLength) {
    if (typeof string !== 'string' || (maxLength && typeof maxLength !== 'number')) {
      return '';
    }

    if (string.length > maxLength) {
      return string.trim().slice(0, maxLength) + '...';
    } else {
      return string.trim();
    }
  }

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

  function capFirst(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
  }

  function isFullDayEvent(event) {
    return event.duration.days > 0 ? true : false;
  }

  function calculateTimeTitle(nowTimeInMs, newEvent) {
    if (newEvent.fullDayEvent) {
      const startTimeInMsOnlyDate = startOfDayAsTimeInMs(newEvent.startTimeInMs);
      const endTimeInMsOnlyDate = startOfDayAsTimeInMs(newEvent.endTimeInMs);
      if (isSame(newEvent.startTimeInMs, startTimeInMsOnlyDate) && isSame(newEvent.endTimeInMs, endTimeInMsOnlyDate)) {
        const nowTimeInMsOnlyDate = startOfDayAsTimeInMs(nowTimeInMs);
        if (isSame(startTimeInMsOnlyDate, nowTimeInMsOnlyDate)) {
          return 'Heute';
        } else {
          const diffInMs = newEvent.startTimeInMs - nowTimeInMs;
          if (diffInMs > 0) {
            if (diffInMs < oneDayInMs) {
              return 'Morgen';
            } else if (diffInMs < twoDayInMs) {
              return 'Übermorgen';
            }
          }
        }
      }
    }

    if (isAfter(newEvent.startTimeInMs, nowTimeInMs)) {
      const diffInMs = newEvent.startTimeInMs - nowTimeInMs;
      if (diffInMs < twoDayInMs) {
        return capFirst(moment(newEvent.startTimeInMs).calendar());
      } else {
        const startTimeInMsOnlyDate = startOfDayAsTimeInMs(newEvent.startTimeInMs);
        const nowTimeInMsOnlyDate = startOfDayAsTimeInMs(nowTimeInMs);
        return capFirst(moment(startTimeInMsOnlyDate).from(nowTimeInMsOnlyDate));
      }
    } else if (isAfter(newEvent.endTimeInMs, nowTimeInMs)) {
      return 'Noch ' + moment(newEvent.endTimeInMs).from(nowTimeInMs, true);
    } else {
      return capFirst(moment(newEvent.endTimeInMs).from(nowTimeInMs));
    }
  }

  function uniq(array) {
    let result = [];
    let seen = new Set();

    outer: for (let index = 0; index < array.length; index++) {
      let value = array[index];
      if (seen.has(value)) {
        continue outer;
      }
      seen.add(value);
      result.push(value);
    }

    return result;
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

  const config = $store.config.calendar;

  calendar.header = config.header;

  moment.updateLocale($store.config.common.language, {
    longDateFormat: { LT: 'HH:mm' },
  });

  let updateTimeInAllEventsIntervalId;

  function updateCalendar() {
    if (updateTimeInAllEventsIntervalId) {
      clearInterval(updateTimeInAllEventsIntervalId);
    }

    const nowTimeInMs = Date.now();
    const todayTimeInMs = startOfDayAsTimeInMs(nowTimeInMs);
    const futureTimeInMs = addDaysAsTimeInMs(nowTimeInMs, config.maximumNumberOfDays) - 1000; // Subtract 1 second so that events that start on the middle of the night will not repeat.

    let calendarFetches = [];

    for (let calendar of config.calendars) {
      let url = calendar.url.replace('webcal://', 'https://');
      let auth = calendar.auth;

      let opts = {
        headers: {
          'Access-Control-Allow-Origin': url,
          'Content-Type': 'text/calendar; charset=UTF-8',
        },
      };

      if (auth) {
        if (auth.method === 'bearer') {
          opts.headers['Authorization'] = 'Bearer ' + auth.pass;
        } else {
          opts.headers['Authorization'] = 'Basic ' + btoa(auth.user + ':' + auth.pass);
        }
      }

      calendarFetches.push(
        fetch(url, opts)
          .then(function (response) {
            return response.text();
          })
          .then((data) => {
            let parsedICS = ICAL.parse(data);
            let comp = new ICAL.Component(parsedICS);
            let allComp = comp.getAllSubcomponents();
            let newEvents = [];
            let shiftedRecurrenceEventMap = {};

            for (let index in allComp) {
              let vevent = allComp[index];
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

            return newEvents;
          })
          .catch((error) => window.console.error(error)),
      );
    }

    Promise.all(calendarFetches).then((allCalendarNewEvents) => {
      let allNewEvents = uniq(
        allCalendarNewEvents.reduce(
          (allCalendarNewEvent, calendarNewEvent) => allCalendarNewEvent.concat(calendarNewEvent),
          [],
        ),
      );

      allNewEvents.sort(function (a, b) {
        return a.startTimeInMs - b.startTimeInMs;
      });

      let maximumAllNewEvents = allNewEvents.slice(0, config.maximumEntries);

      calendar.events = maximumAllNewEvents.map((newEvent, index) => {
        let opacity = 1;

        if (config.fade && config.fadePoint < 1) {
          if (config.fadePoint < 0) {
            config.fadePoint = 0;
          }
          let startingPoint = maximumAllNewEvents.length * config.fadePoint;
          let steps = maximumAllNewEvents.length - startingPoint;
          if (index >= startingPoint) {
            let currentStep = index - startingPoint;
            opacity = 1 - (1 / steps) * currentStep;
          }
        }

        return {
          title: sanitize(shorten(newEvent.title, config.maxTitleLength)),
          time: calculateTimeTitle(nowTimeInMs, newEvent),
          symbol: newEvent.title.indexOf('Geburtstag') === -1 ? 'fa-calendar-check-o' : 'fa-birthday-cake',
          opacity: opacity,
        };
      });

      function updateTimeInAllEvents() {
        const nowTimeInMs = Date.now();
        maximumAllNewEvents.forEach((newEvent, index) => {
          calendar.events[index].time = calculateTimeTitle(nowTimeInMs, newEvent);
        });
      }

      updateTimeInAllEventsIntervalId = setInterval(updateTimeInAllEvents, oneMinuteInMs);
    });
  }

  onMount(() => {
    updateCalendar();

    const interval = setInterval(updateCalendar, config.updateIntervall);

    return () => clearInterval(interval);
  });
</script>

<div class="calendar">
  <header>{calendar.header}</header>
  <table class="small">
    {#each calendar.events as event}
      <tr class="normal bright" style:opacity={event.opacity}>
        <td class="symbol align-right">
          <span class="fa fa-fw {event.symbol}" />
        </td>
        <td class="title">{@html event.title}</td>
        <td class="time light">{event.time}</td>
      </tr>
    {/each}
  </table>
</div>

<style>
  .calendar .symbol {
    font-size: 80%;
    vertical-align: top;
    width: 20px;
  }

  .calendar .symbol span {
    display: inline-block;
    transform: translate(0, 2px);
  }

  .calendar .title {
    word-break: break-word;
  }

  .calendar .time {
    text-align: right;
    vertical-align: top;
    width: 135px;
  }
</style>

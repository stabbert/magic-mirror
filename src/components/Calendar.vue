<template>
  <div class="calendar">
    <header>{{ header }}</header>
    <table class="small">
      <tr
        class="normal bright"
        v-bind:style="{ opacity: event.opacity }"
        v-for="event in events"
        :key="event.$index"
      >
        <td class="symbol align-right">
          <span class="fa fa-fw" v-bind:class="event.symbol"></span>
        </td>
        <td class="title">
          <span v-html="event.title"></span>
        </td>
        <td class="time light">{{ event.time }}</td>
      </tr>
    </table>
  </div>
</template>

<script>
import ICAL from "ical.js";
import moment from "moment";

// Define second, minute, hour, and day variables
const oneSecond = 1000; // 1,000 milliseconds
const oneMinute = oneSecond * 60;
const oneHour = oneMinute * 60;
const oneDay = oneHour * 24;
const twoDay = oneDay * 2;

function shorten(string, maxLength, wrapEvents) {
  if (typeof string !== "string") {
    return "";
  }

  if (wrapEvents === true) {
    let temp = "";
    let currentLine = "";
    let words = string.split(" ");

    for (let i = 0; i < words.length; i++) {
      let word = words[i];
      if (
        currentLine.length + word.length <
        (typeof maxLength === "number" ? maxLength : 25) - 1
      ) {
        // max - 1 to account for a space
        currentLine += word + " ";
      } else {
        if (currentLine.length > 0) {
          temp += currentLine + "<br>" + word + " ";
        } else {
          temp += word + "<br>";
        }
        currentLine = "";
      }
    }

    return (temp + currentLine).trim();
  } else {
    if (
      maxLength &&
      typeof maxLength === "number" &&
      string.length > maxLength
    ) {
      return string.trim().slice(0, maxLength) + "&hellip;";
    } else {
      return string.trim();
    }
  }
}

function capFirst(string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

function isFullDayEvent(event) {
  return event.duration.days > 0 ? true : false;
}

function calculateTimeTitle(now, newEvent) {
  if (newEvent.fullDayEvent) {
    const startOnlyDate = newEvent.startDate.clone().startOf("date");
    const endOnlyDate = newEvent.endDate.clone().startOf("date");
    if (
      newEvent.startDate.isSame(startOnlyDate) &&
      newEvent.endDate.isSame(endOnlyDate)
    ) {
      const nowOnlyDate = now.clone().startOf("date");
      if (startOnlyDate.isSame(nowOnlyDate)) {
        return "Heute";
      } else {
        const diff = newEvent.startDate.diff(now);
        if (diff > 0) {
          if (diff < oneDay) {
            return "Morgen";
          } else if (diff < twoDay) {
            return "Übermorgen";
          }
        }
      }
    }
  }

  if (newEvent.startDate.isAfter(now)) {
    if (newEvent.startDate.diff(now) < twoDay) {
      return capFirst(newEvent.startDate.calendar());
    } else {
      const startOnlyDate = newEvent.startDate.clone().startOf("date");
      const nowOnlyDate = now.clone().startOf("date");
      return capFirst(startOnlyDate.from(nowOnlyDate));
    }
  } else if (newEvent.endDate.isAfter(now)) {
    return "Noch " + newEvent.endDate.from(now, true);
  } else {
    return capFirst(newEvent.endDate.from(now));
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

export default {
  name: "Calendar",
  data() {
    return {
      header: "",
      events: []
    };
  },
  created() {
    const config = this.$store.state.config.calendar;
    let self = this;
    self.header = config.header;

    moment.updateLocale(this.$store.state.config.common.language, {
      longDateFormat: { LT: "HH:mm" }
    });

    let updateTimeInAllEventsIntervalId;

    function updateCalendar() {
      if (updateTimeInAllEventsIntervalId) {
        clearInterval(updateTimeInAllEventsIntervalId);
      }

      const now = moment();
      const today = now.clone().startOf("date");
      const future = today
        .clone()
        .add(config.maximumNumberOfDays, "days")
        .subtract(1, "seconds"); // Subtract 1 second so that events that start on the middle of the night will not repeat.

      let calendarFetches = [];

      for (let calendar of config.calendars) {
        let url = calendar.url.replace("webcal://", "https://");

        let opts = {
          headers: {
            "Access-Control-Allow-Origin": url,
            "Content-Type": "text/calendar; charset=UTF-8"
          }
        };

        if (calendar.auth) {
          opts.auth = {
            user: calendar.auth.user,
            pass: calendar.auth.pass,
            sendImmediately: true
          };
        }

        calendarFetches.push(
          fetch(url, opts)
            .then(function(response) {
              return response.text();
            })
            .then(data => {
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

                let startDate = moment(event.startDate.toJSDate());
                let endDate;

                if (typeof event.endDate !== "undefined") {
                  endDate = moment(event.endDate.toJSDate());
                } else {
                  endDate = startDate;
                }

                let title = "Termin";
                if (event.summary) {
                  title =
                    typeof event.summary.val !== "undefined"
                      ? event.summary.val
                      : event.summary;
                } else if (event.description) {
                  title = event.description;
                }

                // Replaying event
                if (event.isRecurring()) {
                  let dates = [];
                  let rule = event.iterator();
                  let shiftedRecurrenceEvent =
                    shiftedRecurrenceEventMap[event.uid];

                  let shiftedMoment = shiftedRecurrenceEvent
                    ? moment(shiftedRecurrenceEvent.recurrenceId.toJSDate())
                    : null;

                  let nextDate = rule.next();
                  while (nextDate) {
                    let nextMoment = moment(nextDate.toJSDate());
                    if (shiftedMoment && shiftedMoment.isSame(nextMoment)) {
                      nextDate = rule.next();
                      continue;
                    }
                    if (
                      nextMoment.isBetween(today, future) ||
                      nextMoment.isSame(today)
                    ) {
                      dates.push(nextMoment);
                    }
                    if (
                      dates.length <= config.maximumEntries &&
                      nextMoment.isBefore(future)
                    ) {
                      nextDate = rule.next();
                    } else {
                      nextDate = false;
                    }
                  }

                  // calculate the duration of the event for use with recurring events.
                  let duration = endDate.diff(startDate);

                  for (let d in dates) {
                    startDate = dates[d];
                    endDate = startDate.clone().add(duration);

                    if (endDate.isAfter(now)) {
                      newEvents.push({
                        title: title,
                        startDate: startDate,
                        endDate: endDate,
                        fullDayEvent: isFullDayEvent(event)
                      });
                    }
                  }
                } else {
                  if (endDate.isSameOrBefore(now)) {
                    continue;
                  }

                  if (startDate.isAfter(future)) {
                    continue;
                  }

                  // Every thing is good. Add it to the list.
                  newEvents.push({
                    title: title,
                    startDate: startDate,
                    endDate: endDate,
                    fullDayEvent: isFullDayEvent(event)
                  });
                }
              }

              return newEvents;
            })
            .catch(error => window.console.error(error))
        );
      }

      Promise.all(calendarFetches).then(allCalendarNewEvents => {
        let allNewEvents = uniq(
          allCalendarNewEvents.reduce(
            (allCalendarNewEvent, calendarNewEvent) =>
              allCalendarNewEvent.concat(calendarNewEvent),
            []
          )
        );

        allNewEvents.sort(function(a, b) {
          return a.startDate.valueOf() - b.startDate.valueOf();
        });

        let maximumAllNewEvents = allNewEvents.slice(0, config.maximumEntries);

        self.events = maximumAllNewEvents.map((newEvent, index) => {
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
            title: shorten(
              newEvent.title,
              config.maxTitleLength,
              config.wrapEvents
            ),
            time: calculateTimeTitle(now, newEvent),
            symbol:
              newEvent.title.indexOf("Geburtstag") === -1
                ? "fa-calendar-check-o"
                : "fa-birthday-cake",
            opacity: opacity
          };
        });

        function updateTimeInAllEvents() {
          const now = moment();
          maximumAllNewEvents.forEach((newEvent, index) => {
            self.events[index].time = calculateTimeTitle(now, newEvent);
          });
        }

        updateTimeInAllEventsIntervalId = setInterval(
          updateTimeInAllEvents,
          oneMinute
        );
      });
    }

    updateCalendar();
    setInterval(updateCalendar, config.updateIntervall);
  }
};
</script>

<style scoped>
@import "../../node_modules/font-awesome/css/font-awesome.min.css";

.calendar .symbol {
  padding-left: 0;
  padding-right: 10px;
  font-size: 80%;
  vertical-align: top;
  width: 20px;
}

.calendar .symbol span {
  display: inline-block;
  transform: translate(0, 2px);
}

.calendar .title {
  padding-left: 0;
  padding-right: 0;
}

.calendar .time {
  padding-left: 10px;
  text-align: right;
  vertical-align: top;
}
</style>

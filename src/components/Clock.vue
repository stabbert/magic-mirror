<template>
  <div>
    <div class="normal medium">{{ date }}</div>
    <div class="bright large light">
      {{ hours }}:{{ minutes }}<sup class="dimmed">{{ seconds }}</sup>
    </div>
  </div>
</template>

<script>
import moment from "moment";

export default {
  name: "Clock",
  data: function() {
    return {
      date: "",
      hours: "",
      minutes: "",
      seconds: ""
    };
  },
  created: function() {
    let self = this;
    setInterval(function() {
      let now = moment().locale(self.$store.state.config.common.language);
      self.date = now.format("dddd, l");
      self.hours = timeNumber(now.hours());
      self.minutes = timeNumber(now.minutes());
      self.seconds = timeNumber(now.seconds());

      function timeNumber(number) {
        return number < 10 ? "0" + number : number;
      }
    }, 1000);
  }
};
</script>

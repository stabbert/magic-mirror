<script>
  import moment from 'moment/min/moment-with-locales';
  import { onMount } from 'svelte';
  import { store } from '../store';

  let clock = {
    date: '',
    hours: '',
    minutes: '',
    seconds: '',
  };

  const language = $store.config.common.language;

  function timeNumber(number) {
    return number < 10 ? '0' + number : number;
  }

  function updateClock() {
    let now = moment().locale(language);
    clock.date = now.format('dddd, l');
    clock.hours = timeNumber(now.hours());
    clock.minutes = timeNumber(now.minutes());
    clock.seconds = timeNumber(now.seconds());
  }

  onMount(() => {
    updateClock();

    const interval = setInterval(updateClock, 1000);

    return () => clearInterval(interval);
  });
</script>

<div class="normal medium">{clock.date}</div>
<div class="bright large light">
  {clock.hours}:{clock.minutes}
  <sup class="dimmed">{clock.seconds}</sup>
</div>

<script>
  import { onMount } from 'svelte';
  import { store } from '../store';

  const clock = $state({
    date: '',
    time: '',
    seconds: '',
  });

  const language = $store.config.common.language;

  const ONE = 1;
  const TEN = 10;
  const FIFTY_NINE = 59;
  const ZERO_STRING = '0';

  const DATE_FORMAT = new Intl.DateTimeFormat(language, {
    weekday: 'long',
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
  });

  const TIME_FORMAT = new Intl.DateTimeFormat(language, {
    hour: '2-digit',
    minute: '2-digit',
  });

  function timeNumber(number) {
    return number < TEN ? ZERO_STRING + number : number;
  }

  let lastHour = -1;
  let secondCounter = 59;

  function updateClock() {
    if (secondCounter === FIFTY_NINE) {
      const now = new Date();
      const currentHour = now.getHours();

      if (lastHour !== currentHour) {
        clock.date = DATE_FORMAT.format(now);
        lastHour = currentHour;
      }

      clock.time = TIME_FORMAT.format(now);
      secondCounter = now.getSeconds();
    } else {
      secondCounter = secondCounter + ONE;
    }
    clock.seconds = timeNumber(secondCounter);
  }

  onMount(() => {
    updateClock();

    const intervalId = setInterval(updateClock, 1000);

    return () => clearInterval(intervalId);
  });
</script>

<div class="normal medium">{clock.date}</div>
<div class="bright large light">
  {clock.time}
  <sup class="dimmed">{clock.seconds}</sup>
</div>

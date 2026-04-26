<script>
  import { onMount } from 'svelte';
  import { store } from '../store';

  let date = $state('');
  let hours = $state('');
  let minutes = $state('');
  let seconds = $state('');

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

  function timeNumber(number) {
    return number < TEN ? ZERO_STRING + number.toString() : number.toString();
  }

  let lastHour = -1;
  let secondCounter = 59;

  function updateClock() {
    if (secondCounter === FIFTY_NINE) {
      const now = new Date();
      const currentHour = now.getHours();

      if (lastHour !== currentHour) {
        date = DATE_FORMAT.format(now);
        lastHour = currentHour;
      }

      hours = timeNumber(currentHour);
      minutes = timeNumber(now.getMinutes());
      secondCounter = now.getSeconds();
    } else {
      secondCounter = secondCounter + ONE;
    }
    seconds = timeNumber(secondCounter);
  }

  onMount(() => {
    updateClock();

    const intervalId = setInterval(updateClock, 1000);

    return () => clearInterval(intervalId);
  });
</script>

<div class="normal medium">{date}</div>
<div class="bright large light"><span>{hours}</span>:<span>{minutes}</span><sup class="dimmed">{seconds}</sup></div>

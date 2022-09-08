<script lang="ts">
  import {onMount, onDestroy} from 'svelte'
  import Clock from './Clock.svelte'

  let widthPadding = 0

  let interval = null;

  onMount(async () => {
    interval = setInterval(() => {
      widthPadding = Math.random() * 5 + 0.5
    }, import.meta.env.VITE_CLOCK_PADDING_INTERVAL * 1000)
  })

  onDestroy(() => {
    if (interval) {
      clearInterval(interval)
    }
  })

  $: clockStyle = () => {
    return `width: calc(100vw - ${widthPadding}rem);`
  }
</script>

<div class="clock">
  <div style={clockStyle()}>
    <Clock color="#e4bd6" />
  </div>
</div>

<style>
  .clock {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>

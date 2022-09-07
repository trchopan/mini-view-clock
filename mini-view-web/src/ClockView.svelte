<script lang="ts">
  import {onMount} from 'svelte'
  import {sample} from 'lodash'
  import Clock from './Clock.svelte'

  let widthPadding = 0

  const colors = [
    //
    '#f4b78d',
    '#85a28b',
    '#e4bd6',
    '#d4d4bd',
    '#d6d4ff',
  ]
  let clockColor = sample(colors)

  onMount(async () => {
    setInterval(() => {
      widthPadding = Math.random() * 5 + 0.5
      clockColor = sample(colors)
    }, import.meta.env.VITE_CLOCK_PADDING_INTERVAL * 1000)
  })

  $: clockStyle = () => {
    return `width: calc(100vw - ${widthPadding}rem);`
  }
</script>

<div class="clock">
  <div style={clockStyle()}>
    <Clock color={clockColor} />
  </div>
</div>

<style>
  .clock {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>

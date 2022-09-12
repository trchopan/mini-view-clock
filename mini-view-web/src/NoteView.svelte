<script lang="ts">
  import Clock from './Clock.svelte'
  import axios from 'axios'
  import {onDestroy, onMount} from 'svelte'

  let text = ''
  let isLoading = false
  const getNote = async () => {
    isLoading = true
    try {
      const server = import.meta.env.VITE_SERVER
      const {data} = await axios.get(server + '/note-or-inspire')
      text = data.content
    } catch (err) {
      console.error('Error getting daily note', err)
    } finally {
      isLoading = false
    }
  }

  let isSmallClock = false
  const toggleSmallClock = () => {
    isSmallClock = !isSmallClock
  }

  let interval = []
  onMount(async () => {
    getNote()
    interval = [
      {
        fn: () => {
          getNote()
        },
        interval: import.meta.env.VITE_NOTE_REFRESH_INTERVAL * 1000,
      },
    ].map(i => setInterval(i.fn, i.interval))
  })

  onDestroy(() => {
    interval.forEach(i => clearInterval(i))
  })

  onMount(() => {})
</script>

<div class="container">
  <div
    class="clock"
    style:background-color={isLoading ? 'var(--background)' : 'var(--darker)'}
    style:width={isSmallClock ? '12rem' : '18rem'}
  >
    <Clock />
  </div>
  <div class="text">{@html text}</div>
  <div class="buttons">
    <button on:click={() => getNote()}>Load</button>
    <button on:click={() => toggleSmallClock()}>
      {isSmallClock ? 'Big' : 'Small'}
    </button>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 3.5fr 1fr;
  }
  .clock {
    position: fixed;
    top: 0.5rem;
    right: 0.5rem;
  }
  .text {
    white-space: pre;
    white-space: break-spaces;
    font-size: 1.2rem;
    padding: 5rem 1rem;
    overflow-y: scroll;
  }
  .buttons {
    position: fixed;
    bottom: 0.5rem;
    right: 0.5rem;
    display: grid;
    grid-template-columns: 1fr;
    grid-gap: 1rem;
  }
</style>

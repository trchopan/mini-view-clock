<script lang="ts">
  import axios from 'axios'

  import {onDestroy, onMount} from 'svelte'
  import Calendar from './Calendar.svelte'
  import Clock from './Clock.svelte'
  import {fmtNumber, fmtPercent} from './helpers'

  let coinInfo: {
    id: string
    name: string
    usd: number
    usd_24h_change: number
  }[] = []

  const coins = [
    {id: 'bitcoin', name: 'BTC'},
    {id: 'ethereum', name: 'ETH'},
    {id: 'matic-network', name: 'MATIC'},
    {id: 'cardano', name: 'ADA'},
    {id: 'coti', name: 'COTI'},
    {id: 'xrp', name: 'XRP'},
    {id: 'binancecoin', name: 'BNB'},
    {id: 'polkadot', name: 'DOT'},
    {id: 'near', name: 'NEAR'},
    {id: 'solana', name: 'SOL'},
    {id: 'singularitynet', name: 'AGIX'},
    {id: 'avalanche-2', name: 'AVAX'},
  ]

  const fetchCoins = async () => {
    try {
      const ids = coins.map(c => c.id).join(',')
      const {data} = await axios.get<
        {[key: string]: {usd: number; usd_24h_change: number}}[]
      >(
        `https://api.coingecko.com/api/v3/simple/price?ids=${ids}&vs_currencies=usd&include_24hr_change=true`
      )
      return data
    } catch (err) {
      console.error('>>> err getting coin', err)
      return null
    }
  }

  const getCoins = async () => {
    let data = (() => {
      try {
        const cachedCoins = JSON.parse(localStorage.getItem('cachedCoins'))
        if (cachedCoins.expires < new Date().getTime()) {
          return cachedCoins.data
        } else {
          return null
        }
      } catch (err) {
        return null
      }
    })()
    if (data == null) {
      data = await fetchCoins()
      const cachedCoins = {
        data,
        expires: new Date().getTime() + 60 * 60 * 1000,
      }
      localStorage.setItem('cachedCoins', JSON.stringify(cachedCoins))
    }
    coinInfo = coins.map(coin => ({...coin, ...data[coin.id]}))
  }

  const timers = [
    {text: 'Off', durationMin: -1},
    {text: '5 min', durationMin: 5},
    {text: '10 min', durationMin: 10},
    {text: '30 min', durationMin: 30},
    {text: '60 min', durationMin: 60},
  ]

  let currentTimer = timers[0]
  let currentTimerMark = new Date()
  let timerActive = false
  let diffSeconds = -1
  $: timerDiffStr = () => {
    const mins = Math.floor(diffSeconds / 60)
    const secs = Math.floor(diffSeconds % 60)
    const minsStr = mins ? mins + ' m ' : ''
    const secsStr = secs ? secs + ' s' : ''
    return minsStr + secsStr
  }

  const toggleTimmer = () => {
    currentTimerMark = new Date()
    const currentIndex = timers.indexOf(currentTimer)
    currentTimer =
      currentIndex >= timers.length - 1 ? timers[0] : timers[currentIndex + 1]
  }

  const timerTick = () => {
    if (currentTimer.text === 'Off') {
      timerActive = false
      return
    }
    diffSeconds = (new Date().getTime() - currentTimerMark.getTime()) / 1000
    const durationMin = currentTimer.durationMin
    timerActive = diffSeconds > durationMin * 60
    if (diffSeconds >= (durationMin + 1) * 60) {
      // Set timer to off after 1 minutes alert
      currentTimer = timers[0]
    }
  }

  const resetTimerMark = () => {
    currentTimerMark = new Date()
    timerActive = false
  }

  let interval = []

  onMount(async () => {
    getCoins()
    interval = [
      {
        fn: getCoins,
        interval: import.meta.env.VITE_COIN_REFRESH_INTERVAL * 1000,
      },
      {
        fn: timerTick,
        interval: 1000,
      },
    ].map(i => setInterval(i.fn, i.interval))
  })

  onDestroy(() => {
    interval.forEach(i => clearInterval(i))
  })
</script>

<div class="clock-container">
  <div class="clock">
    <Clock />
    <Calendar />
  </div>
  <div class="coins">
    {#each coinInfo as coin}
      <div class="coin">
        <span>{coin.name}:</span>
        <span>{fmtNumber(coin.usd, 4)}</span>
        <span style:color={coin.usd_24h_change > 0 ? '#78d578' : '#ff3636'}>
          {fmtPercent(coin.usd_24h_change)}
        </span>
      </div>
    {/each}
  </div>
  <div class="buttons">
    <button on:click={() => getCoins()}>Coins</button>
    {#if timerActive}
      <button class="flash-timer" on:click={() => resetTimerMark()}>
        Reset
      </button>
    {:else}
      <button on:click={() => toggleTimmer()}>
        {currentTimer.text}
        {#if currentTimer.text !== 'Off'}
          <span class="diff-seconds">
            {timerDiffStr()}
          </span>
        {/if}
      </button>
    {/if}
  </div>
</div>

<style>
  .clock-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    align-items: center;
    justify-content: center;
    padding: 0 0.5rem;
    overflow: hidden;
  }
  .clock {
    flex: 1;
    height: 100%;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }
  .coins {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-row-gap: 0rem;
    grid-column-gap: 0rem;
    padding-bottom: 0.5rem;
  }
  .coin {
    font-size: 0.82rem;
    border: solid 1px #303030;
    padding: 0.2rem;
  }
  .coin > span {
    margin-right: 0.3rem;
  }
  .buttons {
    position: fixed;
    top: 0.5rem;
    right: 0.5rem;
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1rem;
  }
  @keyframes flash-timer-animate {
    0% {
      color: white;
      background-color: red;
    }
    50% {
      color: black;
      background-color: white;
    }
    100% {
      color: white;
      background-color: red;
    }
  }
  .flash-timer {
    animation: flash-timer-animate 500ms infinite;
    position: relative;
  }
  .diff-seconds {
    color: #686868;
    position: absolute;
    bottom: -0.3rem;
    right: 0.5rem;
    font-size: 0.6rem;
  }
</style>

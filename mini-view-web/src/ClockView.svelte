<script lang="ts">
  import axios from 'axios'

  import {onMount, onDestroy} from 'svelte'
  import Clock from './Clock.svelte'
  import {fmtNumber, fmtPercent} from './helpers'

  let widthPadding = 0

  let showCoins = true
  let coinInfo: {
    id: string
    name: string
    usd: number
    usd_24h_change: number
  }[] = []

  const getCoins = async () => {
    if (!showCoins) return

    let coins = [
      {id: 'bitcoin', name: 'BTC'},
      {id: 'ethereum', name: 'ETH'},
      {id: 'matic-network', name: 'MATIC'},
      {id: 'cardano', name: 'ADA'},
      {id: 'coti', name: 'COTI'},
      {id: 'binancecoin', name: 'BNB'},
      {id: 'polkadot', name: 'DOT'},
      {id: 'harmony', name: 'ONE'},
      {id: 'near', name: 'NEAR'},
      {id: 'solana', name: 'SOL'},
    ]

    const promises = async () => {
      try {
        const ids = coins.map(c => c.id).join(',')
        return await axios.get<
          {[key: string]: {usd: number; usd_24h_change: number}}[]
        >(
          `https://api.coingecko.com/api/v3/simple/price?ids=${ids}&vs_currencies=usd&include_24hr_change=true`
        )
      } catch (err) {
        console.error('>>> err getting coin', err)
        return null
      }
    }
    const {data} = await promises()
    coinInfo = coins.map(coin => ({...coin, ...data[coin.id]}))
  }

  const toggleShowCoins = () => {
    showCoins = !showCoins
    if (showCoins) {
      getCoins()
    }
  }

  let interval = []
  onMount(async () => {
    getCoins()
    interval = [
      {
        fn: () => {
          widthPadding = Math.random() * 5 + 0.5
        },
        interval: import.meta.env.VITE_CLOCK_PADDING_INTERVAL * 1000,
      },
      {
        fn: () => {
          getCoins()
        },
        interval: import.meta.env.VITE_COIN_REFRESH_INTERVAL * 1000,
      },
    ].map(i => setInterval(i.fn, i.interval))
  })

  onDestroy(() => {
    interval.forEach(i => clearInterval(i))
  })

  $: clockStyle = () => {
    return `width: calc(100vw - ${widthPadding}rem);`
  }
</script>

<div class="clock-container">
  <div style={clockStyle()}>
    <Clock />
  </div>
  {#if showCoins}
    <div class="coins">
      {#each coinInfo as coin}
        <div class="coin">
          <span>{coin.name}:</span>
          <span>{fmtNumber(coin.usd)}</span>
          <span
            style:color={coin.usd_24h_change > 0
              ? 'var(--secondary)'
              : 'var(--primary)'}
          >
            {fmtPercent(coin.usd_24h_change)}
          </span>
        </div>
      {/each}
    </div>
  {/if}
  <div class="buttons">
    <button on:click={() => toggleShowCoins()}>Coins</button>
  </div>
</div>

<style>
  .clock-container {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .coins {
    position: fixed;
    left: 0.5rem;
    bottom: 0.5rem;
    right: 0.5rem;
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    grid-row-gap: 0.3rem;
    grid-column-gap: 0.5rem;
  }
  .coin {
    font-size: 0.68rem;
  }
  .coin > span {
    margin-right: 0.3rem;
  }
  .buttons {
    position: fixed;
    top: 0.5rem;
    right: 0.5rem;
    display: grid;
    grid-template-columns: 1fr;
    grid-gap: 1rem;
  }
</style>

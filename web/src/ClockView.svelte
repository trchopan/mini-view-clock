<script lang="ts">
  import axios from 'axios'

  import {onMount, onDestroy} from 'svelte'
  import Clock from './Clock.svelte'
  import {fmtNumber, fmtPercent} from './helpers'

  let coinInfo: {
    id: string
    name: string
    usd: number
    usd_24h_change: number
  }[] = []

  const getCoins = async () => {
    let coins = [
      {id: 'bitcoin', name: 'BTC'},
      {id: 'ethereum', name: 'ETH'},
      {id: 'matic-network', name: 'MATIC'},
      {id: 'cardano', name: 'ADA'},
      {id: 'coti', name: 'COTI'},
      {id: 'binancecoin', name: 'BNB'},
      {id: 'polkadot', name: 'DOT'},
      {id: 'near', name: 'NEAR'},
      {id: 'solana', name: 'SOL'},
      {id: 'singularitynet', name: 'AGIX'},
      {id: 'avalanche-2', name: 'AVAX'},
      {id: 'harmony', name: 'ONE'},
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

  let interval = []
  onMount(async () => {
    getCoins()
    interval = [
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
</script>

<div class="clock-container">
  <div class="clock">
      <Clock />
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
    grid-template-columns: 1fr;
    grid-gap: 1rem;
  }
</style>

<script lang="ts">
    import axios from 'axios'

    import {onDestroy, onMount} from 'svelte'
    import Calendar from './Calendar.svelte'
    import Clock from './Clock.svelte'
    import CoinChart from './CoinChart.svelte' // Import the new component
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
        {id: 'cardano', name: 'ADA'},
        {id: 'binancecoin', name: 'BNB'},
        {id: 'avalanche-2', name: 'AVAX'},
        {id: 'matic-network', name: 'MATIC'},
        {id: 'solana', name: 'SOL'},
        {id: 'coti', name: 'COTI'},
        {id: 'polkadot', name: 'DOT'},
        {id: 'near', name: 'NEAR'},
        {id: 'singularitynet', name: 'AGIX'},
        {id: 'tether-gold', name: 'GOLD'},
    ]

    let bitcoinChartData: {prices: [number, number][]} | null = null

    // State for rotating coin chart
    let currentChartIdx = 0
    let currentChartData: {prices: [number, number][]} | null = null
    let currentChartName: string = coins[0].name
    let timeframeDays: number = 365

    function setTimeframe(days: number) {
        timeframeDays = days
        updateCurrentChart()
    }

    async function updateCurrentChart() {
        const coin = coins[currentChartIdx]
        currentChartName = coin.name
        const data = await getChart(coin.id, timeframeDays)
        if (data) {
            currentChartData = data
        } else {
            currentChartData = null
        }
    }

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
                if (cachedCoins.expire > new Date().getTime()) {
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
                expire: new Date().getTime() + 5 * 60 * 1000,
            }
            localStorage.setItem('cachedCoins', JSON.stringify(cachedCoins))
        }
        coinInfo = coins.map(coin => ({...coin, ...data[coin.id]}))
    }

    const getChart = async (coinId: string, days: number) => {
        const cacheKey = `${coinId}-${days}`
        try {
            let cachedCharts = {}
            try {
                cachedCharts = JSON.parse(localStorage.getItem('cachedChart')) || {}
            } catch (e) {
                cachedCharts = {}
            }
            const cacheEntry = cachedCharts[cacheKey]
            const now = new Date().getTime()
            if (cacheEntry && cacheEntry.expire > now) {
                return cacheEntry.data
            }
            // Fetch OHLC data and map to [timestamp, close] format
            const {data} = await axios.get<any[]>(
                `https://api.coingecko.com/api/v3/coins/${coinId}/ohlc?vs_currency=usd&days=${days}`
            )
            // CoinGecko OHLC returns [time, open, high, low, close]
            const prices = data.map((entry: any[]) => [entry[0], entry[4]] as [number, number])
            const transformed = {prices}
            cachedCharts[cacheKey] = {
                data: transformed,
                expire: now + 5 * 60 * 1000,
            }
            localStorage.setItem('cachedChart', JSON.stringify(cachedCharts))
            return transformed
        } catch (err) {
            console.error(`>>> err getting chart for ${coinId}`, err)
            return null
        }
    }

    const getBitcoinChart = async () => {
        const data = await getChart('bitcoin', 365)
        if (data) {
            bitcoinChartData = data
        }
    }

    let intervals = []

    onMount(async () => {
        getCoins()
        getBitcoinChart()
        await updateCurrentChart()
        intervals = [
            setInterval(getCoins, import.meta.env.VITE_COIN_REFRESH_INTERVAL * 1000),
            setInterval(async () => {
                currentChartIdx = (currentChartIdx + 1) % coins.length
                await updateCurrentChart()
            }, 60 * 1000),
        ]
    })

    onDestroy(() => {
        intervals.forEach(i => clearInterval(i))
    })
</script>

<div class="clock-container">
    <div class="clock">
        <Clock />
        <Calendar />
    </div>
    <div class="coins-and-chart">
        <div class="coins-container">
            {#each coinInfo as coin, i}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <button
                    class="coin"
                    on:click={() => {
                        currentChartIdx = i
                        updateCurrentChart()
                    }}
                    style="cursor: pointer;"
                    class:selected={currentChartIdx === i}
                >
                    <span>{coin.name}:</span>
                    <span>{fmtNumber(coin.usd, 4)}</span>
                    <span style:color={coin.usd_24h_change > 0 ? '#78d578' : '#ff3636'}>
                        {fmtPercent(coin.usd_24h_change)}
                    </span>
                </button>
            {/each}
        </div>
        <div class="coinchart-container">
            {#if currentChartData?.prices}
                <CoinChart prices={currentChartData.prices} coinName={currentChartName} />
            {/if}
            <!-- Timeframe buttons moved inside the coinchart-container to appear under the chart -->
            <div
                class="timeframe-buttons"
                style="display:flex; gap:.5rem; padding: .5rem 0; align-items: center;"
            >
                <button on:click={() => setTimeframe(90)} class:selected={timeframeDays === 90}
                    >3M</button
                >
                <button on:click={() => setTimeframe(180)} class:selected={timeframeDays === 180}
                    >6M</button
                >
                <button on:click={() => setTimeframe(365)} class:selected={timeframeDays === 365}
                    >1Y</button
                >
            </div>
        </div>
    </div>
</div>

<style>
    @media (max-width: 600px) {
        .clock-container {
            padding: 0 0.2rem;
            min-width: 0;
        }
        .clock {
            display: flex;
            flex-direction: column;
            grid-template-columns: unset;
            gap: 0.5rem;
            width: 100%;
        }
        .coins-and-chart {
            flex-direction: column;
            gap: 0.3rem;
            margin-bottom: 0.3rem;
            width: 100%;
            align-items: stretch;
        }
        .coins-container {
            grid-template-columns: repeat(2, 1fr);
            padding-bottom: 0.3rem;
            font-size: 0.74rem;
            gap: 0.2rem;
        }
        .coinchart-container {
            margin-left: 0;
            padding: 0.3rem 0.2rem;
            max-width: 100vw;
            width: 100%;
            min-height: 120px;
            box-sizing: border-box;
        }
        .coin {
            font-size: 0.69rem;
            padding: 0.15rem;
        }
    }
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
    .coins-and-chart {
        display: flex;
        flex-direction: row;
        align-items: flex-start;
        width: 100%;
        gap: 1rem;
        margin-bottom: 0.5rem;
        /* Ensure children stretch equally in height */
        align-items: stretch;
    }

    .coins-container {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        grid-row-gap: 0rem;
        grid-column-gap: 0rem;
        padding-bottom: 0.5rem;
        flex: 2;
        /* Match height with chart */
        height: 100%;
    }

    .coinchart-container {
        flex: 3;
        max-width: 525px;
        display: flex;
        flex-direction: column;
        align-items: center;
        box-sizing: border-box;
        padding: 0.5rem 0.75rem;
        margin-left: 0.5rem;
        /* Make chart fill available height */
        height: 100%;
        min-height: 100%;
    }
    .coin {
        font-size: 0.82rem;
        border: solid 1px #303030;
        padding: 0.2rem;
    }
    .coin.selected {
        background: rgba(255, 255, 255, 0.08);
        border-color: #6ba7ff;
    }
    .coin > span {
        margin-right: 0.3rem;
    }
    .timeframe-buttons {
        display: flex;
        gap: 0.5rem;
        padding: 0.25rem 0;
    }
    .timeframe-buttons button {
        background: transparent;
        border: 1px solid #303030;
        color: inherit;
        padding: 0.25rem 0.6rem;
        border-radius: 4px;
        font-size: 0.8rem;
        cursor: pointer;
    }
    .timeframe-buttons button.selected {
        background: rgba(255, 255, 255, 0.08);
        border-color: #6ba7ff;
    }
</style>

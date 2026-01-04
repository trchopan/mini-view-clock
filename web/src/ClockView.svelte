<script lang="ts">
    import axios from 'axios'
    import {onDestroy, onMount} from 'svelte'
    import Calendar from './Calendar.svelte'
    import Clock from './Clock.svelte'
    import Pomodoro from './Pomodoro.svelte'
    import CoinChart from './CoinChart.svelte'
    import CoinGrid from './CoinGrid.svelte'
    import {SessionColors, SessionType} from './types'

    // Toggle Clock/Pomodoro in the Clock quadrant
    let showPomodoro = false

    let pomoSessionType: SessionType | null = null

    $: clockTint = (pomoSessionType && SessionColors[pomoSessionType]) || 'white'

    function onPomodoroChangeSession(e: CustomEvent<{sessionType: SessionType | null}>) {
		console.log('Pomodoro changeSession', e.detail)
        pomoSessionType = e.detail.sessionType
    }

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
        {id: 'solana', name: 'SOL'},
        {id: 'coti', name: 'COTI'},
        {id: 'polkadot', name: 'DOT'},
        {id: 'near', name: 'NEAR'},
        {id: 'ripple', name: 'XRP'},
        {id: 'tether-gold', name: 'GOLD'},
        {id: 'sp500-xstock', name: 'SP500'},
        {id: 'tesla-xstock', name: 'TSLA'},
        {id: 'apple-xstock', name: 'AAPL'},
        {id: 'amazon-xstock', name: 'AMZN'},
        {id: 'alphabet-xstock', name: 'GOOGL'},
        {id: 'nvidia-xstock', name: 'NVDA'},
        {id: 'meta-platforms-xstock', name: 'META'},
        {id: 'netflix-xstock', name: 'NFLX'},
        {id: 'mastercard-xstock', name: 'MA'},
    ]

    // State for rotating coin chart
    let currentChartIdx = 0
    let currentChartData: {prices: [number, number][]} | null = null
    let currentChartName: string = coins[0].name
    let timeframeDays: number = 365
    let getCoinsInterval: any = null
    let rotationInterval: any = null

    function setTimeframe(days: number) {
        timeframeDays = days
        updateCurrentChart()
    }

    function startRotation() {
        rotationInterval = setInterval(async () => {
            const nextIdx = (currentChartIdx + 1) % coins.length
            await updateCurrentChart(nextIdx)
        }, 60 * 1000)
    }

    function resetRotation() {
        if (rotationInterval) {
            clearInterval(rotationInterval)
        }
        startRotation()
    }

    async function updateCurrentChart(idx?: number) {
        if (idx !== undefined) {
            currentChartIdx = idx
            resetRotation()
        }
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
            console.log('👉 name', ids)
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
                const cachedCoinsRaw = localStorage.getItem('cachedCoins')
                if (!cachedCoinsRaw) return null
                const cachedCoins = JSON.parse(cachedCoinsRaw)
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
        coinInfo = coins
            .map(coin => ({...coin, ...data[coin.id]}))
            .filter(coin => {
                if (Boolean(coin.usd)) {
                    return true
                } else {
                    console.error('Missing data for coin', coin)
                    return false
                }
            })
    }

    const getChart = async (coinId: string, days: number) => {
        const cacheKey = `${coinId}-${days}`
        try {
            let cachedCharts: {[key: string]: any} = {}
            try {
                const cachedChartRaw = localStorage.getItem('cachedChart')
                if (cachedChartRaw) {
                    cachedCharts = JSON.parse(cachedChartRaw) || {}
                }
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

    onMount(async () => {
        getCoins()
        await updateCurrentChart()
        startRotation()
        getCoinsInterval = setInterval(getCoins, import.meta.env.VITE_COIN_REFRESH_INTERVAL * 1000)
    })

    onDestroy(() => {
        if (rotationInterval) clearInterval(rotationInterval)
        if (getCoinsInterval) clearInterval(getCoinsInterval)
    })
</script>

<div class="h-screen text-white">
    <div class="grid grid-cols-2 grid-rows-2 h-full">
        <!-- Clock quadrant with Clock/Pomodoro toggle -->
        <div class="col-span-1 row-span-1 p-1 flex items-center justify-center relative">
            <div class="absolute top-2 left-2 flex gap-2 z-10">
                <button
                    class="toggle-btn"
                    class:active={!showPomodoro}
                    on:click={() => (showPomodoro = false)}
                >
                    Clock
                </button>
                <button
                    class="toggle-btn"
                    class:active={showPomodoro}
                    on:click={() => (showPomodoro = true)}
                >
                    Pomodoro
                </button>
            </div>

            <div class:hidden={!showPomodoro} class="h-full w-full">
                <Pomodoro on:changeSession={onPomodoroChangeSession} />
            </div>

            <div class:hidden={showPomodoro} class="h-full w-full">
                <Clock color={clockTint} />
            </div>
        </div>

        <div class="col-span-1 row-span-1 p-1 flex items-center justify-center">
            <Calendar />
        </div>
        <div class="col-span-1 row-span-1 p-1 overflow-y-auto">
            <CoinGrid {coinInfo} {currentChartIdx} {updateCurrentChart} />
        </div>
        <div class="col-span-1 row-span-1 p-1">
            <div class="h-full flex flex-col">
                {#if currentChartData?.prices}
                    <div>
                        <CoinChart prices={currentChartData.prices} coinName={currentChartName} />
                    </div>
                {/if}
                <div class="flex justify-center items-center gap-3">
                    {#each [{value: 90, text: '3M'}, {value: 180, text: '6M'}, {value: 365, text: '1Y'}] as item}
                        <button
                            on:click={() => setTimeframe(item.value)}
                            class:selected={timeframeDays === item.value}
                        >
                            {item.text}
                        </button>
                    {/each}
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .selected {
        @apply bg-blue-600 text-white bg-gray-800 rounded;
    }
    .toggle-btn {
        @apply px-3 py-1 rounded bg-gray-800 text-white text-sm;
    }
    .toggle-btn.active {
        @apply bg-blue-600;
    }
</style>

<script lang="ts">
    import axios from 'axios'
    import {onDestroy, onMount} from 'svelte'
    import Calendar from './Calendar.svelte'
    import Clock from './Clock.svelte'
    import Pomodoro from './Pomodoro.svelte'
    import CoinChart from './CoinChart.svelte'
    import CoinGrid from './CoinGrid.svelte'
    import {SessionColors, SessionType} from './types'
    import {syncEnabled} from './sync/store'
    import {roomState, sendAction, serverOffsetMs} from './sync/syncClient'
    import type {SyncedRoomState} from './sync/types'

    // Toggle Clock/Pomodoro in the Clock quadrant
    let localShowPomodoro = false

    $: synced = $syncEnabled
    $: syncedState = $roomState as SyncedRoomState | null

    $: showPomodoro = synced ? !!syncedState?.showPomodoro : localShowPomodoro

    let pomoSessionType: SessionType | null = null

    $: clockTint = (pomoSessionType && SessionColors[pomoSessionType]) || 'white'

    function onPomodoroChangeSession(e: CustomEvent<{sessionType: SessionType | null}>) {
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
    let localChartIdx = 0
    let localTimeframeDays: number = 365

    $: currentChartIdx = synced ? syncedState?.currentChartIdx ?? 0 : localChartIdx
    $: timeframeDays = synced ? syncedState?.timeframeDays ?? 365 : localTimeframeDays
    $: if (synced && syncedState) {
        ;(async () => {
            const idx = syncedState.currentChartIdx ?? 0
            const days = syncedState.timeframeDays ?? 365
            const coin = coins[idx]
            if (!coin) return
            currentChartName = coin.name
            const data = await getChart(coin.id, days)
            currentChartData = data ? data : null
        })()
    }

    let currentChartData: {prices: [number, number][]} | null = null
    let currentChartName: string = coins[0].name

    let getCoinsInterval: any = null
    let rotationInterval: any = null

    function setTimeframe(days: number) {
        if (synced) {
            sendAction({type: 'SET_TIMEFRAME_DAYS', value: days})
            return
        }
        localTimeframeDays = days
        updateCurrentChart()
    }

    function startRotation() {
        if (synced) return
        rotationInterval = setInterval(async () => {
            const nextIdx = (localChartIdx + 1) % coins.length
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
        if (synced) {
            if (idx !== undefined) {
                sendAction({type: 'SET_CHART_IDX', value: idx})
            }
            return
        }

        if (idx !== undefined) {
            localChartIdx = idx
            resetRotation()
        }

        const coin = coins[localChartIdx]
        currentChartName = coin.name
        const data = await getChart(coin.id, localTimeframeDays)
        currentChartData = data ? data : null
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

        if (!synced) startRotation()

        getCoinsInterval = setInterval(getCoins, import.meta.env.VITE_COIN_REFRESH_INTERVAL * 1000)
    })

    onDestroy(() => {
        if (rotationInterval) clearInterval(rotationInterval)
        if (getCoinsInterval) clearInterval(getCoinsInterval)
    })
</script>

<div class="h-screen text-white">
    <div class="grid grid-cols-1 sm:grid-cols-2 sm:gap-0 h-full">
        <!-- Clock quadrant with Clock/Pomodoro toggle -->
        <div class="col-span-1 row-span-1 p-1 flex items-center justify-center relative">
            <div class="absolute top-2 left-2 flex gap-2 z-10">
                <button
                    class="toggle-btn"
                    class:active={!showPomodoro}
                    on:click={() => {
                        if (synced) sendAction({type: 'SET_SHOW_POMODORO', value: false})
                        else localShowPomodoro = false
                    }}
                >
                    ⏰
                </button>

                <button
                    class="toggle-btn"
                    class:active={showPomodoro}
                    on:click={() => {
                        if (synced) sendAction({type: 'SET_SHOW_POMODORO', value: true})
                        else localShowPomodoro = true
                    }}
                >
                    🍅
                </button>
            </div>

            <div class:hidden={!showPomodoro} class="h-full w-full">
                <Pomodoro
                    on:changeSession={onPomodoroChangeSession}
                    {synced}
                    syncState={syncedState?.pomodoro}
                    serverOffsetMs={$serverOffsetMs}
                    {sendAction}
                />
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
        @apply px-2 py-1 rounded bg-gray-800 text-white text-xs;
    }
    .toggle-btn.active {
        @apply bg-blue-600;
    }
</style>

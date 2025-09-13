<script lang="ts">
    import {fmtNumber, fmtPercent} from './helpers'

    export let coinInfo: {
        id: string
        name: string
        usd: number
        usd_24h_change: number
    }[] = []
    export let currentChartIdx = 0
    export let updateCurrentChart: (idx: number) => void
</script>

<div class="grid grid-cols-3 gap-2">
    {#each coinInfo as coin, i}
        <button
            class={`w-full p-1 rounded-lg transition-colors duration-200 
                    ${currentChartIdx === i ? 'bg-gray-500' : 'bg-gray-800 hover:bg-gray-900'}`}
            on:click={() => updateCurrentChart(i)}
        >
            <div class="flex items-center justify-between text-sm text-white">
                <div class="flex items-baseline gap-2">
                    <span class="font-semibold">{coin.name}</span>
                    <span class="text-gray-300">{fmtNumber(coin.usd, 4)}</span>
                </div>
                <span class={coin.usd_24h_change > 0 ? 'text-green-400' : 'text-red-400'}>
                    {fmtPercent(coin.usd_24h_change)}
                </span>
            </div>
        </button>
    {/each}
</div>

<script lang="ts">
    import dayjs from 'dayjs'
    import {onMount} from 'svelte'

    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
    let current = new Date()

    const getDaysInMonth = (month: number, year: number) => {
        var date = new Date(year, month, 1)
        var days: Date[] = []
        while (date.getMonth() === month) {
            days.push(new Date(date))
            date.setDate(date.getDate() + 1)
        }
        return days
    }

    const loadDates = (cur: Date) => {
        const datesInMonth = getDaysInMonth(cur.getMonth(), cur.getFullYear())
        const firstDateIndex = days.findIndex((_, i) => i === datesInMonth[0].getDay())

        return Array(firstDateIndex).fill(null).concat(datesInMonth)
    }

    $: dates = loadDates(current)

    const isWeekend = (date: Date | null) => {
        return date?.getDay() === 0 || date?.getDay() === 6
    }

    const isToday = (date: Date | null) => {
        const now = new Date()
        return now.getMonth() === date?.getMonth() && now.getDate() === date?.getDate()
    }

    const onToday = () => {
        current = new Date()
    }

    const onMonthAdd = (d: number) => {
        current = dayjs(current).add(d, 'month').toDate()
    }

    onMount(() => {
        const interval = setInterval(
            () => {
                onToday()
            },
            6 * 60 * 60 * 1000
        )
        return () => {
            clearInterval(interval)
        }
    })
</script>

<div class="flex gap-2">
    <div class="flex flex-col items-center p-1 gap-2 mb-1">
        <div class="font-bold text-base">
            {dayjs(current).format('MMM YYYY')}
        </div>
        <div class="flex flex-col gap-1">
            {#each [{onClick: () => onToday(), label: 'Today'}, {onClick: () => onMonthAdd(-1), label: 'Back'}, {onClick: () => onMonthAdd(1), label: 'Next'}] as item}
                <button
                    class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
                    type="button"
                    on:click={item.onClick}
                >
                    {item.label}
                </button>
            {/each}
        </div>
    </div>

    <div class="grid grid-cols-7 gap-0.5 border-t border-b border-gray-200 pt-1">
        {#each days as day}
            <div class="text-center text-sm text-gray-500 font-semibold">{day}</div>
        {/each}
        {#each dates as date}
            <div class="p-1 flex items-center justify-center">
                {#if date}
                    <div class="relative w-full h-full flex items-center justify-center">
                        {#if isToday(date)}
                            <span class="absolute inset-0 rounded-md border-2 border-red-500" />
                        {/if}
                        <span class="relative z-10" class:text-gray-500={isWeekend(date)}>
                            {dayjs(date).format('D')}
                        </span>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

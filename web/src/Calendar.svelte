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

<div class="flex flex-col items-center justify-center text-xs md:text-base">
    <div class="w-full">
        <div class="calendar-month-container flex items-center justify-between mb-1">
            <div class="calendar-month font-bold text-base">{dayjs(current).format('MMM YYYY')}</div>
            <div class="calendar-control flex gap-1">
                <button
                    class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
                    type="button"
                    on:click={onToday}>Today</button
                >
                <button
                    class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
                    type="button"
                    on:click={() => onMonthAdd(-1)}>Back</button
                >
                <button
                    class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
                    type="button"
                    on:click={() => onMonthAdd(1)}>Next</button
                >
            </div>
        </div>

        <div class="grid grid-cols-7 gap-0.5 border-t border-b border-gray-200 pt-1">
            {#each days as day}
                <div class="text-center text-gray-500 font-semibold pb-0.5">{day}</div>
            {/each}
            {#each dates as date}
                <div class="cell p-1 flex items-center justify-center">
                    {#if date}
                        <div class="relative w-full h-full flex items-center justify-center">
                            {#if isToday(date)}
                                <span class="absolute inset-0 rounded-md border-2 border-red-500"
                                ></span>
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
</div>

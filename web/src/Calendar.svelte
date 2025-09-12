<script lang="ts">
    import {addMonths, format} from 'date-fns'
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
        current = addMonths(current, d)
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

<div class="calendar">
    <div>
        <div class="calendar-month-container">
            <div class="calendar-month">{format(current, 'MMMM yyyy')}</div>
            <div class="calendar-control">
                <button type="button" on:click={() => onToday()}>Today</button>
                <button type="button" on:click={() => onMonthAdd(-1)}>Back</button>
                <button type="button" on:click={() => onMonthAdd(1)}>Next</button>
            </div>
        </div>
        <div class="calendar-table">
            {#each days as day}
                <div class="head cell">{day}</div>
            {/each}
            {#each dates as date}
                <div class="cell">
                    <div class:today={isToday(date)} class:weekend={isWeekend(date)}>
                        {date?.getDate() || ''}
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .calendar {
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 2vw;
    }
    .calendar-month-container {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .calendar-month {
        font-weight: bold;
        padding-left: 0.5em;
    }
    .calendar-control {
        display: flex;
        gap: 0.5em;
    }
    .calendar-table {
        display: grid;
        grid-template-columns: repeat(7, 1fr);
    }
    .calendar-table > .head {
        color: #686868;
    }
    .calendar-table > .cell {
        padding: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .today {
        color: #ff3636;
        font-weight: bold;
        position: relative;
    }
    .today::before {
        content: '';
        position: absolute;
        top: -0.2em;
        left: -0.25em;
        right: -0.25em;
        bottom: -0.2em;
        border-radius: 5px;
        border: 1px solid #ff3636;
    }
    .weekend {
        color: #686868;
    }
</style>

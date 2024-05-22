<script lang="ts">
  import {onMount} from 'svelte'

  const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
  let dates: (Date | null)[] = []

  const getDaysInMonth = (month: number, year: number) => {
    var date = new Date(year, month, 1)
    var days: Date[] = []
    while (date.getMonth() === month) {
      days.push(new Date(date))
      date.setDate(date.getDate() + 1)
    }
    return days
  }

  onMount(() => {
    const now = new Date()
    const datesInMonth = getDaysInMonth(now.getMonth(), now.getFullYear())
    const firstDateIndex = days.findIndex(
      (_, i) => i === datesInMonth[0].getDay()
    )

    dates = Array(firstDateIndex).fill(null).concat(datesInMonth)
  })

  const isWeekend = (date: Date | null) => {
    return date?.getDay() === 0 || date?.getDay() === 6
  }

  const isToday = (date: Date | null) => {
    const now = new Date()
    return now.getDate() === date?.getDate()
  }
</script>

<div class="calendar">
  <div>
    <div class="calendar-table">
      {#each days as day}
        <div class="head cell">{day}</div>
      {/each}
      {#each dates as date}
        <div
          class="cell"
          class:today={isToday(date)}
          class:weekend={isWeekend(date)}
        >
          <div>
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
    font-size: 1.8rem;
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
    border-radius: 5px;
    background-color: #686868;
  }
  .weekend {
    color: #686868;
  }
</style>

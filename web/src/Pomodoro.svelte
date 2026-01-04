<script lang="ts">
    import {createEventDispatcher, onDestroy} from 'svelte'
    import {SessionColors, SessionType} from './types'

    const sessionOpts = [SessionType.Work, SessionType.Short, SessionType.Long]

    const dispatch = createEventDispatcher<{
        changeSession: {sessionType: SessionType | null}
    }>()

    // Durations (seconds)
    const WORK_SECONDS = 25 * 60
    const SHORT_BREAK_SECONDS = 5 * 60
    const LONG_BREAK_SECONDS = 15 * 60

    // After 4 work sessions, the break becomes long
    const LONG_BREAK_EVERY = 4

    let sessionType: SessionType = SessionType.Work
    let activeColor: string = SessionColors[SessionType.Work]

    // Keep activeColor in sync with sessionType
    $: activeColor = SessionColors[sessionType]

    let remainingSeconds = WORK_SECONDS
    let running = false
    let interval: number | null = null
    let workSessionsCompleted = 0

    function secondsFor(type: SessionType) {
        if (type === SessionType.Work) return WORK_SECONDS
        if (type === SessionType.Short) return SHORT_BREAK_SECONDS
        return LONG_BREAK_SECONDS
    }

    function formatMMSS(totalSeconds: number) {
        const m = Math.floor(totalSeconds / 60)
        const s = totalSeconds % 60
        return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
    }

    function start() {
        if (running) return
        running = true
        interval = window.setInterval(() => {
            if (!running) return
            if (remainingSeconds > 0) remainingSeconds -= 1
            if (remainingSeconds <= 0) finishSession()
        }, 1000)

        dispatch('changeSession', {sessionType})
    }

    function pause() {
        running = false
        if (interval) {
            clearInterval(interval)
            interval = null
        }
        dispatch('changeSession', {sessionType: null})
    }

    function reset() {
        pause()
        remainingSeconds = secondsFor(sessionType)
        dispatch('changeSession', {sessionType: null})
    }

    function setSession(type: SessionType) {
        pause()
        sessionType = type
        remainingSeconds = secondsFor(type)
    }

    function nextSessionAfter(current: SessionType): SessionType {
        if (current === SessionType.Work) {
            workSessionsCompleted += 1
            return workSessionsCompleted % LONG_BREAK_EVERY === 0
                ? SessionType.Long
                : SessionType.Short
        }
        return SessionType.Work
    }

    function finishSession() {
        pause()
        sessionType = nextSessionAfter(sessionType)
        remainingSeconds = secondsFor(sessionType)
        dispatch('changeSession', {sessionType: null})
    }

    function skip() {
        finishSession()
    }

    onDestroy(() => pause())
</script>

<div class="h-full w-full flex flex-col items-center justify-center gap-2 select-none">
    <div class="text-sm text-gray-300 font-semibold">Pomodoro</div>

    <div class="text-base font-bold" style="color: {SessionColors[sessionType]}">
        {sessionType}
    </div>

    <div class="text-6xl font-mono leading-none text-white">
        {formatMMSS(remainingSeconds)}
    </div>

    <div class="flex gap-2 mt-2">
        <button
            class={`px-2 py-0.5 rounded transition-colors duration-200 ${
                running ? 'bg-gray-500' : 'bg-gray-800 hover:bg-gray-700'
            }`}
            on:click={() => (running ? pause() : start())}
            type="button"
        >
            {running ? 'Pause' : 'Start'}
        </button>
        <button
            class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
            on:click={reset}
            type="button"
        >
            Reset
        </button>
        <button
            class="px-2 py-0.5 rounded bg-gray-800 hover:bg-gray-700"
            on:click={skip}
            type="button"
        >
            Skip
        </button>
    </div>

    <div class="flex gap-2 mt-1">
        {#each sessionOpts as item}
            <button
                class={`px-2 py-0.5 rounded transition-colors duration-200 ${
                    sessionType === item ? 'bg-gray-500' : 'bg-gray-800 hover:bg-gray-700'
                }`}
                on:click={() => setSession(item)}
                type="button"
            >
                {item}
            </button>
        {/each}
    </div>
</div>

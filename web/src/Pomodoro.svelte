<script lang="ts">
    import {createEventDispatcher, onDestroy, onMount} from 'svelte'
    import {SessionColors, SessionType} from './types'

    const sessionOpts = [SessionType.Work, SessionType.Short, SessionType.Long]

    const dispatch = createEventDispatcher<{
        changeSession: {sessionType: SessionType | null}
    }>()

    /* -------------------- Constants -------------------- */

    const WORK_SECONDS = 25 * 60
    const SHORT_BREAK_SECONDS = 5 * 60
    const LONG_BREAK_SECONDS = 15 * 60
    const LONG_BREAK_EVERY = 4

    const STORAGE_KEY = 'pomodoroState'

    /* -------------------- State -------------------- */

    let sessionType: SessionType = SessionType.Work
    let remainingSeconds = WORK_SECONDS
    let running = false
    let interval: number | null = null
    let startedAt: number | null = null
    let workSessionsCompleted = 0

    /* ---- Counters ---- */

    let workCount = 0
    let shortCount = 0
    let longCount = 0

    $: activeColor = SessionColors[sessionType]

    /* -------------------- Helpers -------------------- */

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

    /* -------------------- Persistence -------------------- */

    function saveState() {
        localStorage.setItem(
            STORAGE_KEY,
            JSON.stringify({
                sessionType,
                remainingSeconds,
                running,
                startedAt,
                workSessionsCompleted,
                counters: {
                    workCount,
                    shortCount,
                    longCount,
                },
            })
        )
    }

    function clearState() {
        localStorage.removeItem(STORAGE_KEY)
    }

    function restoreState() {
        try {
            const raw = localStorage.getItem(STORAGE_KEY)
            if (!raw) return

            const data = JSON.parse(raw)

            sessionType = data.sessionType
            remainingSeconds = data.remainingSeconds
            running = data.running
            startedAt = data.startedAt
            workSessionsCompleted = data.workSessionsCompleted ?? 0

            const counters = data.counters ?? {}
            workCount = counters.workCount ?? 0
            shortCount = counters.shortCount ?? 0
            longCount = counters.longCount ?? 0

            if (running && startedAt) {
                const elapsed = Math.floor((Date.now() - startedAt) / 1000)
                remainingSeconds -= elapsed

                if (remainingSeconds <= 0) {
                    finishSession()
                    return
                }

                startInterval()
                dispatch('changeSession', {sessionType})
            }
        } catch {
            clearState()
        }
    }

    /* -------------------- Timer Logic -------------------- */

    function startInterval() {
        interval = window.setInterval(() => {
            if (!running) return
            remainingSeconds -= 1
            saveState()

            if (remainingSeconds <= 0) {
                finishSession()
            }
        }, 1000)
    }

    function start() {
        if (running) return
        running = true
        startedAt = Date.now()
        startInterval()
        saveState()
        dispatch('changeSession', {sessionType})
    }

    function pause() {
        running = false
        startedAt = null
        if (interval) {
            clearInterval(interval)
            interval = null
        }
        saveState()
        dispatch('changeSession', {sessionType: null})
    }

    function reset() {
        pause()
        sessionType = SessionType.Work
        remainingSeconds = WORK_SECONDS
        workSessionsCompleted = 0

        workCount = 0
        shortCount = 0
        longCount = 0

        clearState()
        dispatch('changeSession', {sessionType: null})
    }

    function setSession(type: SessionType) {
        pause()
        sessionType = type
        remainingSeconds = secondsFor(type)
        saveState()
    }

    function incrementCounters(type: SessionType) {
        if (type === SessionType.Work) workCount += 1
        if (type === SessionType.Short) shortCount += 1
        if (type === SessionType.Long) longCount += 1
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

        // Increment counters for the session that just finished
        incrementCounters(sessionType)

        sessionType = nextSessionAfter(sessionType)
        remainingSeconds = secondsFor(sessionType)

        saveState()
        dispatch('changeSession', {sessionType: null})
    }

    function skip() {
        finishSession()
    }

    /* -------------------- Lifecycle -------------------- */

    onMount(restoreState)
    onDestroy(pause)
</script>

<div class="h-full w-full flex flex-col items-center justify-center gap-2 select-none">
    <div class="text-base font-bold" style="color: {SessionColors[sessionType]}">
        {sessionType}
    </div>

    <div class="text-6xl font-mono leading-none text-white">
        {formatMMSS(remainingSeconds)}
    </div>

    <!-- Counters -->
    <div class="grid grid-cols-3 gap-x-6 gap-y-1 text-sm text-gray-300 mt-2">
        <div>Work: <span class="text-white">{workCount}</span></div>
        <div>Short: <span class="text-white">{shortCount}</span></div>
        <div>Long: <span class="text-white">{longCount}</span></div>
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

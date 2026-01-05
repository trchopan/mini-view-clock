<script lang="ts">
    import {createEventDispatcher, onDestroy, onMount} from 'svelte'
    import {SessionColors, SessionType} from './types'

    // Sync props
    export let synced: boolean = false
    export let syncState: {
        sessionType: SessionType
        mode: 'idle' | 'running' | 'paused'
        durationSec: number
        startedAtMs: number | null
        pausedAtMs: number | null
        elapsedBeforePauseMs: number
        workSessionsCompleted: number
        workCount: number
        shortCount: number
        longCount: number
        config: {
            workSec: number
            shortSec: number
            longSec: number
            longBreakEvery: number
        }
    } | null = null
    export let serverOffsetMs: number = 0
    export let sendAction: (action: any) => void = () => {}

    const sessionOpts = [SessionType.Work, SessionType.Short, SessionType.Long]

    const dispatch = createEventDispatcher<{
        changeSession: {sessionType: SessionType | null}
    }>()

    /* -------------------- Local mode constants -------------------- */

    const WORK_SECONDS = 25 * 60
    const SHORT_BREAK_SECONDS = 5 * 60
    const LONG_BREAK_SECONDS = 15 * 60
    const LONG_BREAK_EVERY = 4

    const STORAGE_KEY = 'pomodoroState'

    /* -------------------- Local mode state -------------------- */

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

    /* -------------------- Derived / Sync display -------------------- */

    let displayRemainingSeconds = remainingSeconds
    let displaySessionType: SessionType = sessionType
    let displayWorkCount = workCount
    let displayShortCount = shortCount
    let displayLongCount = longCount
    let displayRunning = running

    $: activeColor = SessionColors[displaySessionType]

    function effectiveServerNowMs() {
        return Date.now() + (serverOffsetMs || 0)
    }

    function secondsForLocal(type: SessionType) {
        if (type === SessionType.Work) return WORK_SECONDS
        if (type === SessionType.Short) return SHORT_BREAK_SECONDS
        return LONG_BREAK_SECONDS
    }

    function formatMMSS(totalSeconds: number) {
        const m = Math.floor(totalSeconds / 60)
        const s = totalSeconds % 60
        return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`
    }

    function computeSyncedRemainingSeconds() {
        if (!syncState) return 0
        const p = syncState
        const now = effectiveServerNowMs()

        // elapsedMs
        let elapsedMs = 0
        if (p.mode === 'idle') {
            elapsedMs = 0
        } else if (!p.startedAtMs) {
            elapsedMs = 0
        } else if (p.mode === 'running') {
            elapsedMs = now - p.startedAtMs + (p.elapsedBeforePauseMs || 0)
        } else {
            // paused
            if (p.pausedAtMs) {
                elapsedMs = p.pausedAtMs - p.startedAtMs + (p.elapsedBeforePauseMs || 0)
            } else {
                elapsedMs = p.elapsedBeforePauseMs || 0
            }
        }

        const remainingMs = p.durationSec * 1000 - elapsedMs
        const remainingSec = Math.max(0, Math.ceil(remainingMs / 1000))
        return remainingSec
    }

    /* -------------------- Local persistence -------------------- */

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

    /* -------------------- Local timer logic -------------------- */

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
        remainingSeconds = secondsForLocal(type)
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
        incrementCounters(sessionType)
        sessionType = nextSessionAfter(sessionType)
        remainingSeconds = secondsForLocal(sessionType)
        saveState()
        dispatch('changeSession', {sessionType: null})
    }

    function skip() {
        finishSession()
    }

    /* -------------------- Synced actions -------------------- */

    function syncedStartPauseToggle() {
        if (!syncState) return
        if (syncState.mode === 'running') sendAction({type: 'POMO_PAUSE'})
        else if (syncState.mode === 'paused') sendAction({type: 'POMO_RESUME'})
        else sendAction({type: 'POMO_START'})
    }

    function syncedReset() {
        sendAction({type: 'POMO_RESET'})
    }

    function syncedSkip() {
        sendAction({type: 'POMO_SKIP'})
    }

    function syncedSetSession(type: SessionType) {
        sendAction({type: 'POMO_SET_SESSION', sessionType: type})
    }

    /* -------------------- Lifecycle -------------------- */

    // In sync mode, do NOT restore local storage. Just run a lightweight render tick.
    let renderTick: number | null = null

    onMount(() => {
        if (!synced) {
            restoreState()
        }

        renderTick = window.setInterval(() => {
            if (synced) {
                if (!syncState) return
                displaySessionType = syncState.sessionType
                displayRemainingSeconds = computeSyncedRemainingSeconds()
                displayWorkCount = syncState.workCount
                displayShortCount = syncState.shortCount
                displayLongCount = syncState.longCount
                displayRunning = syncState.mode === 'running'

                // clock tint behavior: sessionType only while running
                dispatch('changeSession', {
                    sessionType: syncState.mode === 'running' ? syncState.sessionType : null,
                })
            } else {
                displaySessionType = sessionType
                displayRemainingSeconds = remainingSeconds
                displayWorkCount = workCount
                displayShortCount = shortCount
                displayLongCount = longCount
                displayRunning = running
            }
        }, 250)
    })

    onDestroy(() => {
        if (!synced) pause()
        if (renderTick) clearInterval(renderTick)
    })
</script>

<div class="h-full w-full flex items-center justify-center select-none">
    <div class="w-full max-w-xl flex items-center justify-center gap-6">
        <div class="flex flex-col items-center justify-center gap-2 flex-1">
            <div class="text-base font-bold" style="color: {SessionColors[displaySessionType]}">
                {displaySessionType}
            </div>

            <div
                class="font-mono leading-none text-white w-full text-center"
                style="font-size: clamp(2.5rem, 10vw, 7rem);"
            >
                {formatMMSS(displayRemainingSeconds)}
            </div>

            <div class="grid grid-cols-3 gap-x-6 gap-y-1 text-sm text-gray-300 mt-2">
                <div>Work: <span class="text-white">{displayWorkCount}</span></div>
                <div>Short: <span class="text-white">{displayShortCount}</span></div>
                <div>Long: <span class="text-white">{displayLongCount}</span></div>
            </div>
        </div>

        <div class="w-32 grid grid-cols-2 gap-1 auto-rows-min">
            <div class="flex flex-col gap-1">
                {#each sessionOpts as item}
                    <button
                        class={`pomo-btn ${displaySessionType === item && '!bg-gray-500'}`}
                        on:click={() => (synced ? syncedSetSession(item) : setSession(item))}
                        type="button"
                    >
                        {item}
                    </button>
                {/each}
            </div>

            <div class="flex flex-col gap-1">
                <button
                    class={`pomo-btn ${displayRunning && '!bg-gray-500'}`}
                    on:click={() =>
                        synced ? syncedStartPauseToggle() : running ? pause() : start()}
                    type="button"
                >
                    {synced
                        ? syncState?.mode === 'running'
                            ? 'Pause'
                            : syncState?.mode === 'paused'
                              ? 'Resume'
                              : 'Start'
                        : running
                          ? 'Pause'
                          : 'Start'}
                </button>

                <button
                    class="pomo-btn"
                    on:click={() => (synced ? syncedReset() : reset())}
                    type="button"
                >
                    Reset
                </button>

                <button
                    class="pomo-btn"
                    on:click={() => (synced ? syncedSkip() : skip())}
                    type="button"
                >
                    Skip
                </button>
            </div>
        </div>
    </div>
</div>

<style>
    .pomo-btn {
        @apply w-full px-2 py-1 rounded bg-gray-800 text-sm;
    }
</style>

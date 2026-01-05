export type SessionType = 'Work' | 'Short' | 'Long';
export type PomodoroMode = 'idle' | 'running' | 'paused';

export type RoomState = {
    showPomodoro: boolean;
    currentChartIdx: number;
    timeframeDays: number;
    pomodoro: {
        sessionType: SessionType;
        mode: PomodoroMode;
        durationSec: number;

        startedAtMs: number | null;
        pausedAtMs: number | null;
        elapsedBeforePauseMs: number;

        workSessionsCompleted: number;

        workCount: number;
        shortCount: number;
        longCount: number;

        config: {
            workSec: number;
            shortSec: number;
            longSec: number;
            longBreakEvery: number;
        };
    };
};

export type Action =
    | {type: 'SET_SHOW_POMODORO'; value: boolean}
    | {type: 'SET_CHART_IDX'; value: number}
    | {type: 'SET_TIMEFRAME_DAYS'; value: number}
    | {type: 'POMO_SET_SESSION'; sessionType: SessionType}
    | {type: 'POMO_START'}
    | {type: 'POMO_PAUSE'}
    | {type: 'POMO_RESUME'}
    | {type: 'POMO_RESET'}
    | {type: 'POMO_SKIP'};

export function defaultState(): RoomState {
    const config = {workSec: 25 * 60, shortSec: 5 * 60, longSec: 15 * 60, longBreakEvery: 4};
    return {
        showPomodoro: false,
        currentChartIdx: 0,
        timeframeDays: 365,
        pomodoro: {
            sessionType: 'Work',
            mode: 'idle',
            durationSec: config.workSec,
            startedAtMs: null,
            pausedAtMs: null,
            elapsedBeforePauseMs: 0,
            workSessionsCompleted: 0,
            workCount: 0,
            shortCount: 0,
            longCount: 0,
            config,
        },
    };
}

function secondsFor(cfg: RoomState['pomodoro']['config'], type: SessionType) {
    if (type === 'Work') return cfg.workSec;
    if (type === 'Short') return cfg.shortSec;
    return cfg.longSec;
}

function nextSessionAfter(state: RoomState): SessionType {
    // matches your existing logic
    const p = state.pomodoro;
    if (p.sessionType === 'Work') {
        const completed = p.workSessionsCompleted + 1;
        const isLong = completed % p.config.longBreakEvery === 0;
        return isLong ? 'Long' : 'Short';
    }
    return 'Work';
}

function incrementCounter(p: RoomState['pomodoro']) {
    if (p.sessionType === 'Work') p.workCount += 1;
    if (p.sessionType === 'Short') p.shortCount += 1;
    if (p.sessionType === 'Long') p.longCount += 1;
}

function pomodoroElapsedMs(p: RoomState['pomodoro'], nowMs: number): number {
    if (p.mode === 'idle') return 0;
    if (!p.startedAtMs) return 0;

    if (p.mode === 'running') {
        return nowMs - p.startedAtMs + p.elapsedBeforePauseMs;
    }
    // paused
    if (!p.pausedAtMs) return p.elapsedBeforePauseMs;
    return p.pausedAtMs - p.startedAtMs + p.elapsedBeforePauseMs;
}

function maybeAutoFinish(state: RoomState, nowMs: number): RoomState {
    const p = state.pomodoro;
    if (p.mode !== 'running') return state;

    const elapsed = pomodoroElapsedMs(p, nowMs);
    const remainingMs = p.durationSec * 1000 - elapsed;

    if (remainingMs > 0) return state;

    // finish
    const finishedSessionType = p.sessionType;

    const next: RoomState = structuredClone(state);
    const np = next.pomodoro;

    // stop timer
    np.mode = 'idle';
    np.startedAtMs = null;
    np.pausedAtMs = null;
    np.elapsedBeforePauseMs = 0;

    // increment counters for session that ended
    incrementCounter(np);

    if (finishedSessionType === 'Work') {
        np.workSessionsCompleted += 1;
    }

    // advance session type
    np.sessionType = nextSessionAfter(next);
    np.durationSec = secondsFor(np.config, np.sessionType);

    return next;
}

export function reduce(state: RoomState, action: Action, nowMs: number): RoomState {
    // Always check auto-finish first so state stays consistent
    let s = maybeAutoFinish(state, nowMs);

    switch (action.type) {
        case 'SET_SHOW_POMODORO':
            return {...s, showPomodoro: action.value};

        case 'SET_CHART_IDX':
            return {...s, currentChartIdx: action.value};

        case 'SET_TIMEFRAME_DAYS':
            return {...s, timeframeDays: action.value};

        case 'POMO_SET_SESSION': {
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;
            // stop timer like your pause()
            p.mode = 'idle';
            p.startedAtMs = null;
            p.pausedAtMs = null;
            p.elapsedBeforePauseMs = 0;

            p.sessionType = action.sessionType;
            p.durationSec = secondsFor(p.config, p.sessionType);
            return next;
        }

        case 'POMO_START': {
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;
            if (p.mode === 'running') return next;

            // start from zero for current session
            p.mode = 'running';
            p.startedAtMs = nowMs;
            p.pausedAtMs = null;
            p.elapsedBeforePauseMs = 0;
            p.durationSec = secondsFor(p.config, p.sessionType);
            return next;
        }

        case 'POMO_PAUSE': {
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;
            if (p.mode !== 'running') return next;
            p.mode = 'paused';
            p.pausedAtMs = nowMs;
            return next;
        }

        case 'POMO_RESUME': {
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;
            if (p.mode !== 'paused') return next;
            if (!p.startedAtMs || !p.pausedAtMs) {
                // treat as fresh start if anchors are missing
                p.mode = 'running';
                p.startedAtMs = nowMs;
                p.pausedAtMs = null;
                p.elapsedBeforePauseMs = 0;
                return next;
            }
            // move elapsed into accumulator, restart startedAtMs
            p.elapsedBeforePauseMs += p.pausedAtMs - p.startedAtMs;
            p.startedAtMs = nowMs;
            p.pausedAtMs = null;
            p.mode = 'running';
            return next;
        }

        case 'POMO_RESET': {
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;

            p.mode = 'idle';
            p.sessionType = 'Work';
            p.startedAtMs = null;
            p.pausedAtMs = null;
            p.elapsedBeforePauseMs = 0;
            p.workSessionsCompleted = 0;
            p.workCount = 0;
            p.shortCount = 0;
            p.longCount = 0;
            p.durationSec = secondsFor(p.config, p.sessionType);
            return next;
        }

        case 'POMO_SKIP': {
            // force finish regardless of remaining
            const next: RoomState = structuredClone(s);
            const p = next.pomodoro;

            // stop
            p.mode = 'idle';
            p.startedAtMs = null;
            p.pausedAtMs = null;
            p.elapsedBeforePauseMs = 0;

            // increment counters for session that was active
            incrementCounter(p);
            if (p.sessionType === 'Work') p.workSessionsCompleted += 1;

            p.sessionType = nextSessionAfter(next);
            p.durationSec = secondsFor(p.config, p.sessionType);
            return next;
        }

        default:
            return s;
    }
}

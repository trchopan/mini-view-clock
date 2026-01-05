import type {SessionType} from '../types'

export type PomodoroMode = 'idle' | 'running' | 'paused'

export type SyncedRoomState = {
    showPomodoro: boolean
    currentChartIdx: number
    timeframeDays: number
    pomodoro: {
        sessionType: SessionType
        mode: PomodoroMode
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
    }
}

export type WireIn =
    | {type: 'hello'; roomId: string; clientId: string}
    | {type: 'action'; roomId: string; clientId: string; action: any}

export type WireOut =
    | {type: 'state'; roomId: string; version: number; serverNowMs: number; state: SyncedRoomState}
    | {type: 'error'; code: string; message: string}

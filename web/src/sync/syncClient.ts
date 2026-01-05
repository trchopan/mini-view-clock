import {get, writable} from 'svelte/store'
import {syncRoomId, syncEnabled} from './store'
import type {SyncedRoomState, WireOut} from './types'

function makeClientId() {
    try {
        return crypto.randomUUID()
    } catch {
        return Math.random().toString(16).slice(2) + Date.now().toString(16)
    }
}

const clientId = makeClientId()

export const connected = writable(false)
export const lastError = writable<string | null>(null)
export const serverOffsetMs = writable(0) // serverNowMs - Date.now()
export const roomState = writable<SyncedRoomState | null>(null)
export const roomVersion = writable<number>(0)

let ws: WebSocket | null = null
let reconnectTimer: number | null = null
let reconnectAttempt = 0

function clearReconnect() {
    if (reconnectTimer) {
        clearTimeout(reconnectTimer)
        reconnectTimer = null
    }
}

function scheduleReconnect() {
    clearReconnect()
    // bounded exponential backoff
    const delay = Math.min(1000 * Math.pow(2, reconnectAttempt), 15000)
    reconnectAttempt += 1
    reconnectTimer = window.setTimeout(() => {
        const rid = get(syncRoomId)
        const enabled = get(syncEnabled)
        if (enabled && rid) connect(rid)
    }, delay)
}

function getWsBaseUrl(): string {
    // Configure in Firebase env (recommended):
    // VITE_SYNC_WS_URL=wss://your-sync-server.example.com/ws
    const envUrl = (import.meta as any).env?.VITE_SYNC_WS_URL as string | undefined
    if (envUrl) return envUrl

    // fallback: same host, /ws (only works if you proxy)
    const proto = window.location.protocol === 'https:' ? 'wss' : 'ws'
    return `${proto}://${window.location.host}/ws`
}

export function connect(roomId: string) {
    disconnect()

    const base = getWsBaseUrl()
    const url = `${base}?roomId=${encodeURIComponent(roomId)}`
    lastError.set(null)

    ws = new WebSocket(url)

    ws.onopen = () => {
        connected.set(true)
        reconnectAttempt = 0
        clearReconnect()
        ws?.send(
            JSON.stringify({
                type: 'hello',
                roomId,
                clientId,
            })
        )
    }

    ws.onmessage = ev => {
        let msg: WireOut
        try {
            msg = JSON.parse(String(ev.data))
        } catch {
            return
        }

        if (msg.type === 'error') {
            lastError.set(`${msg.code}: ${msg.message}`)
            return
        }

        if (msg.type === 'state') {
            roomState.set(msg.state)
            roomVersion.set(msg.version)
            serverOffsetMs.set(msg.serverNowMs - Date.now())
            return
        }
    }

    ws.onclose = () => {
        connected.set(false)
        // only reconnect if still in sync mode
        const enabled = get(syncEnabled)
        const rid = get(syncRoomId)
        if (enabled && rid === roomId) {
            scheduleReconnect()
        }
    }

    ws.onerror = () => {
        lastError.set('WebSocket error')
    }
}

export function disconnect() {
    clearReconnect()
    if (ws) {
        try {
            ws.close()
        } catch {}
        ws = null
    }
    connected.set(false)
}

export function sendAction(action: any) {
    const rid = get(syncRoomId)
    if (!rid || !ws || ws.readyState !== WebSocket.OPEN) return
    ws.send(
        JSON.stringify({
            type: 'action',
            roomId: rid,
            clientId,
            action,
        })
    )
}

// Auto-connect when syncRoomId/syncEnabled changes
let unsubscribed = false
syncRoomId.subscribe(rid => {
    if (unsubscribed) return
    const enabled = get(syncEnabled)
    if (!enabled) return
    if (rid) connect(rid)
})
syncEnabled.subscribe(enabled => {
    if (unsubscribed) return
    const rid = get(syncRoomId)
    if (enabled && rid) connect(rid)
    if (!enabled) disconnect()
})

// Optional: call in HMR cleanup if desired
export function destroySyncClient() {
    unsubscribed = true
    disconnect()
}

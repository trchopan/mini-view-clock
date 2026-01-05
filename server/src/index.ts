import {openDb} from './db';
import {defaultState, reduce, type Action, type RoomState} from './state';

type WireHello = {type: 'hello'; roomId: string; clientId: string};
type WireAction = {type: 'action'; roomId: string; clientId: string; action: Action};
type WireIn = WireHello | WireAction;

type WireState = {
    type: 'state';
    roomId: string;
    version: number;
    serverNowMs: number;
    state: RoomState;
};
type WireError = {type: 'error'; code: string; message: string};

const db = openDb(process.env.SQLITE_PATH ?? './sync.db');

function isValidRoomId(id: string) {
    return /^[0-9]{8}$/.test(id);
}

function roomExists(roomId: string): boolean {
    const row = db.query('SELECT id FROM rooms WHERE id = ? AND enabled = 1').get(roomId);
    return !!row;
}

function loadRoomState(roomId: string): {version: number; state: RoomState} {
    const row = db
        .query('SELECT version, state_json FROM room_state WHERE room_id = ?')
        .get(roomId) as any;
    if (!row) return {version: 0, state: defaultState()};
    try {
        return {version: Number(row.version), state: JSON.parse(row.state_json) as RoomState};
    } catch {
        return {version: 0, state: defaultState()};
    }
}

function saveRoomState(roomId: string, version: number, state: RoomState) {
    const now = Date.now();
    const json = JSON.stringify(state);
    db.query(
        `
    INSERT INTO room_state (room_id, version, state_json, updated_at)
    VALUES (?, ?, ?, ?)
    ON CONFLICT(room_id) DO UPDATE SET version=excluded.version, state_json=excluded.state_json, updated_at=excluded.updated_at
  `
    ).run(roomId, version, json, now);
}

type RoomRuntime = {
    roomId: string;
    version: number;
    state: RoomState;
    clients: Set<WebSocket>;
};

const rooms = new Map<string, RoomRuntime>();

function getOrLoadRoom(roomId: string): RoomRuntime {
    const existing = rooms.get(roomId);
    if (existing) return existing;

    const {version, state} = loadRoomState(roomId);
    const rt: RoomRuntime = {roomId, version, state, clients: new Set()};
    rooms.set(roomId, rt);
    return rt;
}

function broadcast(room: RoomRuntime) {
    const msg: WireState = {
        type: 'state',
        roomId: room.roomId,
        version: room.version,
        serverNowMs: Date.now(),
        state: room.state,
    };
    const raw = JSON.stringify(msg);
    for (const ws of room.clients) {
        try {
            ws.send(raw);
        } catch {}
    }
}

function sendError(ws: WebSocket, code: string, message: string) {
    const msg: WireError = {type: 'error', code, message};
    try {
        ws.send(JSON.stringify(msg));
    } catch {}
}

const port = Number(process.env.PORT ?? 8787);

Bun.serve({
    port,
    fetch(req, server) {
        const url = new URL(req.url);

        if (url.pathname === '/health') {
            return new Response('ok');
        }

        if (url.pathname === '/ws') {
            const roomId = url.searchParams.get('roomId') ?? '';
            if (!roomId || !isValidRoomId(roomId) || !roomExists(roomId)) {
                return new Response('room not found', {status: 404});
            }
            if (server.upgrade(req, {data: {roomId}})) return;
            return new Response('upgrade failed', {status: 400});
        }

        return new Response('not found', {status: 404});
    },
    websocket: {
        open(ws) {
            const roomId = (ws.data as any).roomId as string;
            const room = getOrLoadRoom(roomId);
            room.clients.add(ws);
            // send snapshot immediately
            const msg: WireState = {
                type: 'state',
                roomId,
                version: room.version,
                serverNowMs: Date.now(),
                state: room.state,
            };
            ws.send(JSON.stringify(msg));
        },
        close(ws) {
            const roomId = (ws.data as any).roomId as string;
            const room = rooms.get(roomId);
            if (!room) return;
            room.clients.delete(ws);
            // optional: evict room from memory if no clients
            if (room.clients.size === 0) {
                // keep cached or delete; safe to delete
                rooms.delete(roomId);
            }
        },
        message(ws, message) {
            const roomId = (ws.data as any).roomId as string;
            const room = rooms.get(roomId);
            if (!room) {
                sendError(ws, 'ROOM_NOT_LOADED', 'Room runtime missing');
                return;
            }

            let data: WireIn;
            try {
                data = JSON.parse(String(message));
            } catch {
                sendError(ws, 'BAD_JSON', 'Invalid JSON');
                return;
            }

            if (data.type === 'hello') {
                // no-op for now, snapshot already sent in open()
                return;
            }

            if (data.type === 'action') {
                const nowMs = Date.now();
                const nextState = reduce(room.state, data.action, nowMs);
                // auto-finish might not happen unless actions arrive; optional: also run a tick loop per room later
                room.state = nextState;
                room.version += 1;
                saveRoomState(roomId, room.version, room.state);
                broadcast(room);
                return;
            }

            sendError(ws, 'UNKNOWN_MSG', 'Unknown message type');
        },
    },
});

console.log(`Sync server listening on :${port}`);

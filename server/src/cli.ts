import {openDb} from './db';

const db = openDb(process.env.SQLITE_PATH ?? './sync.db');

const [, , cmd, ...args] = process.argv;

function usage() {
    console.log(`
Usage:
  bun run cli.ts rooms:add [id]
  bun run cli.ts rooms:remove <id>
  bun run cli.ts rooms:list
`);
}

if (!cmd) {
    usage();
    process.exit(1);
}

if (cmd === 'rooms:add') {
    let id: string;
    const providedId = args[0];

    if (providedId) {
        id = providedId;
        if (db.query('SELECT id FROM rooms WHERE id = ?').get(id)) {
            console.error(`Error: Room with ID '${id}' already exists.`);
            process.exit(1);
        }
    } else {
        function generateRoomId(): string {
            return Math.floor(10000000 + Math.random() * 90000000).toString();
        }

        id = generateRoomId();

        // ensure uniqueness
        while (db.query('SELECT id FROM rooms WHERE id = ?').get(id)) {
            id = generateRoomId();
        }
    }

    db.query('INSERT INTO rooms (id, created_at, enabled) VALUES (?, ?, 1)').run(id, Date.now());

    console.log(id);
    process.exit(0);
}

if (cmd === 'rooms:remove') {
    const id = args[0];
    if (!id) {
        usage();
        process.exit(1);
    }
    db.query('DELETE FROM rooms WHERE id = ?').run(id);
    db.query('DELETE FROM room_state WHERE room_id = ?').run(id);
    console.log('removed', id);
    process.exit(0);
}

if (cmd === 'rooms:list') {
    const rows = db
        .query('SELECT id, created_at, enabled FROM rooms ORDER BY created_at DESC')
        .all() as any[];
    for (const r of rows) {
        console.log(
            `${r.id}  enabled=${r.enabled}  created_at=${new Date(r.created_at).toISOString()}`
        );
    }
    process.exit(0);
}

usage();
process.exit(1);

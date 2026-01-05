import {writable} from 'svelte/store'

export const syncRoomId = writable<string | null>(null)
export const syncEnabled = writable(false)

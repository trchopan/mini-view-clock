import {writable} from 'svelte/store'
import {View} from './types'

export const currentView = writable(View.Clock)

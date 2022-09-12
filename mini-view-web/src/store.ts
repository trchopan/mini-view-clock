import {addMinutes} from 'date-fns'
import {writable} from 'svelte/store'
import {strToView, View} from './types'

export const currentView = writable(View.Clock)

// Socket Variable declaration
var mySocket: WebSocket | undefined = undefined

const socketMessageListener = (event: MessageEvent) => {
  console.log('Received')
  const data = event.data as string
  if (data.startsWith('CmdChangeView')) {
    const view = data.split(' ')[1]
    currentView.set(strToView(view))
  }
}

// Open
const socketOpenListener = (_: Event) => {
  console.log('Connected')
}

// Closed
const socketCloseListener = async () => {
  if (mySocket) {
    console.error(
      'Disconnected. Retry in 10 seconds.',
      addMinutes(new Date(), 10)
    )
    await new Promise(resolve => setTimeout(resolve, 10000))
  }
  mySocket = new WebSocket(import.meta.env.VITE_WS_SERVER + '/ws_command')
  mySocket.addEventListener('open', socketOpenListener)
  mySocket.addEventListener('message', socketMessageListener)
  mySocket.addEventListener('close', socketCloseListener)
}
socketCloseListener()

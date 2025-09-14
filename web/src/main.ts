import App from './App.svelte'
import './main.css'

const el = document.getElementById('app')
if (!el) throw new Error('No #app element found')

const app = new App({target: el})

export default app

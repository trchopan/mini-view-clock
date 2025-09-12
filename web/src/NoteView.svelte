<script lang="ts">
    import Clock from './Clock.svelte'
    import axios from 'axios'
    import {onDestroy, onMount} from 'svelte'
    import type {NoteHeader} from './types'

    let text = ''
    let isLoading = false
    const toRank = (s: string) => {
        if (s.toLowerCase().includes('[work]')) return 2
        if (s.toLowerCase().includes('[video]')) return 1
        return 0
    }
    const shouldHighlight = (content: string) => {
        const highlightKeywords = ['[Work]']
        return highlightKeywords.find(keyword => content.includes(keyword))
    }
    const getNote = async () => {
        isLoading = true
        try {
            const server = import.meta.env.VITE_SERVER
            const {data} = await axios.get<NoteHeader[]>(server + '/notes')
            text = data
                .map(n => ({...n, rank: toRank(n.content)}))
                .sort((a, b) => (a.content > b.content ? 1 : -1))
                .sort((a, b) => b.rank - a.rank)
                .map(n => {
                    const emoji = n.emoji ? `<span class="emoji">${n.emoji}</span>` : ''
                    const highlight = shouldHighlight(n.content) ? 'header-highlight' : ''
                    return `<h2 class="org-h2 ${highlight}">${emoji}${n.content}</h2>`
                })
                .join('\n')
        } catch (err) {
            console.error('Error getting daily note', err)
        } finally {
            isLoading = false
        }
    }

    let interval = []
    onMount(async () => {
        getNote()
        interval = [
            {
                fn: () => {
                    getNote()
                },
                duration: import.meta.env.VITE_NOTE_REFRESH_INTERVAL * 1000,
            },
        ].map(i => setInterval(i.fn, i.duration))
    })

    onDestroy(() => {
        interval.forEach(i => clearInterval(i))
    })
</script>

<div class="container">
    <div class="clock" style:background-color={isLoading ? 'var(--background)' : 'var(--darker)'}>
        <Clock />
    </div>
    <div class="text">{@html text}</div>
    <div class="buttons">
        <button on:click={() => getNote()}>Load</button>
    </div>
</div>

<style lang="scss">
    .container {
        display: grid;
        grid-template-columns: 2.5fr 1fr;
    }
    .clock {
        position: fixed;
        top: 0.5rem;
        right: 0.5rem;
        width: calc(100vw / 3.6);
    }
    .text {
        white-space: pre;
        white-space: break-spaces;
        font-size: 1.2rem;
        padding: 5rem 0.5rem 20rem 1rem;
        overflow-y: scroll;
    }
    .buttons {
        position: fixed;
        bottom: 0.5rem;
        right: 0.5rem;
        display: grid;
        grid-template-columns: 1fr;
        grid-gap: 1rem;
    }
</style>

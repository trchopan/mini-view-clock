<script lang="ts">
    import {onDestroy, onMount} from 'svelte'
    import About from './About.svelte'
    import ClockView from './ClockView.svelte'
    import MenuIcon from './MenuIcon.svelte'
    import {currentView} from './store'
    import {View} from './types'

    const routes = [
        {component: ClockView, text: View.Clock},
        {component: About, text: View.About},
    ]

    let showMenu = false

    const toggleMenu = () => {
        showMenu = !showMenu
    }

    const selectView = (view: View) => {
        currentView.set(view)
        showMenu = false
    }

    const reload = () => {
        window.location.reload()
    }

    let isFullscreen = false // Declare a state variable for fullscreen

    // Function to update the isFullscreen state based on document.fullscreenElement
    const updateFullscreenState = () => {
        isFullscreen = !!document.fullscreenElement
    }

    // Event handler for toggling fullscreen
    const toggleFullscreen = () => {
        if (!document.fullscreenElement) {
            document.documentElement.requestFullscreen().catch(err => {
                console.error(`Error attempting to enable fullscreen: ${err.message} (${err.name})`)
            })
        } else {
            document.exitFullscreen()
        }
    }

    // Listen for fullscreenchange events
    onMount(() => {
        document.addEventListener('fullscreenchange', updateFullscreenState)
        // Initialize state
        updateFullscreenState()
    })

    onDestroy(() => {
        document.removeEventListener('fullscreenchange', updateFullscreenState)
    })
</script>

<main>
    <div class="view-container">
        {#each routes as route}
            {#if $currentView == route.text}
                <svelte:component this={route.component} />
            {/if}
        {/each}
    </div>
    <nav>
        <button on:click={() => toggleMenu()}>
            <MenuIcon />
        </button>
        {#if showMenu}
            <ul>
                {#each routes as route}
                    <li>
                        <button on:click={() => selectView(route.text)}>
                            {route.text}
                        </button>
                    </li>
                {/each}
                <li>
                    <button on:click={() => reload()}>Reload</button>
                </li>

                <li>
                    <button on:click={() => toggleFullscreen()}>
                        {isFullscreen ? 'Exit Fullscreen' : 'Fullscreen'}
                    </button>
                </li>
            </ul>
        {/if}
    </nav>
</main>

<style>
    main {
        width: 100vw;
        height: 100dvh;
    }
    .view-container {
        height: 100%;
        width: 100%;
    }
    nav {
        position: fixed;
        top: 0.1rem;
        right: 0.1rem;
    }
    ul {
        padding: 0.5rem;
        margin: 0;
        color: var(--accent);
        background: var(--darker);
        z-index: 9999;
    }
    ul > li {
        list-style: none;
        padding: 0.5rem;
        border-bottom: solid 1px var(--primary);
    }
</style>

<script lang="ts">
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

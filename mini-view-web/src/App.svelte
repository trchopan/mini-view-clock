<script lang="ts">
  import ClockView from './ClockView.svelte'
  import MenuIcon from './MenuIcon.svelte'
  import NoteView from './NoteView.svelte'
  import {currentView} from './store'
  import {View} from './types'

  let showMenu = false

  const toggleMenu = () => {
    showMenu = !showMenu
  }

  const selectView = (view: View) => {
    currentView.set(view)
    showMenu = false
  }
</script>

<main>
  <div class="view-container">
    {#if $currentView == View.Clock}
      <ClockView />
    {:else if $currentView == View.Note}
      <NoteView />
    {/if}
  </div>
  <nav>
    <button on:click={() => toggleMenu()}>
      <MenuIcon />
    </button>
    {#if showMenu}
      <ul>
        <li on:click={() => selectView(View.Clock)}>Clock</li>
        <li on:click={() => selectView(View.Note)}>Note</li>
      </ul>
    {/if}
  </nav>
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
  }
  .view-container {
    height: 100%;
    width: 100%;
  }
  nav {
    position: fixed;
    top: 0.1rem;
    left: 0.1rem;
  }
  ul {
    padding: 0.5rem;
    margin: 0;
    color: var(--accent);
    background: var(--darker);
  }
  ul > li {
    list-style: none;
    padding: 0.5rem;
    border-bottom: solid 1px var(--primary);
  }
</style>

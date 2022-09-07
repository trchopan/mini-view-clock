<script lang="ts">
  import Clock from './Clock.svelte'
  import * as org from 'org'
  import axios from 'axios'
  import {onMount} from 'svelte'

  let text =""
  const getNote = async () => {
    const {data} = await axios.get('https://mini-view.firebaseio.com/note.json')
    console.log(data)
  }

  onMount(() => {
    getNote()
  })

  let parser = new org.Parser()
  $: orgHTMLDocument = () => {
    let _text = text
      .split('\n')
      .map(x => x.replace(/^:.*$/, ''))
      .join('\n')
      .replaceAll(/\n\s*\n/g, '\n')
    let orgDocument = parser.parse(_text)
    return orgDocument.convert(org.ConverterHTML, {
      headerOffset: 1,
      exportFromLineNumber: false,
      suppressSubScriptHandling: false,
      suppressAutoLink: false,
      htmlClassPrefix: 'org-',
    })
  }
</script>

<div class="container">
  <div class="clock">
    <Clock />
  </div>
  <div class="text">{@html orgHTMLDocument()?.contentHTML}</div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 3.5fr 1fr;
  }
  .clock {
    position: fixed;
    top: 0.5rem;
    right: 0.5rem;
    width: 18rem;
    background-color: var(--darker);
  }
  .text {
    white-space: break-spaces;
    font-size: 1.2rem;
    padding: 1rem 1rem;
    overflow-y: scroll;
  }
</style>

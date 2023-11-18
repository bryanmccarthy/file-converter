<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  let echo_text = ''
  let response = ''

  async function echo() {
    response = await invoke('echo_cmd', { text: echo_text })
  }
</script>

<main class="container">
  <input type="text" bind:value={echo_text} />
  <button on:click={echo}>Echo</button>

  <p>{response}</p>
</main>

<style>
</style>
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  let echo_text = ''
  let echo_response = ''
  let file_path = ''
  let output_path = ''
  let convert_response = ''

  async function echo() {
    echo_response = await invoke('echo_cmd', { text: echo_text })
  }

  async function convert() {
    convert_response = await invoke('convert_cmd', { file: file_path, output: output_path })
  }
</script>

<main class="container">
  <input type="text" bind:value={echo_text} />
  <button on:click={echo}>Echo</button>

  <p>{echo_response}</p>

  <input type="text" bind:value={file_path} />
  <input type="text" bind:value={output_path} />
  <button on:click={convert}>Convert</button>

  <p>{convert_response}</p>
</main>

<style>
</style>
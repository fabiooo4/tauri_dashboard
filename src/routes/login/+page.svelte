<script lang="ts">
import {invoke} from '@tauri-apps/api/core';

let username: string = $state("");
let password: string = $state("");

let res: string = $state("");

async function login(event: Event) {
  event.preventDefault()

  res = await invoke("login", {username, password})
} 
</script>

<main>
  <h1>Login</h1>
  <form onsubmit={login}>
    <input type="text" placeholder="Username" bind:value={username}>
    <input type="password" placeholder="Password" bind:value={password}>
    <button type="submit">Login</button>
  </form>

  <p>{res}</p>
</main>

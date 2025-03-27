<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let username: string = $state("");
  let password: string = $state("");

  let res: string = $state("");

  async function register(event: Event) {
    event.preventDefault();

    await invoke("register", { username, password })
      .then((_) => {
        res = "Registered successfully";
        username = "";
        password = "";
      })
      .catch((err) => (res = err));
  }
</script>

<main>
  <h1>Register</h1>
  <form onsubmit={register}>
    <input type="text" placeholder="Username" bind:value={username} />
    <input type="password" placeholder="Password" bind:value={password} />
    <button type="submit">Register</button>
  </form>

  <p>{res}</p>
</main>

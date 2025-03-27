<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { userSchema } from "$lib/types/schemas";

  let username: string = $state("");
  let password: string = $state("");

  let user: User = $state(undefined);

  let message: string = $state("");

  async function login(event: Event) {
    event.preventDefault();

    await invoke("login", { username, password })
      .then((usr) => {
        username = "";
        password = "";

        user = userSchema.parse(usr);

        message = "Login successful";
      })
      .catch(() => (message = "Wrong username or password"));
  }
</script>

<main>
  <h1>Login</h1>
  <form onsubmit={login}>
    <input type="text" placeholder="Username" bind:value={username} />
    <input type="password" placeholder="Password" bind:value={password} />
    <button type="submit">Login</button>
  </form>

  <p>{message}</p>

  {#if user !== undefined}
    <a href="/dashboard">Go to dashboard</a>
  {/if}

  <p>{JSON.stringify(user, null, 2)}</p>
</main>

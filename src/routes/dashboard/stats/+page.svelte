<script lang="ts">
  import UsersTable from "$lib/components/UsersTable.svelte";
  import { userSchema } from "$lib/types/schemas";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let { data } = $props();

  let currentUser = userSchema.parse(data.user);

  let users: Array<User> = $state([]);

  onMount(async () => {
    users = await invoke("get_users");

    listen("updated-users", async () => (users = await invoke("get_users")));
  });
</script>

<main class="p-4">
  {#if currentUser.is_admin}
    <h1 class="text-2xl mb-2">Promote to admin</h1>

    <UsersTable {users} />
  {:else}
    <h1 class="text-2xl mb-2">User stats</h1>
  {/if}
</main>

<script lang="ts">
  import UsersTable from "$lib/components/UsersTable.svelte";
  import { userSchema } from "$lib/types/schemas";
  import { invoke } from "@tauri-apps/api/core";

  let { data } = $props();

  let currentUser = userSchema.parse(data.user);

  let users: Promise<Array<User>> = invoke("get_users");
</script>

<main class="p-4">
  {#if currentUser.is_admin}
    <h1 class="text-2xl mb-2">Promote to admin</h1>

    {#await users then users: Array<User>}
      <UsersTable {users} />
    {/await}
  {:else}
    <h1 class="text-2xl mb-2">User stats</h1>
  {/if}
</main>

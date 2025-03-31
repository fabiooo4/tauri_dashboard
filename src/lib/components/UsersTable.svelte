<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Button from "./ui/button/button.svelte";

  let { users }: { users: Array<User> } = $props();

  let mess = $state("");
  async function makeAdmin(user: User) {
    invoke("make_admin", { user })
      .then(() => (mess = "admin"))
      .catch((e) => (mess = "ERROR: " + e));
  }
</script>

<div class="rounded-xl">
  {#each users as user}
    {#if !user.is_admin}
      <div class="flex flex-row even:bg-secondary odd:bg-transparent text-center items-center">
        <Button
          variant="default"
          class="text-destructive-foreground p-3.5"
          onclick={() => makeAdmin(user)}>↑</Button
        >
        <p class="px-3 py-2">{user.username}</p>
      </div>
    {/if}
  {/each}
  <p>{mess}</p>
</div>

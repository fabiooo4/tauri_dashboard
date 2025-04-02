<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Button from "./ui/button/button.svelte";
  import { toast } from "svelte-sonner";
  import { userSchema } from "$lib/types/schemas";

  let { users }: { users: Array<User> } = $props();

  async function makeAdmin(user: User) {
    await invoke("toggle_admin", { user })
      .then((admin) =>
        toast.success("Promoted " + user.username, {
          action: {
            label: "Undo",
            onClick: async () =>
              await invoke("toggle_admin", { user: userSchema.parse(admin) }),
          },
        }),
      )
      .catch((e) =>
        toast.error("Failed to promote " + user.username, {
          description: e,
        }),
      );
  }
</script>

<div class="rounded-xl">
  {#each users as user}
    {#if !user.is_admin}
      <div
        class="flex flex-row even:bg-secondary odd:bg-transparent text-center items-center"
      >
        <Button
          variant="default"
          class="text-destructive-foreground p-3.5"
          onclick={() => makeAdmin(user)}>↑</Button
        >
        <p class="px-3 py-2">{user.username}</p>
      </div>
    {/if}
  {/each}
</div>

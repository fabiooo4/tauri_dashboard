<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import { userSchema } from "$lib/types/schemas.js";

  let { data } = $props();

  let currentUser: User = userSchema.parse(data.user);

  let cards: Array<{
    title: string;
    subtitle: string;
    button: string;
    href?: string;
  }> = [
    {
      title: "Stats Overview",
      subtitle: "This card could contain stats or charts.",
      button: "Details",
      href: "/dashboard/stats",
    },
    {
      title: "Recent Activity",
      subtitle: "This card could contain recent user activity.",
      button: "View All",
      href: undefined,
    },
    {
      title: "Notifications",
      subtitle: "This card could contain user notifications.",
      button: "Clear All",
      href: undefined,
    },
  ];
</script>

<main class="flex flex-col p-4">
  <h1 class="text-2xl mb-4">Dashboard</h1>

  <p class="mb-2">Welcome to your dashboard, {currentUser.username}</p>
  <p>This is a protected dashboard page. Only logged in users can see this.</p>

  <div class="flex flex-row justify-evenly items-center my-6 space-x-4">
    {#each cards as card}
      <Card {...card} />
    {/each}
  </div>
</main>

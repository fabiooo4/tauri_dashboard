<script lang="ts">
  import Navbar from "$lib/components/Navbar.svelte";

  let paths: Array<{ path: string; name: string }> = [
    { path: "/", name: "Home" },
    { path: "/login", name: "Login" },
    { path: "/register", name: "Home" },
  ];

  let { children } = $props();
</script>

<div class="h-svh">
  <Navbar {paths} />

  {@render children()}
</div>

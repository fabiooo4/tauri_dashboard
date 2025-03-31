<script lang="ts">
  import { logout } from "$lib/auth";
  import Navbar from "$lib/components/Navbar.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let paths: Array<{ path: string; name: string; callback: any }> = $state([
    { path: "/", name: "Home", callback: null },
    { path: "/login", name: "Login", callback: null },
    { path: "/register", name: "Register", callback: null },
  ]);

  let unlistenLogin: UnlistenFn;
  let unlistenLogout: UnlistenFn;

  onMount(async () => {
    unlistenLogin = await listen("logged-in", () => {
      // If the user is authenticated
      paths = [
        { path: "/", name: "Home", callback: null },
        { path: "/dashboard", name: "Dashboard", callback: null },
        { path: "/dashboard/profile", name: "Profile", callback: null },
        { path: "", name: "Logout", callback: logout },
      ];
    });

    unlistenLogout = await listen("logged-out", () => {
      // If the user is not authenticated
      paths = [
        { path: "/", name: "Home", callback: null },
        { path: "/login", name: "Login", callback: null },
        { path: "/register", name: "Register", callback: null },
      ];
    });
  });

  onDestroy(() => {
    unlistenLogin;
    unlistenLogout;
  });

  let { children } = $props();
</script>

<div class="h-svh">
  <Navbar {paths} />

  {@render children()}
</div>

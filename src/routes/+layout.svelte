<script lang="ts">
  import { logout } from "$lib/auth";
  import { getAuthenticatedUser } from "$lib/auth";
  import Navbar from "$lib/components/Navbar.svelte";

  let user = $state();

  getAuthenticatedUser()
    .then((u) => (user = u))
    .catch(() => (user = null));

  let paths: Array<{ path: string; name: string; callback: any }> = $state([]);

  $effect(() => {
    if (user != null) {
      paths = [
        { path: "/", name: "Home", callback: null },
        { path: "/dashboard/profile", name: "Profile", callback: null },
        { path: "", name: "Logout", callback: logout },
      ];
    } else {
      paths = [
        { path: "/", name: "Home", callback: null },
        { path: "/login", name: "Login", callback: null },
        { path: "/register", name: "Register", callback: null },
      ];
    }
  });

  let { children } = $props();
</script>

<div class="h-svh">
  <Navbar {paths} />

  {@render children()}
</div>

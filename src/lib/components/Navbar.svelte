<script lang="ts">
  import "../../app.css";
  import { page } from "$app/state";

  let { paths } = $props();
</script>

<nav
  class="flex h-16 items-center justify-between bg-primary px-4 font-bold
    text-primary-foreground cursor-default"
>
  <div class="cursor-auto">Dash app</div>

  <div class="space-x-2 cursor-default">
    {#each paths as { path, name, callback }}
      <a
        href={path}
        aria-current={page.url.pathname === "/"}
        onclick={callback}
        class="hover:bg-primary-foreground hover:text-primary rounded p-2
          hover:transition-all duration-100
          {page.url.pathname === path
          ? 'bg-primary-foreground text-primary'
          : ''}"
      >
        {name}
      </a>
    {/each}
  </div>
</nav>

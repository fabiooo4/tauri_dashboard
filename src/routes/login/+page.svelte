<script lang="ts">
  import LoginForm from "./login-form.svelte";
  let { data } = $props();
</script>

<main class="py-4 mx-[10%] sm:mx-[30%] lg:mx-[35%] flex flex-col justify-center">
  <h1 class="text-3xl font-bold mb-3">Login</h1>

  <LoginForm data={data.form} />
</main>

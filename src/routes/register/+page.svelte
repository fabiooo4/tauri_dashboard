<script lang="ts">
  import RegisterForm from "$lib/components/RegisterForm.svelte";

  let { data } = $props();
</script>

<main
  class="py-4 mx-[10%] sm:mx-[30%] lg:mx-[35%] flex flex-col justify-center"
>
  <h1 class="text-3xl font-bold mb-3">Register</h1>

  <RegisterForm data={data.form} />
</main>

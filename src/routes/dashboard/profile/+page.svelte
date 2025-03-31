<script lang="ts">
    import { userSchema } from '$lib/types/schemas';

  let { data } = $props();
  let currentUser = userSchema.parse(data.user);
</script>

<main class="p-4">
  <h1 class="text-3xl font-bold">{currentUser.username}</h1>
  <p>Password: {currentUser.password}</p>
</main>

<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<nav>
  Dash app
  <a href="/">Home</a>
  <a href="/login">Login</a>
  <a href="/register">Register</a>
</nav>

{@render children()}

<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<nav>
  Dash app
  <a href="/">Home</a>
  <a href="/">Login</a>
  <a href="/">Register</a>
</nav>

{@render children()}

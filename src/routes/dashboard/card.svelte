<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
  import * as Card from "$lib/components/ui/card";

  let {
    title,
    subtitle,
    button,
  }: { title: string; subtitle: string; button: string } = $props();
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>{title}</Card.Title>
    <Card.Description>{subtitle}</Card.Description>
  </Card.Header>
  <Card.Content>
    <!-- <p>Card Content</p> -->
  </Card.Content>
  <Card.Footer>
    <Button class="w-full">{button}</Button>
  </Card.Footer>
</Card.Root>

<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
  import * as Card from "$lib/components/ui/card";

  let {
    title,
    subtitle,
    button,
    href,
  }: { title: string; subtitle: string; button: string; href?: string } = $props();
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
    <Button class="w-full" {href}>{button}</Button>
  </Card.Footer>
</Card.Root>

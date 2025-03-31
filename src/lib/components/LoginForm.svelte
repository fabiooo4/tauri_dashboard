<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";
  import { userFormSchema } from "$lib/types/schemas";
  import { invoke } from "@tauri-apps/api/core";
  import { type SuperValidated, superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { goto } from "$app/navigation";

  let { data }: { data: SuperValidated<UserForm> } = $props();

  const form = superForm(data, {
    validators: zodClient(userFormSchema),
    dataType: "json",
  });

  const { form: formData } = form;

  let errorMsg = $state("");

  async function login(event: Event) {
    event.preventDefault();

    console.log($formData)

    await invoke("login", {
      username: $formData.username,
      password: $formData.password,
    })
      .then(() => {
        errorMsg = "";
        goto("/dashboard");
      })
      .catch((err) => (errorMsg = err));
  }
</script>

<p class="text-destructive font-bold">{errorMsg}</p>
<form onsubmit={login}>
  <Form.Field {form} name="username">
    <Form.Control let:attrs>
      <Form.Label>Username</Form.Label>
      <Input {...attrs} bind:value={$formData.username} />
    </Form.Control>
    <!-- <Form.Description>This is your public display name.</Form.Description> -->
    <Form.FieldErrors />
  </Form.Field>
  <Form.Field {form} name="password">
    <Form.Control let:attrs>
      <Form.Label>Password</Form.Label>
      <Input {...attrs} type="password" bind:value={$formData.password} />
    </Form.Control>
    <!-- <Form.Description>This is your private password.</Form.Description> -->
    <Form.FieldErrors />
  </Form.Field>
  <Form.Button>Login</Form.Button>
</form>

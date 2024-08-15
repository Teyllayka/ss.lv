<script lang="ts">
  import { enhance } from "$app/forms";
  export let form;
  import InputField from "$lib/components/InputField.svelte";
</script>

<form method="POST" use:enhance>
  <InputField
    name="email"
    type="email"
    placeholder="email"
    errors={form?.errors || []}
    value={form?.email}
  />

  <InputField
    name="password"
    type="password"
    placeholder="password"
    errors={form?.errors || []}
  />

  <button type="submit">Login</button>
</form>

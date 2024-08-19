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
    value={form?.data.email}
  />

  <InputField
    name="password"
    type="password"
    placeholder="password"
    errors={form?.errors || []}
  />

  <InputField
    name="repeatPassword"
    type="password"
    placeholder="repeatPassword"
    errors={form?.errors || []}
  />

  <InputField
    name="name"
    type="text"
    placeholder="name"
    errors={form?.errors || []}
  />

  <InputField
    name="surname"
    type="text"
    placeholder="surname"
    errors={form?.errors || []}
  />

  <InputField
    name="phone"
    type="text"
    placeholder="phone"
    errors={form?.errors || []}
  />

  <InputField
    name="image"
    type="text"
    placeholder="image"
    errors={form?.errors || []}
  />

  <button type="submit">Register</button>
</form>

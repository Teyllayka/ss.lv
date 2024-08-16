<script lang="ts">
  import { enhance } from "$app/forms";

  let inputValue = "";

  $: {
    fetch(
      `https://api.geoapify.com/v1/geocode/autocomplete?text=${inputValue}&apiKey=76e56f7178e34d1f90b702904b22e1e4`,
      {
        method: "GET",
      }
    )
      .then((response) => response.json())
      .then((result) => console.log(result))
      .catch((error) => console.log("error", error));
  }
</script>

<form method="POST" use:enhance>
  <input type="text" name="" id="" bind:value={inputValue} />

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

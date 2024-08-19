<script lang="ts">
  import { enhance } from "$app/forms";
  import InputField from "$lib/components/InputField.svelte";
  export let form;


  let inputValue = "";
  let selectedCategory = ""; 

  const categories = {
    cars: ["fuelType", "assemblyYear", "model", "brand"],
  };

  $: dynamicFields =
    categories[selectedCategory as keyof typeof categories] || [];

  // $: {
  //   fetch(
  //     `https://api.geoapify.com/v1/geocode/autocomplete?text=${inputValue}&apiKey=76e56f7178e34d1f90b702904b22e1e4`,
  //     {
  //       method: "GET",
  //     }
  //   )
  //     .then((response) => response.json())
  //     .then((result) => console.log(result))
  //     .catch((error) => console.log("error", error));
  // }
</script>

<form method="POST" use:enhance>
  {JSON.stringify(form)}
  <input type="text" bind:value={inputValue} />

  <InputField name="price" type="number" placeholder="price" value={form?.data.price}     errors={form?.errors || []}
  />
  <InputField name="title" type="text" placeholder="title"     errors={form?.errors || []}
  />
  <InputField name="description" type="textfield" placeholder="description"     errors={form?.errors || []}
  />

  <select bind:value={selectedCategory}>
    <option value="">Select a category</option>
    <option value="cars">Cars</option>
  </select>

  {#each dynamicFields as field}
    <InputField type="text" name={field} placeholder={field}     errors={form?.errors || []}
    />
  {/each}

  <button type="submit">Submit</button>
</form>

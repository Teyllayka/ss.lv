<script lang="ts">
  import { enhance } from "$app/forms";
  import InputField from "$lib/components/InputField.svelte";

  let inputValue = "";
  let selectedCategory = ""; // Track the selected category

  // Define categories and their corresponding fields
  const categories = {
    cars: ["fuelType", "assemblyYear", "model", "brand"],
    // Add other categories here
  };

  // Reactive statement to update dynamic fields based on selected category
  $: dynamicFields =
    categories[selectedCategory as keyof typeof categories] || [];

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
  <input type="text" bind:value={inputValue} />

  <InputField name="price" type="number" placeholder="price" />
  <InputField name="title" type="text" placeholder="title" />
  <InputField name="description" type="textfield" placeholder="description" />

  <!-- Updated select element -->
  <select bind:value={selectedCategory}>
    <option value="">Select a category</option>
    <option value="cars">Cars</option>
    <!-- Add other options here -->
  </select>

  <!-- Dynamically generated fields based on selected category -->
  {#each dynamicFields as field}
    <label>{field}</label>
    <input type="text" name={field} placeholder={field} />
  {/each}

  <button type="submit">Submit</button>
</form>

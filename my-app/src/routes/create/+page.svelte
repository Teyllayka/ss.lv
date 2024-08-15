<script>
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

<input type="text" name="" id="" bind:value={inputValue} />

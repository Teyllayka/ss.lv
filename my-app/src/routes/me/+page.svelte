<script lang="ts">
  import type { PageData } from "./$houdini";
  export let data: PageData;
  import Advert from "$lib/components/Advert.svelte";

  function verify() {

  }

  $: ({ me } = data);

  $: adverts = $me.data?.me.adverts || [];

  function logout() {
    fetch("/api/logout", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({}),
    })
      .then((res) => res.json())
      .then((data) => {
        if (data.status == 200) {
          window.location.href = "/";
        }
      });
  }
</script>

{#if $me.fetching}
  loading...
{:else if $me.errors}
  err...
{:else}
  {JSON.stringify($me.data)}

  {#if !$me?.data?.me.emailVerified}
    <h1>Your email is not verified</h1>
    <button on:click={() => verify()}>verify</button>
  {/if}

  <section class="adverts">
    {#each adverts as advert}
      <Advert
        title={advert.title}
        description={advert.description}
        date={advert.createdAt}
        location={advert.location}
        price={advert.price}
        oldPrice={advert.oldPrice}
      />
    {/each}
  </section>
{/if}

<button on:click={logout}>logout</button>

<style lang="scss">
  .adverts {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 42px 100px;
  }
</style>

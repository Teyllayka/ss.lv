<!-- <script lang="ts">
import type { PageData } from "./$houdini";
export let data: PageData;
import Advert from "$lib/components/Advert.svelte";

$: ({ Favorites } = data);

$: adverts = $Favorites.data?.getFavorites || [];

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

{#if $Favorites.fetching}
  loading...
{:else if $Favorites.errors}
  {JSON.stringify($Favorites.errors)}
  err...
{:else}
  {#each adverts as advert}
    <Advert
      advert={advert}
      userPage={false}
    />
  {/each}
{/if}

<button on:click={logout}>logout</button> -->

<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { Heart, Trash2, Loader } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import { formatDate } from "$lib/helpers";
  import Advert from "$lib/components/Advert.svelte";

  export let data: PageData;

  $: ({ Favorites } = data);

  $: favoritedAdverts = $Favorites.data?.getFavorites || [];

  function handleFavoriteChange(advertId: number, isFavorited: boolean) {
    if (!isFavorited) {
      // Remove the advert from the favoritedAdverts array
      favoritedAdverts = favoritedAdverts.filter((ad) => ad.id !== advertId);
    }
  }
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto">
    <h1
      class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center"
    >
      Recent Adverts
    </h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
      {#if $Favorites.fetching}
        loading...
      {:else if $Favorites.errors}
        err...
        {JSON.stringify($Favorites.errors)}
      {:else if $Favorites.data}
        {#each favoritedAdverts as advert (advert.id)}
          <Advert
            {advert}
            userPage={false}
            onFavoriteChange={handleFavoriteChange}
          />
        {/each}
      {/if}
    </div>
  </div>
</div>

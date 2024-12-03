<script lang="ts">
import type { PageData } from "./$houdini";
import Advert from "$lib/components/Advert.svelte";

export let data: PageData;

$: ({ Favorites } = data);

$: favoritedAdverts = $Favorites.data?.getFavorites || [];

function handleFavoriteChange(advertId: number, isFavorited: boolean) {
	if (!isFavorited) {
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
      Favorited Adverts
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

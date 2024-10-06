<script lang="ts">
import { onMount } from "svelte";
import { fade } from "svelte/transition";
import { Heart, Star, MapPin } from "lucide-svelte";
import { graphql } from "$houdini";
import { formatDate } from "$lib/helpers";
import Advert from "$lib/components/Advert.svelte";

const adverts = graphql(`
    query Adverts($offset: Int!) {
      getAdverts(limit: 10, offset: $offset) @paginate {
        id
        title
        price
        oldPrice
        location
        createdAt
        isFavorited
        user {
          id
          name
          surname
          rating
        }
      }
    }
  `);

let isLoggedIn = false;

onMount(async () => {
	await adverts.fetch({ variables: { offset: 0 } });
});
</script>

{JSON.stringify($adverts)}

<button on:click={() => adverts.loadNextPage}>
  load next
</button>


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
      {#if $adverts.fetching}
        loading...
      {:else if $adverts.errors}
        err...
        {JSON.stringify($adverts.errors)}
      {:else if $adverts.data}
        {#each $adverts.data.getAdverts as advert (advert.id)}
            <Advert advert={advert} userPage={false} />
        {/each}
      {/if}
     
    </div>
  </div>
</div>

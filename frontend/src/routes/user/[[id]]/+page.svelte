<script lang="ts">
import type { PageData } from "./$houdini";
export let data: PageData;
import Advert from "$lib/components/Advert.svelte";

$: ({ User } = data);
$: adverts = $User.data?.user.adverts || [];
</script>

{#if $User.fetching}
  loading...
{:else}
  {JSON.stringify($User.data)}

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
{/if}

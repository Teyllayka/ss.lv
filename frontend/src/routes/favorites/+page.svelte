<script lang="ts">
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

<button on:click={logout}>logout</button>

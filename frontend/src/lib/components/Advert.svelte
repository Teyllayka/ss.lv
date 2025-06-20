<script lang="ts">
  import { calculateDistance, formatDate } from "$lib/helpers";
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { Heart, Star, MapPin } from "lucide-svelte";
  import { AddFavoriteStore, RemoveFavoriteStore } from "$houdini";
  import ImageGallery from "./ImageGallery.svelte";
  import { getContext, onMount } from "svelte";
  import type { Writable } from "svelte/store";
  import { user } from "$lib/userStore";

  const locationStore = getContext<Writable<[number, number]>>("location");

  const addFavorite = new AddFavoriteStore();
  const removeFavorite = new RemoveFavoriteStore();

  interface Props {
    onFavoriteChange?: (advertId: number, isFavorited: boolean) => void;
    advert: any;
    userPage?: boolean;
  }

  let {
    onFavoriteChange = () => {},
    advert,
    userPage = false,
  }: Props = $props();

  let isFavorited = $derived(advert.isFavorited as boolean);

  let allPhotos = [advert.photoUrl, ...(advert.additionalPhotos || [])];

  let location = $state("");

  onMount(() => {
    fetch(`/api/reverse-geocode?lat=${advert.lat}&lon=${advert.lon}`)
      .then((response) => response.json())
      .then((data) => {
        location = data.location;
      })
      .catch((err) => console.error("Error with reverse geocoding:", err));
  });

  async function toggleSaveAdvert() {
    if (!$user.isLoggedIn) {
      goto("/login");
      return;
    }

    try {
      let res;
      if (isFavorited) {
        res = await removeFavorite.mutate({ advertId: advert.id });
      } else {
        res = await addFavorite.mutate({ advertId: advert.id });
      }

      isFavorited = !isFavorited;
      onFavoriteChange(advert.id, isFavorited);
    } catch (error) {
      console.error("Error toggling favorite:", error);
    }
  }

  let distance = $state(0);

  $effect(() => {
    distance = calculateDistance(
      [advert.lat, advert.lon],
      [$locationStore[0], $locationStore[1]],
    );
  });
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
  in:fade={{ duration: 300 }}
>
  <a class="relative cursor-pointer" href={`/advert/${advert.id}`}>
    <ImageGallery images={allPhotos} />
  </a>

  <div class="p-4">
    <div class="flex justify-between items-start mb-2">
      <h2
        class="text-lg font-semibold text-gray-900 dark:text-white truncate cursor-pointer"
        onclick={() => goto(`/advert/${advert.id}`)}
      >
        {advert.title}
      </h2>
      <button
        class="text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 transition-colors duration-300"
        onclick={toggleSaveAdvert}
      >
        <Heart
          size={20}
          fill={isFavorited ? "red" : "none"}
          color={isFavorited ? "red" : "currentColor"}
        />
      </button>
    </div>

    <div class="flex items-end mb-2">
      {#if advert.price !== advert.oldPrice}
        <p class="text-xl font-bold text-green-600 dark:text-green-400 mr-2">
          €{advert.price.toFixed(2)}
        </p>
        <p class="text-lg text-gray-500 dark:text-gray-400 line-through">
          ${advert.oldPrice.toFixed(2)}
        </p>
      {:else}
        <p class="text-xl font-bold text-gray-900 dark:text-white">
          €{advert.price.toFixed(2)}
        </p>
      {/if}
    </div>

    <div
      class="flex items-center mb-1 text-sm text-gray-600 dark:text-gray-300"
    >
      <MapPin size={16} class="mr-1" />
      <p class="flex flex-col items-start justify-start">
        <span class="truncate">{location}</span>
        {#if !userPage && $locationStore[0] != 0 && $locationStore[1] != 0}
          <span class="text-gray-500 dark:text-gray-400">{distance} km</span>
        {/if}
      </p>
    </div>

    <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
      {formatDate(advert.createdAt.toString())}
    </p>

    <div class="flex items-center justify-between">
      {#if !userPage}
        <a
          class="text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium"
          href={`/user/${advert.user.id}`}
        >
          {advert.user.name}
          {advert.user.surname}
        </a>
        <div class="flex items-center">
          <Star size={16} class="text-yellow-400 mr-1" />
          <span class="text-sm text-gray-500 dark:text-gray-400">
            {advert.user.rating.toFixed(1)}
          </span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  .relative {
    z-index: 20;
  }
</style>

<script lang="ts">
  import { formatDate } from "$lib/helpers";
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { Heart, Star, MapPin } from "lucide-svelte";
  import { user } from "$lib/userStore";

  let isLoggedIn = false;

  interface Props {
    onFavoriteChange?: (advertId: number, isFavorited: boolean) => void;
    advert: any;
    userPage: any;
  }

  let { onFavoriteChange = () => {}, advert, userPage }: Props = $props();

  let isFavorited = $state(advert.isFavorited);

  function handleImageScroll(event: any) {
    const container = event.currentTarget;
    if (!container) return;
    const containerWidth = container.offsetWidth;
    const mouseX = event.clientX - container.getBoundingClientRect().left;
    const scrollPercentage = mouseX / containerWidth;
    const maxScroll = container.scrollWidth - containerWidth;
    container.scrollLeft = maxScroll * scrollPercentage;
  }

  function toggleSaveAdvert(advert: any) {
    if (!$user.isLogedIn) {
      goto("/login");
    }

    fetch("/api/favorite", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        advertId: advert.id,
        isFavorited: !isFavorited,
      }),
    })
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        onFavoriteChange(advert.id, isFavorited);
      });

    isFavorited = !isFavorited;
  }
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
  in:fade={{ duration: 300 }}
>
  <div
    class="relative h-48 overflow-hidden cursor-pointer"
    onmousemove={(e) => handleImageScroll(e)}
    onclick={() => goto(`/advert/${advert.id}`)}
    role="img"
  ></div>
  <div class="p-4">
    <div class="flex justify-between items-start mb-2">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white truncate">
        {advert.title}
      </h2>
      <button
        class="text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 transition-colors duration-300"
        onclick={() => toggleSaveAdvert(advert)}
        title={isLoggedIn
          ? isFavorited
            ? "Remove from saved"
            : "Save for later"
          : "Log in to save adverts"}
      >
        <Heart
          size={20}
          fill={isFavorited ? "red" : "none"}
          color={isFavorited ? "red" : "currentColor"}
        />
      </button>
    </div>
    <div class="flex items-end mb-2">
      {#if advert.price != advert.oldPrice}
        <p class="text-xl font-bold text-green-600 dark:text-green-400 mr-2">
          ${advert.price.toFixed(2)}
        </p>
        <p class="text-lg text-gray-500 dark:text-gray-400 line-through">
          ${advert.oldPrice.toFixed(2)}
        </p>
      {:else}
        <p class="text-xl font-bold text-gray-900 dark:text-white">
          ${advert.price.toFixed(2)}
        </p>
      {/if}
    </div>
    <div
      class="flex items-center mb-1 text-sm text-gray-600 dark:text-gray-300"
    >
      <MapPin size={16} class="mr-1" />
      <span class="truncate">{advert.location}</span>
      {#if !userPage}<span class="ml-1 text-gray-500 dark:text-gray-400">
          2 km
        </span>{/if}
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

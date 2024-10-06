<script lang="ts">
  import { formatDate } from "$lib/helpers";
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { Heart, Star, MapPin } from "lucide-svelte";
  
  let isLoggedIn = false;
  
  export let advert;
  export let userPage;
  
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
    if (isLoggedIn) {
      advert.isFavorited = !advert.isSaved;
      console.log(
        advert.isSaved ? "Saving advert:" : "Unsaving advert:",
        advert.id,
      );
    } else {
      console.log("User needs to log in to save adverts");
    }
  }
  
  function navigateToUserProfile(userId: number) {
    goto(`/user/${userId}`);
  }
  
  function calculateDiscountPercentage(originalPrice: number, discountedPrice: number): number {
    return Math.round(((originalPrice - discountedPrice) / originalPrice) * 100);
  }
  </script>
  
  <div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
  in:fade={{ duration: 300 }}
  >
  <div
    class="relative h-48 overflow-hidden"
    on:mousemove={(e) => handleImageScroll(e)}
    role="img"
  >
    <!-- Image carousel code here -->
  </div>
  <div class="p-4">
    <div class="flex justify-between items-start mb-2">
      <h2
        class="text-lg font-semibold text-gray-900 dark:text-white truncate"
      >
        {advert.title}
      </h2>
      <button
        class="text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 transition-colors duration-300"
        on:click={() => toggleSaveAdvert(advert)}
        disabled={!isLoggedIn}
        title={isLoggedIn
          ? advert.isFavorited
            ? "Remove from saved"
            : "Save for later"
          : "Log in to save adverts"}
      >
        <Heart
          size={20}
          fill={advert.isFavorited ? "red-500 " : "none"}
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
        <!-- <p class="text-sm text-green-600 dark:text-green-400 ml-2">
          {calculateDiscountPercentage(advert.oldPrice, advert.price)}% off
        </p> -->
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
      {#if !userPage}<span class="ml-1 text-gray-500 dark:text-gray-400"> 2 km </span>{/if}
    </div>
    <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
      {formatDate(advert.createdAt.toString())}
    </p>
    <div class="flex items-center justify-between">
      {#if !userPage}
          <button
          class="text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium"
          on:click={() => navigateToUserProfile(advert.user.id)}
        >
          {advert.user.name}
          {advert.user.surname}
        </button>
        <div class="flex items-center">
          <Star size={16} class="text-yellow-400 mr-1" />
          {#if userPage}
            <!-- <span class="text-sm text-gray-500 dark:text-gray-400">
              {advert.user.rating.toFixed(1)}
            </span> -->
          {:else}
          <span class="text-sm text-gray-500 dark:text-gray-400">
            {advert.user.rating.toFixed(1)}
          </span>
          {/if}
        </div>
      {/if}
     
    </div>
  </div>
  </div>
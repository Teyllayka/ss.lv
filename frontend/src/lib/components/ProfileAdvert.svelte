<script>
  import { goto } from "$app/navigation";
  import { MapPin, Star } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import ImageGallery from "./ImageGallery.svelte";
  import { formatDate, renderStars } from "$lib/helpers";

  export let advert;
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden"
  in:fade
>
  <div
    class="relative cursor-pointer z-20"
    on:click={() => goto(`/advert/${advert.id}`)}
  >
    <ImageGallery
      images={[advert.photoUrl, ...(advert.additionalPhotos || [])]}
    />
  </div>

  <div class="p-4">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
      {advert.title}
    </h3>
    <div class="flex justify-between items-center">
      <p class="text-xl font-bold text-gray-900 dark:text-white mb-2">
        ${advert.price.toFixed(2)}
      </p>

      {#if advert.available}
        <span
          class="px-2 py-1 text-xs font-semibold text-green-800 bg-green-100 rounded-full"
          >Active</span
        >
      {:else}
        <span
          class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-200 rounded-full"
          >Sold</span
        >
      {/if}
    </div>
    <div class="flex items-center mb-2">
      <MapPin class="w-4 h-4 text-gray-500 dark:text-gray-400 mr-1" />
      <span class="text-sm text-gray-600 dark:text-gray-300">
        {advert.location}
      </span>
    </div>
    <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
      {formatDate(advert.createdAt.toString())}
    </p>
    {#if advert.review}
      <div>
        <div class="flex items-center">
          {#each renderStars(advert.review.rating) as star, index}
            <Star
              class={star.isFilled
                ? "text-yellow-400 fill-current"
                : "text-gray-300"}
              size="16"
            />
          {/each}

          <span class="ml-2 text-sm text-gray-600 dark:text-gray-400"
            >{advert.review.rating.toFixed(1)}</span
          >
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
          {advert.review.message}
        </p>
      </div>
    {/if}
  </div>
</div>

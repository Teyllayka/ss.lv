<script>
  import { formatDate, renderStars } from "$lib/helpers";
  import { Star } from "lucide-svelte";
  import { fade } from "svelte/transition";

  export let advert;
  export let userName = "";
  export let written = false;
</script>

<div
  class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden mb-4"
  in:fade
>
  <div class="p-4">
    <div class="flex flex-col md:flex-row">
      <div class="flex-grow">
        <div class="flex items-start mb-2">
          <div
            class="w-10 h-10 rounded-full bg-gray-300 mr-3 flex items-center justify-center"
          >
            <span class="text-xl font-semibold text-gray-700">
              {#if written}
                {userName?.[0] ?? ""}
              {:else}
                {advert.review.user?.name?.[0] ?? ""}
              {/if}
            </span>
          </div>
          <div>
            <h3 class="text-sm font-semibold">
              {#if written}
                {userName}
              {:else}
                {advert.review.user?.name}
              {/if}
            </h3>
            <div class="flex items-center">
              {#each renderStars(advert.review.rating) as star, index}
                <Star
                  class={star.isFilled
                    ? "text-yellow-400 fill-current"
                    : "text-gray-300"}
                  size="16"
                />
              {/each}
              <span class="ml-2 text-sm text-gray-500">
                {formatDate(advert.review.createdAt.toString())}
              </span>
            </div>
          </div>
        </div>
        <p class="text-sm text-gray-600 dark:text-gray-300">
          {advert.review.message}
        </p>
      </div>
      <div class="mt-4 md:mt-0 md:ml-4 flex-shrink-0">
        <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-2 w-full md:w-48">
          <img
            alt={advert.title}
            class="w-full h-24 object-cover mb-2 rounded"
          />
          <h4 class="text-sm font-semibold truncate">
            {advert.title}
          </h4>
          <p class="text-xs text-gray-500">
            ${advert.price.toFixed(2)}
          </p>
        </div>
      </div>
    </div>
  </div>
</div>

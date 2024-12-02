<script>
    import { goto } from "$app/navigation";
    import { MapPin } from "lucide-svelte";
    import { fade, fly } from "svelte/transition";
    import ImageGallery from "./ImageGallery.svelte";
    import { formatDate } from "$lib/helpers";

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
      images={[
        advert.photoUrl,
        ...(advert.additionalPhotos || []),
      ]}
    />
  </div>

  <div class="p-4">
    <h3
      class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
    >
      {advert.title}
    </h3>
    <div class="flex justify-between items-center">
      <p
        class="text-xl font-bold text-gray-900 dark:text-white mb-2"
      >
        ${advert.price.toFixed(2)}
      </p>
      <span
        class="px-2 py-1 text-xs font-semibold text-green-800 bg-green-100 rounded-full"
        >Active</span
      >
    </div>
    <div class="flex items-center mb-2">
      <MapPin
        class="w-4 h-4 text-gray-500 dark:text-gray-400 mr-1"
      />
      <span
        class="text-sm text-gray-600 dark:text-gray-300"
      >
        {advert.location}
      </span>
    </div>
    <p
      class="text-sm text-gray-500 dark:text-gray-400 mb-3"
    >
      {formatDate(advert.createdAt.toString())}
    </p>
  </div>
</div>
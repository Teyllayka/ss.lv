
<script lang="ts">
import { onMount } from "svelte";
import { fade } from "svelte/transition";
import {
	Star,
	Phone,
	Mail,
	MapPin,
	ChevronLeft,
	ChevronRight,
} from "lucide-svelte";
import type { PageData } from "./$houdini";
import Advertt from "$lib/components/Advert.svelte";
import { renderStars } from "$lib/helpers";
    import { goto } from "$app/navigation";
export let data: PageData;
$: ({ Advert } = data);
$: advert = $Advert.data?.advert;

let currentImageIndex = 0;

function nextImage() {
	currentImageIndex = (currentImageIndex + 1) % advert.images.length;
}

function prevImage() {
	currentImageIndex =
		(currentImageIndex - 1 + advert.images.length) % advert.images.length;
}

function formatDate(dateString: string) {
	return new Date(dateString).toLocaleDateString("en-US", {
		year: "numeric",
		month: "long",
		day: "numeric",
	});
}
</script>
  {#if $Advert.fetching}
    loading...
  {:else if $Advert.errors}
    err...
    {JSON.stringify($Advert.errors)}
  {:else if advert}
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-7xl mx-auto">
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden">
        <div class="md:flex">
          <div class="md:w-1/2 p-4">
            <div class="relative h-96">
              <!-- <img src={advert.images[currentImageIndex]} alt={advert.title} class="w-full h-full object-cover rounded-lg" /> -->
              <button on:click={prevImage} class="absolute left-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 rounded-full p-2 shadow-md">
                <ChevronLeft size={24} />
              </button>
              <button on:click={nextImage} class="absolute right-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 rounded-full p-2 shadow-md">
                <ChevronRight size={24} />
              </button>
            </div>
            <div class="flex mt-4 space-x-2 overflow-x-auto">
              <!-- {#each advert.images as image, index}
                <img 
                  src={image} 
                  alt={`${advert.title} - Image ${index + 1}`} 
                  class="w-20 h-20 object-cover rounded-md cursor-pointer" 
                  class:border-2={index === currentImageIndex}
                  class:border-blue-500={index === currentImageIndex}
                  on:click={() => currentImageIndex = index}
                />
              {/each} -->
            </div>
          </div>
          <div class="md:w-1/2 p-6">
            <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">{advert.title}</h1>
            <p class="text-xl font-semibold text-gray-900 dark:text-white mb-4">${advert.price.toFixed(2)}</p>
            <p class="text-gray-600 dark:text-gray-400 mb-4">{advert.description}</p>
            <div class="flex items-center mb-4">
              <MapPin class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
              <span class="text-gray-600 dark:text-gray-400">{advert.location}</span>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">Posted on {formatDate(advert.createdAt.toString())}</p>
            <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-4 mb-6">
              <div class="flex items-center mb-4">
                <!-- <img src={advert.user.avatarUrl} alt={advert.user.name} class="w-12 h-12 rounded-full mr-4" /> -->
                <div>
                    <h2 class="text-lg font-semibold text-gray-900 dark:text-white cursor-pointer hover:underline" on:click={() => goto(`user/${advert.user.id}`)}>
                        {advert.user.name}
                      </h2>
                  <div class="flex items-center">
                    {#each renderStars(advert.user.rating) as star, index}
                    <Star
                      class={star.isFilled
                        ? "text-yellow-400 fill-current"
                        : "text-gray-300"}
                      size="16"
                    />
                  {/each}
                    <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                      ({advert.user.rating.toFixed(1)})
                    </span>
                  </div>
                </div>
              </div>
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">Member since {formatDate(advert.user.createdAt.toString())}</p>
              <div class="flex space-x-4">
                <button class="flex-1 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out">
                  <Mail class="w-5 h-5 inline-block mr-2" />
                  Message
                </button>
                <button class="flex-1 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out">
                  <Phone class="w-5 h-5 inline-block mr-2" />
                  Call
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="mt-12">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Related Adverts</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <!-- {#each relatedAdverts as relatedAdvert}
            <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden">
              <img src={relatedAdvert.image} alt={relatedAdvert.title} class="w-full h-48 object-cover" />
              <div class="p-4">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">{relatedAdvert.title}</h3>
                <p class="text-gray-600 dark:text-gray-400">${relatedAdvert.price.toFixed(2)}</p>
              </div>
            </div>
          {/each} -->
        </div>
      </div>
    </div>
  </div>
  
  {/if}
 
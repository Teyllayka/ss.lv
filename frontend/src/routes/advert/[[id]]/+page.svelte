<script lang="ts">
  import {
    Star,
    Phone,
    Mail,
    MapPin,
    ChevronLeft,
    ChevronRight,
    Edit,
    Trash2,
  } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import { renderStars } from "$lib/helpers";
  export let data: PageData;
  $: ({ Advert } = data);
  $: advert = $Advert.data?.advert;

  let isEditMode = false;

  let editForm = {
    title: advert?.title || "",
    price: advert?.price || 0,
    description: advert?.description || "",
    location: advert?.location || "",
  };

  function toggleEditMode() {
    isEditMode = !isEditMode;
  }

  function handleSubmit() {
    console.log("Submitting updated advert:", editForm);
    isEditMode = false;
  }

  function handleDelete() {
    if (!advert) return;

    if (confirm("Are you sure you want to delete this advert?")) {
      console.log("Deleting advert:", advert.id);
    }
  }

  let images: string[] = [];

  // Safely add `images` property to advert if it exists
  $: if (advert) {
    images = [advert.photoUrl, ...(advert.additionalPhotos || [])];
  }

  let currentImageIndex = 0;

  function nextImage() {
    currentImageIndex = (currentImageIndex + 1) % images.length;
  }

  function prevImage() {
    currentImageIndex = (currentImageIndex - 1 + images.length) % images.length;
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
  <div class="flex justify-center items-center h-screen">
    <p class="text-xl text-gray-600 dark:text-gray-400">Loading...</p>
  </div>
{:else if $Advert.errors}
  <div class="flex justify-center items-center h-screen">
    <p class="text-xl text-red-600 dark:text-red-400">
      Error: {JSON.stringify($Advert.errors)}
    </p>
  </div>
{:else if advert}
  <div
    class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
  >
    <div class="max-w-7xl mx-auto">
      <div
        class="bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
      >
        <div class="md:flex">
          <div class="md:w-1/2 p-4">
            <div class="relative h-96">
              <img
                src={images[currentImageIndex]}
                alt={advert.title}
                class="w-full h-full object-cover rounded-lg"
              />
              <button
                on:click={prevImage}
                class="absolute left-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 rounded-full p-2 shadow-md"
              >
                <ChevronLeft size={24} />
              </button>
              <button
                on:click={nextImage}
                class="absolute right-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 rounded-full p-2 shadow-md"
              >
                <ChevronRight size={24} />
              </button>
            </div>
            <div class="flex mt-4 flex-wrap gap-2">
              {#each images as image, index}
                <img
                  src={image}
                  alt={`${advert.title} - Image ${index + 1}`}
                  class="w-20 h-20 object-cover rounded-md cursor-pointer"
                  class:border-2={index === currentImageIndex}
                  class:border-blue-500={index === currentImageIndex}
                  on:click={() => (currentImageIndex = index)}
                />
              {/each}
            </div>
          </div>
          <div class="md:w-1/2 p-6">
            {#if isEditMode}
              <form on:submit|preventDefault={handleSubmit} class="space-y-4">
                <div>
                  <label
                    for="title"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
                    >Title</label
                  >
                  <input
                    type="text"
                    id="title"
                    bind:value={editForm.title}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    required
                  />
                </div>
                <div>
                  <label
                    for="price"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
                    >Price</label
                  >
                  <input
                    type="number"
                    id="price"
                    bind:value={editForm.price}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    required
                  />
                </div>
                <div>
                  <label
                    for="description"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
                    >Description</label
                  >
                  <textarea
                    id="description"
                    bind:value={editForm.description}
                    rows="3"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    required
                  ></textarea>
                </div>
                <div>
                  <label
                    for="location"
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
                    >Location</label
                  >
                  <input
                    type="text"
                    id="location"
                    bind:value={editForm.location}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    required
                  />
                </div>
                <div class="flex justify-end space-x-2">
                  <button
                    type="button"
                    on:click={toggleEditMode}
                    class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-gray-700 bg-gray-200 hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    class="px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                  >
                    Save Changes
                  </button>
                </div>
              </form>
            {:else}
              <div class="flex justify-between items-start mb-4">
                <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
                  {advert.title}
                </h1>
                <div class="flex space-x-2">
                  <button
                    on:click={toggleEditMode}
                    class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
                  >
                    <Edit class="w-5 h-5 inline-block" />
                    Edit
                  </button>
                  <button
                    on:click={handleDelete}
                    class="flex items-center bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
                  >
                    <Trash2 class="w-5 h-5 inline-block mr-2" />
                    Delete
                  </button>
                </div>
              </div>
              <p
                class="text-xl font-semibold text-gray-900 dark:text-white mb-4"
              >
                ${advert.price.toFixed(2)}
              </p>
              <p class="text-gray-600 dark:text-gray-400 mb-4">
                {advert.description}
              </p>
              <div class="flex items-center mb-4">
                <MapPin class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
                <span class="text-gray-600 dark:text-gray-400">
                  {advert.location}
                </span>
              </div>
              <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
                Posted on {formatDate(advert.createdAt.toString())}
              </p>
              <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-4 mb-6">
                <div class="flex items-center mb-4">
                  <div>
                    <a
                      class="text-lg font-semibold text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 cursor-pointer hover:underline"
                      href={`/user/${advert.user.id}`}
                    >
                      {advert.user.name}
                    </a>
                    <div class="flex items-center">
                      {#each renderStars(advert.user.rating) as star, index}
                        <Star
                          class={star.isFilled
                            ? "text-yellow-400 fill-current"
                            : "text-gray-300"}
                          size="16"
                        />
                      {/each}
                      <span
                        class="ml-2 text-sm text-gray-600 dark:text-gray-400"
                      >
                        ({advert.user.rating.toFixed(1)})
                      </span>
                    </div>
                  </div>
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
                  Member since {formatDate(advert.user.createdAt.toString())}
                </p>
                <div class="flex space-x-4">
                  <button
                    class="flex-1 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
                  >
                    <Mail class="w-5 h-5 inline-block mr-2" />
                    Message
                  </button>
                  <button
                    class="flex-1 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
                  >
                    <Phone class="w-5 h-5 inline-block mr-2" />
                    Call
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

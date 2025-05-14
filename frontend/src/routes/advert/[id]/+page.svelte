<script lang="ts">
  import {
    Star,
    Phone,
    Mail,
    MapPin,
    ChevronLeft,
    ChevronRight,
    Edit,
    X,
    Camera,
    MessageSquare,
    CheckCircle,
    Tag,
  } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import { calculateDistance, renderStars } from "$lib/helpers";
  import { user } from "$lib/userStore";
  import { goto } from "$app/navigation";
  import { getContext, onDestroy, onMount } from "svelte";
  import type { Writable } from "svelte/store";
  import AddressField from "$lib/components/AddressField.svelte";
  import InputField from "$lib/components/InputField.svelte";
  import TextField from "$lib/components/TextField.svelte";
  import { enhance } from "$app/forms";
  import Advert from "$lib/components/Advert.svelte";
  import * as m from "$lib/paraglide/messages.js";

  export let data: PageData;
  export let form;

  $: advert = data?.advert?.data?.advert || null;
  $: gridCols = $user.isLoggedIn
    ? advert?.user.phone
      ? "grid-cols-3"
      : "grid-cols-2"
    : "grid-cols-2";

  $: ({ similarAdverts: similarAdvertsQuery } = data);

  let rating = 0;
  let reviewText = "";

  let isEditMode = false;
  let editForm = { title: "", description: "", price: 0 };
  let editMainPhoto = "";
  let editMainPhotoFile: File | null = null;
  let currentAdditionalPhotos: string[] = [];
  let removedPhotos: string[] = [];
  let newAdditionalPhotos: string[] = [];
  let newAdditionalPhotoFiles: File[] = [];
  let initialFormSet = false;
  $: if (advert && !initialFormSet) {
    editForm = {
      title: advert.title,
      description: advert.description,
      price: advert.price,
    };
    editMainPhoto = advert.photoUrl;
    currentAdditionalPhotos = [...(advert.additionalPhotos || [])];
    initialFormSet = true;
  }

  function toggleEditMode() {
    isEditMode = !isEditMode;
  }

  function handleEditMainPhotoChange(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (file) {
      editMainPhoto = URL.createObjectURL(file);
      editMainPhotoFile = file;
    }
  }

  function handleEditAdditionalPhotosChange(event: Event) {
    const files = (event.target as HTMLInputElement).files;
    if (files) {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        newAdditionalPhotos = [
          ...newAdditionalPhotos,
          URL.createObjectURL(file),
        ];
        newAdditionalPhotoFiles = [...newAdditionalPhotoFiles, file];
      }
    }
  }

  function removeCurrentAdditionalPhoto(index: number) {
    const photoUrl = currentAdditionalPhotos[index];
    removedPhotos.push(photoUrl);
    currentAdditionalPhotos = currentAdditionalPhotos.filter(
      (_, i) => i !== index,
    );
  }

  function removeNewAdditionalPhoto(index: number) {
    newAdditionalPhotos = newAdditionalPhotos.filter((_, i) => i !== index);
    newAdditionalPhotoFiles = newAdditionalPhotoFiles.filter(
      (_, i) => i !== index,
    );
  }

  let images: string[] = [];
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

  const locationStore = getContext<Writable<[number, number]>>("location");
  let location = "";
  let distance = 0;
  $: if (advert?.lat && advert?.lon && $locationStore[0] && $locationStore[1]) {
    distance = calculateDistance(
      [advert.lat, advert.lon],
      [$locationStore[0], $locationStore[1]],
    );
    fetch(`/api/reverse-geocode?lat=${advert.lat}&lon=${advert.lon}`)
      .then((response) => response.json())
      .then((data) => {
        location = data.location;
      })
      .catch((err) => console.error("Error with reverse geocoding:", err));
  }

  async function handleDelete() {
    if (!advert) return;
    const response = await fetch(`/advert/${advert.id}?/delete`, {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body: new URLSearchParams(),
    });
    if (response.ok) {
      goto("/");
    } else {
      console.error("Failed to delete the advert");
    }
  }

  let map: any;

  onMount(async () => {
    if (!advert?.lat || !advert?.lon) return;
    const L = (await import("leaflet")).default;

    map = L.map("map", {
      attributionControl: false,
    }).setView([advert.lat, advert.lon], 13);

    L.tileLayer("http://mt1.google.com/vt/lyrs=m&x={x}&y={y}&z={z}").addTo(map);
    L.marker([advert.lat, advert.lon])
      .addTo(map)
      .bindPopup(advert.title)
      .openPopup();
  });

  onDestroy(() => {
    if (map) {
      map.remove();
    }
  });

  let isIOS = false;
  onMount(() => {
    // @ts-ignore
    isIOS = /iPad|iPhone|iPod/.test(navigator.userAgent) && !window.MSStream;
  });
</script>

{#if advert}
  {#if isEditMode}
    <div
      class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
    >
      <div class="max-w-7xl mx-auto">
        <form
          method="post"
          action="?/edit"
          use:enhance={() => {
            return async ({ result, update }) => {
              if (result.type == "failure") {
                update();
              }

              // @ts-ignore
              if (result && result.data && result.type === "success") {
                isEditMode = false;

                if (!result?.data.advert) {
                  return;
                }

                if (!advert) {
                  return;
                }

                const updated = result.data.advert;

                advert = {
                  ...advert,
                  // @ts-ignore
                  title: updated.title,
                  // @ts-ignore
                  description: updated.description,
                  // @ts-ignore
                  price: updated.price,
                  // @ts-ignore
                  photoUrl: updated.photoUrl,
                  // @ts-ignore
                  additionalPhotos: updated.additionalPhotos,
                  // @ts-ignore
                  available: updated.available,
                  // @ts-ignore
                  lat: updated.lat,
                  // @ts-ignore
                  lon: updated.lon,
                };

                images = [advert.photoUrl, ...(advert.additionalPhotos || [])];
                editForm.title = advert.title;
                editForm.description = advert.description;
                editForm.price = advert.price;
                editMainPhoto = advert.photoUrl;
                currentAdditionalPhotos = [...(advert.additionalPhotos || [])];
              }
            };
          }}
          enctype="multipart/form-data"
          class="bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden space-y-6"
        >
          <div class="md:flex">
            <div class="md:w-1/2 p-4 space-y-6">
              <div class="relative">
                <label
                  for="newMainPhoto"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
                >
                  Main Photo
                </label>
                <div class="flex items-center justify-center w-full">
                  <label
                    for="newMainPhoto"
                    class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
                  >
                    {#if editMainPhoto}
                      <img
                        src={editMainPhoto}
                        class="w-full h-full object-contain rounded-lg"
                      />
                    {:else}
                      <div
                        class="flex flex-col items-center justify-center pt-5 pb-6"
                      >
                        <Camera class="w-10 h-10 mb-3 text-gray-400" />
                        <p
                          class="mb-2 text-sm text-gray-500 dark:text-gray-400"
                        >
                          <span class="font-semibold">Click to upload</span> or drag
                          and drop
                        </p>
                        <p class="text-xs text-gray-500 dark:text-gray-400">
                          {m.png_jpg()}
                        </p>
                      </div>
                    {/if}
                    <input
                      id="newMainPhoto"
                      name="newMainPhoto"
                      type="file"
                      accept="image/*"
                      on:change={handleEditMainPhotoChange}
                      class="hidden"
                    />
                  </label>
                </div>
                <input
                  type="hidden"
                  name="existingMainPhoto"
                  value={advert.photoUrl}
                />
              </div>

              <div class="relative">
                <label
                  for="newPhotos"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
                >
                  Additional Photos
                </label>
                <div class="flex items-center justify-center w-full">
                  <label
                    for="newPhotos"
                    class="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
                  >
                    <div
                      class="flex flex-col items-center justify-center pt-5 pb-6"
                    >
                      <Camera class="w-8 h-8 mb-3 text-gray-400" />
                      <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
                        {@html m.click_to_upload()}
                      </p>
                      <p class="text-xs text-gray-500 dark:text-gray-400">
                        {m.png_jpg()}
                      </p>
                    </div>
                    <input
                      id="newPhotos"
                      name="newPhotos"
                      type="file"
                      accept="image/*"
                      multiple
                      on:change={handleEditAdditionalPhotosChange}
                      class="hidden"
                    />
                  </label>
                </div>
                <div class="grid grid-cols-3 gap-4 mt-4">
                  {#each currentAdditionalPhotos as photo, index}
                    <div class="relative">
                      <img
                        src={photo}
                        alt={`Photo ${index + 1}`}
                        class="w-full h-32 object-contain rounded-lg"
                      />
                      <button
                        type="button"
                        on:click={() => removeCurrentAdditionalPhoto(index)}
                        class="absolute top-2 right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                      >
                        <X class="w-4 h-4" />
                      </button>
                    </div>
                  {/each}
                  {#each newAdditionalPhotos as photo, index}
                    <div class="relative">
                      <img
                        src={photo}
                        alt={`New Photo ${index + 1}`}
                        class="w-full h-32 object-contain rounded-lg"
                      />
                      <button
                        type="button"
                        on:click={() => removeNewAdditionalPhoto(index)}
                        class="absolute top-2 right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                      >
                        <X class="w-4 h-4" />
                      </button>
                    </div>
                  {/each}
                </div>
                {#each currentAdditionalPhotos as url}
                  <input type="hidden" name="existingPhotos" value={url} />
                {/each}
                {#each removedPhotos as url}
                  <input type="hidden" name="removedPhotos" value={url} />
                {/each}
              </div>
            </div>

            <div class="md:w-1/2 p-6 space-y-6">
              <InputField
                id="editTitle"
                name="title"
                type="text"
                placeholder={m.title()}
                bind:value={editForm.title}
                errors={form?.errors || []}
              />
              <InputField
                name="price"
                type="number"
                bind:value={editForm.price}
                placeholder={m.price()}
                errors={form?.errors || []}
              />
              <AddressField
                id="editAddress"
                name="location"
                placeholder={m.location()}
                bind:value={location}
                errors={form?.errors || []}
              />
              <TextField
                id="description"
                name="description"
                placeholder={m.description()}
                bind:value={editForm.description}
                errors={form?.errors || []}
              />

              <div class="flex justify-end space-x-4">
                <button
                  type="button"
                  on:click={toggleEditMode}
                  class="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                >
                  {m.cancel()}
                </button>
                <button
                  type="submit"
                  class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  {m.save_changes()}
                </button>
              </div>
            </div>
          </div>
        </form>
      </div>
    </div>
  {:else}
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
                  class="w-full h-full object-contain rounded-lg"
                />
                <button
                  on:click={prevImage}
                  class="absolute left-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 text-black dark:text-white rounded-full p-2 shadow-md"
                >
                  <ChevronLeft size={24} />
                </button>
                <button
                  on:click={nextImage}
                  class="absolute right-2 top-1/2 transform -translate-y-1/2 bg-white dark:bg-gray-800 text-black dark:text-white rounded-full p-2 shadow-md"
                >
                  <ChevronRight size={24} />
                </button>
              </div>
              <div class="flex mt-4 flex-wrap gap-2">
                {#each images as image, index}
                  <img
                    src={image}
                    alt={`${advert.title} - Image ${index + 1}`}
                    class="w-20 h-20 object-contain rounded-md cursor-pointer"
                    class:border-2={index === currentImageIndex}
                    class:border-blue-500={index === currentImageIndex}
                    on:click={() => (currentImageIndex = index)}
                  />
                {/each}
              </div>
            </div>
            <div class="md:w-1/2 p-6 space-y-6">
              <div class="flex justify-between items-start">
                <div class="flex flex-col">
                  <h1
                    class="text-3xl font-bold text-gray-900 dark:text-white overflow-hidden text-ellipsis whitespace-nowrap"
                  >
                    {advert.title}
                  </h1>
                  {#if !advert.available}
                    <div
                      class="inline-flex items-center mt-2 px-3 py-1 bg-red-100 text-red-800 rounded-full text-sm font-medium w-fit"
                    >
                      <CheckCircle class="w-4 h-4 mr-1" />
                      {m.sold()}
                    </div>
                  {:else}
                    <div
                      class="inline-flex items-center mt-2 px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium w-fit"
                    >
                      <Tag class="w-4 h-4 mr-1" />
                      {m.available()}
                    </div>
                  {/if}
                </div>
                <div class="flex space-x-4">
                  {#if $user.id == advert.user.id}
                    <button
                      on:click={toggleEditMode}
                      class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline"
                    >
                      <Edit class="w-5 h-5 inline-block" />
                      {m.edit()}
                    </button>
                  {/if}
                  {#if $user.role == "ADMIN" || $user.role == "MODERATOR" || $user.id == advert.user.id}
                    <button
                      on:click={handleDelete}
                      class="flex items-center bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline"
                    >
                      <X class="w-5 h-5 inline-block mr-2" />
                      {m.delete_()}
                    </button>
                  {/if}
                </div>
              </div>

              <p
                class="text-xl font-semibold text-gray-900 dark:text-white mb-4"
              >
                ${advert.price.toFixed(2)}
              </p>
              <p class="text-gray-600 dark:text-gray-400 mb-4 break-words">
                {advert.description}
              </p>
              {#if advert.specs && advert.specs.length > 0}
                <div>
                  <h2
                    class="text-lg font-medium text-gray-800 dark:text-gray-200 mb-2"
                  >
                    {m.specifications()}
                  </h2>
                  <dl class="grid grid-cols-1 sm:grid-cols-2 gap-x-4 gap-y-2">
                    {#each advert.specs as spec}
                      <div>
                        <dt
                          class="text-sm font-medium text-gray-500 dark:text-gray-400"
                        >
                          {spec.key.charAt(0).toUpperCase() + spec.key.slice(1)}
                        </dt>
                        <dd
                          class="mt-1 text-sm text-gray-900 dark:text-gray-100 break-words"
                        >
                          {spec.value}
                        </dd>
                      </div>
                    {/each}
                  </dl>
                </div>
              {/if}
              <div class="flex items-center">
                <MapPin class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
                <div class="flex flex-col">
                  <span class="truncate text-gray-800 dark:text-gray-200"
                    >{location}</span
                  >
                  {#if $locationStore[0] != 0 && $locationStore[1] != 0}
                    <span class="text-gray-500 dark:text-gray-400"
                      >{distance} {m.km()}</span
                    >
                  {/if}
                </div>
              </div>
              <a
                href={isIOS
                  ? `http://maps.apple.com/?daddr=${advert.lat},${advert.lon}`
                  : `https://www.google.com/maps/dir/?api=1&destination=${advert.lat},${advert.lon}`}
                target="_blank"
                rel="noopener noreferrer"
                class="mt-2 inline-flex items-center bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-md"
              >
                <MapPin class="w-5 h-5 inline-block mr-2" />
                {m.route()}
              </a>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                {m.posted_on()}
                {formatDate(advert.createdAt.toString())}
              </p>
              {#if $user.id !== advert.user.id}
                <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-4">
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
                    {m.member_since()}
                    {formatDate(advert.user.createdAt.toString())}
                  </p>
                  <div class={`grid ${gridCols} gap-4`}>
                    {#if $user.isLoggedIn}
                      <form
                        method="POST"
                        action="?/chat"
                        use:enhance
                        class="flex items-center justify-center"
                      >
                        <input
                          type="hidden"
                          name="advertId"
                          value={advert.id}
                        />
                        <button
                          type="submit"
                          class="w-full bg-purple-500 hover:bg-purple-600 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline flex flex-col sm:flex-row items-center justify-center"
                        >
                          <MessageSquare class="w-5 h-5 mb-1 sm:mb-0 sm:mr-2" />
                          <span>{m.chat()}</span>
                        </button>
                      </form>
                    {/if}

                    <button
                      on:click={() => {
                        window.location.href = `mailto:${advert?.user.email}`;
                      }}
                      title={advert.user.email}
                      class="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline flex flex-col sm:flex-row items-center justify-center"
                    >
                      <Mail class="w-5 h-5 mb-1 sm:mb-0 sm:mr-2" />
                      <span>{m.email()}</span>
                    </button>

                    {#if advert.user.phone}
                      <button
                        class="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:shadow-outline flex flex-col sm:flex-row items-center justify-center"
                      >
                        <Phone class="w-5 h-5 mb-1 sm:mb-0 sm:mr-2" />
                        <span>{m.call()}</span>
                      </button>
                    {/if}
                  </div>
                </div>
              {/if}
            </div>
          </div>
        </div>

        {#if advert.review}
          <div
            class="mt-8 bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
          >
            <div class="p-6">
              <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
                {m.review()}
              </h2>

              <div
                class={`p-4 rounded-lg ${advert.soldTo === $user.id ? "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800" : "bg-gray-50 dark:bg-gray-700"}`}
              >
                <div class="flex justify-between items-start mb-2">
                  <div class="flex items-center">
                    <a
                      href={`/user/${advert.review.user.id}`}
                      class="font-medium text-gray-900 dark:text-white hover:underline"
                    >
                      {advert.review.user.name}
                    </a>
                    {#if advert.soldTo === $user.id}
                      <span
                        class="ml-2 px-2 py-1 bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-200 text-xs font-medium rounded-full"
                      >
                        {m.your_review()}
                      </span>
                    {/if}
                  </div>
                  <div class="flex items-center">
                    {#each renderStars(advert.review.rating) as star}
                      <Star
                        class={star.isFilled
                          ? "text-yellow-400 fill-current"
                          : "text-gray-300"}
                        size="16"
                      />
                    {/each}
                    <span class="ml-1 text-sm text-gray-600 dark:text-gray-400">
                      ({advert.review.rating.toFixed(1)})
                    </span>
                  </div>
                </div>

                <p class="text-gray-700 dark:text-gray-300">
                  {advert.review.message}
                </p>

                <div class="mt-2 text-sm text-gray-500 dark:text-gray-400">
                  {formatDate(advert.review.createdAt.toString())}
                </div>
              </div>
            </div>
          </div>
        {/if}

        {#if advert?.lat && advert?.lon}
          <div class="mt-8">
            <div id="map" class="w-full h-96 rounded-lg"></div>
          </div>
        {/if}

        {#if advert.soldTo === $user.id && !advert.review}
          <form
            method="POST"
            action="?/review"
            use:enhance
            class="mt-8 bg-white dark:bg-gray-800 shadow rounded-lg p-6"
          >
            <h2
              class="text-2xl font-bold text-gray-900 dark:text-white overflow-hidden text-ellipsis whitespace-nowrap mb-4"
            >
              {m.leave_review()}
            </h2>

            <div class="flex items-center mb-8">
              {#each Array(5) as _, i}
                <button
                  type="button"
                  on:click={() => (rating = i + 1)}
                  class="mx-1 focus:outline-none"
                >
                  <Star
                    class={i < rating
                      ? "text-yellow-400 fill-current"
                      : "text-gray-300"}
                    size="24"
                  />
                </button>
              {/each}
              <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                {rating} / 5
              </span>
            </div>

            <input type="hidden" name="rating" value={rating} />

            <TextField
              id="reviewText"
              name="text"
              placeholder="Review"
              bind:value={reviewText}
            />

            <button
              type="submit"
              class="mt-4 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
            >
              {m.submit_review()}
            </button>
          </form>
        {/if}

        {#if similarAdvertsQuery && similarAdvertsQuery.data && similarAdvertsQuery.data.similarAdverts.length > 0}
          <div class="mt-8">
            <h2 class="text-2xl font-bold mb-4 dark:text-gray-200">
              {m.similar_adverts()}
            </h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
              {#each similarAdvertsQuery.data.similarAdverts as similar}
                <Advert advert={similar} userPage={false} />
              {/each}
            </div>
          </div>
        {:else}
          <p class="mt-8 text-gray-600">{m.no_similar_adverts()}</p>
        {/if}
      </div>
    </div>
  {/if}
{/if}

<style lang="scss">
  .relative {
    z-index: 20;
  }
</style>

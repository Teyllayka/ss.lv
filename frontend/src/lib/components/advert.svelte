<script lang="ts">
  import { formatDate } from "$lib/helpers";
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { Heart, Star, MapPin } from "lucide-svelte";
  import { user } from "$lib/userStore";
  import { AddFavoriteStore, RemoveFavoriteStore } from '$houdini';

  const addFavorite = new AddFavoriteStore();
  const removeFavorite = new RemoveFavoriteStore();

  let isLoggedIn = $state(false);

  interface Props {
    onFavoriteChange?: (advertId: number, isFavorited: boolean) => void;
    advert: any;
    userPage: any;
  }

  let { onFavoriteChange = () => {}, advert, userPage }: Props = $props();

  let isFavorited = $state(advert.isFavorited);

  let allPhotos = [advert.photoUrl, ...(advert.additionalPhotos || [])];

  console.log(allPhotos, advert.title);

  async function toggleSaveAdvert() {
    try {
      let res;
      if (isFavorited) {
        res = await removeFavorite.mutate({ advertId: advert.id });
      } else {
        res = await addFavorite.mutate({ advertId: advert.id });
      }
      console.log(res);

      isFavorited = !isFavorited;
      onFavoriteChange(advert.id, isFavorited);
    } catch (error) {
      console.error("Error toggling favorite:", error);
    }
  }

  $effect(() => {
    console.log(`Advert ${advert.id} is ${isFavorited ? 'favorited' : 'unfavorited'}`);
  });
</script>

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
  in:fade={{ duration: 300 }}
>
  <div class="relative cursor-pointer"
  onclick={() => goto(`/advert/${advert.id}`)}
  >
    <div class="thumbnail">
      {#each allPhotos as photo, index}
        <div class="li">
          <img src={photo} alt={`Advert image ${index + 1}`} />
        </div>
      {/each}
      <div class="last-pager">
        ugh +{allPhotos.length - 5} images
      </div>
      <div class="h">
        {#each allPhotos as _, index}
          <div class="hovers"></div>
        {/each}
      </div>
    </div>
  </div>

  <div class="p-4">
    <div class="flex justify-between items-start mb-2">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white truncate cursor-pointer" onclick={() => goto(`/advert/${advert.id}`)}>
        {advert.title}
      </h2>
      <button
        class="text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 transition-colors duration-300"
        onclick={toggleSaveAdvert}
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
      {#if advert.price !== advert.oldPrice}
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

    <div class="flex items-center mb-1 text-sm text-gray-600 dark:text-gray-300">
      <MapPin size={16} class="mr-1" />
      <span class="truncate">{advert.location}</span>
      {#if !userPage}<span class="ml-1 text-gray-500 dark:text-gray-400">2 km</span>{/if}
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
          {advert.user.name} {advert.user.surname}
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

  .thumbnail {
    position: relative;
    height: 200px;

    .li {
      position: absolute;
      inset: 0;
      z-index: -1;

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
    }

    .h {
      display: flex;
      opacity: 0;
      width:100%;
      height: 100%;

      &:has(.hovers:nth-of-type(2)) {
        &:hover {
          opacity: 1;
        }
      }

      .hovers {
        flex: 1 1;
        position: relative;
       

        &::after {
          position: absolute;
          content: "";
          height: 2px;
          bottom: 10px;
          inset-inline: 10px;
          background-color: gray;
        }

        &:hover::after {
          background-color: blue; /* Active color when hovered */
        }
      }
    }

    .li:not(:first-child) {
      opacity: 0;
    }

    &:has(.hovers:nth-of-type(1):hover) .li:nth-of-type(1) {
      opacity: 1;
    }
    &:has(.hovers:nth-of-type(2):hover) .li:nth-of-type(2) {
      opacity: 1;
    }
    &:has(.hovers:nth-of-type(3):hover) .li:nth-of-type(3) {
      opacity: 1;
    }

    .last-pager {
      display: grid;
      place-items: center;
      color: white;
      position: absolute;
      inset: 0;
      background: rgba(0, 0, 0, 0.5);
      opacity: 0;
      pointer-events: none;
    }

    &:has(.hovers:nth-of-type(6)) {
      &:has(.hovers:nth-of-type(5):hover) .last-pager {
        opacity: 1;
      }
    }

    .hovers:nth-of-type(n + 6) {
      display: none;
    }

    .li:nth-of-type(n + 6) {
      display: none;
    }
  }
</style>

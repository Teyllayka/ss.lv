<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { Heart, Star, MapPin } from "lucide-svelte";
  import { graphql } from "$houdini";
  import { formatDate } from "$lib/helpers";

  const advertss = graphql(`
    query Adverts($limit: Int!, $offset: Int!) {
      getAdverts(limit: $limit, offset: $offset) @paginate {
        id
        title
        price
        location
      }
    }
  `);

  let adverts = [];
  let isLoggedIn = false;

  onMount(async () => {
    adverts = await fetchAdverts();
  });

  function handleImageScroll(event, advert) {
    const container = event.currentTarget;
    const containerWidth = container.offsetWidth;
    const mouseX = event.clientX - container.getBoundingClientRect().left;
    const scrollPercentage = mouseX / containerWidth;
    const maxScroll = container.scrollWidth - containerWidth;
    container.scrollLeft = maxScroll * scrollPercentage;
  }

  function toggleSaveAdvert(advert) {
    if (isLoggedIn) {
      advert.isSaved = !advert.isSaved;
      console.log(
        advert.isSaved ? "Saving advert:" : "Unsaving advert:",
        advert.id
      );
    } else {
      console.log("User needs to log in to save adverts");
    }
  }

  function navigateToUserProfile(userId) {
    console.log("Navigating to user profile:", userId);
  }

  async function fetchAdverts() {
    // This is a placeholder. Replace with your actual API call
    return [
      {
        id: 1,
        title: "Vintage Leather Sofa",
        price: 599.99,
        location: "New York, NY",
        datePosted: "2023-05-15",
        images: [
          "https://letsenhance.io/static/8f5e523ee6b2479e26ecc91b9c25261e/1015f/MainAfter.jpg",
          "https://cdn.pixabay.com/photo/2021/08/25/20/42/field-6574455_1280.jpg",
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
        ],
        user: {
          id: 101,
          firstName: "John",
          lastName: "Doe",
          rating: 4.5,
        },
        isSaved: false,
      },
      {
        id: 2,
        title: "iPhone 12 Pro",
        price: 799.99,
        location: "Los Angeles, CA",
        datePosted: "2023-05-14",
        images: [
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
        ],
        user: {
          id: 102,
          firstName: "Jane",
          lastName: "Smith",
          rating: 4.8,
        },
        isSaved: true,
      },
      {
        id: 3,
        title: "Mountain Bike",
        price: 349.99,
        location: "Denver, CO",
        datePosted: "2023-05-13",
        images: ["/placeholder.svg?height=200&width=300"],
        user: {
          id: 103,
          firstName: "Mike",
          lastName: "Johnson",
          rating: 4.2,
        },
        isSaved: false,
      },
      {
        id: 4,
        title: "Antique Wooden Desk",
        price: 299.99,
        location: "Chicago, IL",
        datePosted: "2023-05-12",
        images: [
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
          "/placeholder.svg?height=200&width=300",
        ],
        user: {
          id: 104,
          firstName: "Emily",
          lastName: "Brown",
          rating: 4.6,
        },
        isSaved: true,
      },
    ];
  }
</script>

{$advertss}
<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto">
    <h1
      class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center"
    >
      Recent Adverts
    </h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
      {#each adverts as advert (advert.id)}
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
          in:fade={{ duration: 300 }}
        >
          <div
            class="relative h-48 overflow-hidden"
            on:mousemove={(e) => handleImageScroll(e, advert)}
          >
            <div
              class="flex h-full transition-transform duration-300 ease-in-out"
              style="width: {advert.images.length * 100}%;"
            >
              {#each advert.images.slice(0, 5) as image, index}
                <img
                  src={image}
                  alt={`${advert.title} - Image ${index + 1}`}
                  class="w-full h-full object-cover flex-shrink-0"
                />
              {/each}
            </div>
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
                  ? advert.isSaved
                    ? "Remove from saved"
                    : "Save for later"
                  : "Log in to save adverts"}
              >
                <Heart
                  size={20}
                  fill={advert.isSaved ? "currentColor" : "none"}
                />
              </button>
            </div>
            <p class="text-xl font-bold text-gray-900 dark:text-white mb-2">
              ${advert.price.toFixed(2)}
            </p>
            <div
              class="flex items-center mb-1 text-sm text-gray-600 dark:text-gray-300"
            >
              <MapPin size={16} class="mr-1" />
              <span class="truncate">{advert.location}</span>
              <span class="ml-1 text-gray-500 dark:text-gray-400"> 2 km </span>
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
              {formatDate(advert.datePosted)}
            </p>
            <div class="flex items-center justify-between">
              <button
                class="text-sm text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium"
                on:click={() => navigateToUserProfile(advert.user.id)}
              >
                {advert.user.firstName}
                {advert.user.lastName}
              </button>
              <div class="flex items-center">
                <Star size={16} class="text-yellow-400 mr-1" />
                <span class="text-sm text-gray-600 dark:text-gray-300"
                  >{advert.user.rating.toFixed(1)}</span
                >
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  /* Hide scrollbar for Chrome, Safari and Opera */
  .overflow-hidden::-webkit-scrollbar {
    display: none;
  }

  /* Hide scrollbar for IE, Edge and Firefox */
  .overflow-hidden {
    -ms-overflow-style: none; /* IE and Edge */
    scrollbar-width: none; /* Firefox */
  }
</style>

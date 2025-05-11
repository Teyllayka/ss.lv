<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { graphql } from "$houdini";
  import Advert from "$lib/components/Advert.svelte";
  import type { Writable } from "svelte/store";

  const advertsQuery = graphql(`
    query SearchAdverts(
      $category: String
      $offset: Int!
      $title: String!
      $minPrice: Float
      $maxPrice: Float
      $minRating: Float
      $sortField: String
      $sortOrder: String
      $centerLat: Float
      $centerLon: Float
      $locationRange: Float
      $customFields: JSON
    ) @cache(policy: NoCache) {
      searchAdverts(
        category: $category
        offset: $offset
        title: $title
        minPrice: $minPrice
        maxPrice: $maxPrice
        minRating: $minRating
        sortField: $sortField
        sortOrder: $sortOrder
        centerLat: $centerLat
        centerLon: $centerLon
        locationRange: $locationRange
        customFields: $customFields
      ) {
        id
        title
        price
        oldPrice
        lat
        lon
        createdAt
        isFavorited
        photoUrl
        additionalPhotos
        user {
          id
          name
          surname
          rating
        }
      }
    }
  `);

  type Category = {
    id: string;
    name: string;
    fields: AdvertField[];
  };

  type AdvertField = {
    id: string;
    name: string;
    type: "text" | "number" | "boolean";
  };

  let categories: Category[] = [];
  let selectedCategory: Category | null = null;
  let minPrice = 0;
  let maxPrice = 1000;
  let locationRange = 50;
  let searchTerm = "";
  let minRating = 0;
  let customFields: Record<string, string | number | boolean> = {};
  let sortOption = "price";
  let sortOrder = "asc";
  let offset = 0;
  let adverts: Advert[] = [];

  const locationStore = getContext<Writable<[number, number]>>("location");

  function getVariables() {
    return {
      category: selectedCategory ? selectedCategory.id : null,
      offset,
      title: searchTerm,
      minPrice,
      maxPrice,
      minRating,
      sortField: sortOption,
      sortOrder,
      centerLat: $locationStore[0] === 0 ? null : $locationStore[0],
      centerLon: $locationStore[1] === 0 ? null : $locationStore[1],
      locationRange,
      customFields,
    };
  }

  async function performSearch() {
    const result = await advertsQuery.fetch({ variables: getVariables() });
    if (result.data && result.data.searchAdverts) {
      adverts = result.data.searchAdverts;
    } else {
      adverts = [];
    }
  }

  onMount(() => {
    categories = [
      {
        id: "1",
        name: "Electronics",
        fields: [
          { id: "brand", name: "Brand", type: "text" },
          { id: "condition", name: "Condition", type: "text" },
        ],
      },
      {
        id: "2",
        name: "Vehicles",
        fields: [
          { id: "make", name: "Make", type: "text" },
          { id: "model", name: "Model", type: "text" },
          { id: "year", name: "Year", type: "number" },
        ],
      },
    ];

    const params = new URLSearchParams(window.location.search);
    const q = params.get("q");
    if (q) {
      searchTerm = q;
      performSearch();
    }
  });

  function handleCategoryChange(event: Event) {
    const categoryId = (event.target as HTMLSelectElement).value;
    selectedCategory = categories.find((c) => c.id === categoryId) || null;
    customFields = {};
  }

  function handleCustomFieldChange(field: AdvertField, event: Event) {
    const value = (event.target as HTMLInputElement).value;
    customFields = {
      ...customFields,
      [field.id]: field.type === "number" ? Number(value) : value,
    };
  }
</script>

<svelte:head>
  <title>Search Adverts</title>
  <meta name="description" content="Search adverts with various filters" />
</svelte:head>

<main
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
>
  <div class="max-w-7xl mx-auto">
    <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
      Search Adverts
    </h1>

    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <aside class="md:col-span-1">
        <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
          <h2 class="text-2xl font-semibold text-gray-900 dark:text-white mb-4">
            Filters
          </h2>

          <div class="mb-4">
            <label
              for="category"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Category
            </label>
            <select
              id="category"
              class="w-full p-2 border border-gray-300 rounded-md"
              on:change={handleCategoryChange}
            >
              <option value="">All Categories</option>
              {#each categories as category}
                <option value={category.id}>{category.name}</option>
              {/each}
            </select>
          </div>

          {#if selectedCategory}
            {#each selectedCategory.fields as field}
              <div class="mb-4">
                <label
                  for={field.id}
                  class="block mb-2 font-medium text-gray-900 dark:text-white"
                >
                  {field.name}
                </label>
                <input
                  type={field.type === "number" ? "number" : "text"}
                  id={field.id}
                  class="w-full p-2 border border-gray-300 rounded-md"
                  on:input={(event) => handleCustomFieldChange(field, event)}
                />
              </div>
            {/each}
          {/if}

          <div class="mb-4">
            <label
              for="minPrice"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Min Price
            </label>
            <input
              type="number"
              id="minPrice"
              bind:value={minPrice}
              class="w-full p-2 border border-gray-300 rounded-md"
            />
          </div>

          <div class="mb-4">
            <label
              for="maxPrice"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Max Price
            </label>
            <input
              type="number"
              id="maxPrice"
              bind:value={maxPrice}
              class="w-full p-2 border border-gray-300 rounded-md"
            />
          </div>

          <div class="mb-4">
            <label
              for="minRating"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Min Rating
            </label>
            <input
              type="number"
              id="minRating"
              bind:value={minRating}
              min="0"
              max="5"
              step="0.1"
              class="w-full p-2 border border-gray-300 rounded-md"
            />
          </div>

          <div class="mb-4">
            <label
              for="locationRange"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Location Range (km): {locationRange}
            </label>
            <input
              type="range"
              id="locationRange"
              min="0"
              max="2000"
              step="10"
              bind:value={locationRange}
              class="w-full"
            />
          </div>

          <div class="mb-4">
            <label
              for="sortOption"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Sort By
            </label>
            <select
              id="sortOption"
              bind:value={sortOption}
              class="w-full p-2 border border-gray-300 rounded-md"
            >
              <option value="price">Price</option>
              <option value="rating">Rating</option>
              <option value="dateCreated">Date Created</option>
            </select>
          </div>

          <div class="mb-4">
            <label
              for="sortOrder"
              class="block mb-2 font-medium text-gray-900 dark:text-white"
            >
              Sort Order
            </label>
            <select
              id="sortOrder"
              bind:value={sortOrder}
              class="w-full p-2 border border-gray-300 rounded-md"
            >
              <option value="asc">Ascending</option>
              <option value="desc">Descending</option>
            </select>
          </div>
        </div>
      </aside>

      <div class="md:col-span-3">
        <div class="mb-6 flex gap-2">
          <input
            type="text"
            placeholder="Search adverts..."
            bind:value={searchTerm}
            class="w-full p-2 border border-gray-300 rounded-md dark:bg-gray-800 dark:text-white"
          />
          <button
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            on:click={performSearch}
          >
            Search
          </button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {#if adverts.length === 0}
            <div class="col-span-full text-center py-4 dark:text-gray-400">
              No adverts found matching your criteria.
            </div>
          {:else}
            {#each adverts as advert (advert.id)}
              <Advert {advert} />
            {/each}
          {/if}
        </div>
      </div>
    </div>
  </div>
</main>

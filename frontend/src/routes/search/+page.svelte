<script lang="ts">
  import { getContext, onDestroy, onMount, tick } from "svelte";
  import { graphql } from "$houdini";
  import Advert from "$lib/components/Advert.svelte";
  import type { Writable } from "svelte/store";
  import { categories, categoryFields } from "$lib/consts.js";
  import { page } from "$app/stores";
  import { browser } from "$app/environment";
  import { afterNavigate } from "$app/navigation";

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

  let selectedCategory: string = "";
  let searchTerm = "";
  let minPrice = 0;
  let maxPrice: any = null;
  let minRating = 0;
  let locationRange = 50;
  let sortOption = "price";
  let sortOrder = "asc";
  let offset = 0;
  let adverts: Advert[] = [];
  let customFields: Record<string, string | number | boolean> = {};
  let initialized = false;
  let rangeDebounceTimeout: ReturnType<typeof setTimeout>;

  onDestroy(() => {
    clearTimeout(rangeDebounceTimeout);
  });

  const locationStore = getContext<Writable<[number, number]>>("location");

  function parseQueryParams() {
    const params = $page.url.searchParams;
    selectedCategory = params.get("category") || "";
    searchTerm = params.get("title") || "";
    minPrice = params.has("minPrice")
      ? Number(params.get("minPrice"))
      : minPrice;
    maxPrice = params.has("maxPrice")
      ? Number(params.get("maxPrice"))
      : maxPrice;
    minRating = params.has("minRating")
      ? Number(params.get("minRating"))
      : minRating;
    locationRange = params.has("locationRange")
      ? Number(params.get("locationRange"))
      : locationRange;
    sortOption = params.get("sortField") || sortOption;
    sortOrder = params.get("sortOrder") || sortOrder;
    customFields = {};
    if (selectedCategory && categoryFields[selectedCategory]) {
      categoryFields[selectedCategory].forEach((field) => {
        const value = params.get(field.name);
        customFields[field.name] =
          value != null
            ? field.type === "number"
              ? Number(value)
              : value
            : "";
      });
    }
  }

  function updateQueryParams() {
    const params = new URLSearchParams();
    if (selectedCategory) params.set("category", selectedCategory);
    if (searchTerm) params.set("title", searchTerm);
    if (minPrice) params.set("minPrice", String(minPrice));
    if (maxPrice) params.set("maxPrice", String(maxPrice));
    if (minRating) params.set("minRating", String(minRating));
    if (locationRange !== 50)
      params.set("locationRange", String(locationRange));
    if (sortOption !== "price") params.set("sortField", sortOption);
    if (sortOrder !== "asc") params.set("sortOrder", sortOrder);
    if (selectedCategory && categoryFields[selectedCategory]) {
      categoryFields[selectedCategory].forEach((field) => {
        const val = customFields[field.name];
        if (val !== "" && val != null) params.set(field.name, String(val));
      });
    }
    if (browser) {
      history.replaceState(
        {},
        "",
        `${window.location.pathname}?${params.toString()}`,
      );
    }
  }

  function getVariables() {
    const filteredCustom: Record<string, string | number | boolean> = {};
    Object.keys(customFields).forEach((key) => {
      const val = customFields[key];
      if (val !== "" && val != null) {
        filteredCustom[key] = val;
      }
    });

    return {
      category: selectedCategory || null,
      offset,
      title: searchTerm,
      minPrice,
      maxPrice,
      minRating,
      sortField: sortOption,
      sortOrder,
      centerLat: $locationStore[0] || null,
      centerLon: $locationStore[1] || null,
      locationRange,
      customFields: Object.keys(filteredCustom).length ? filteredCustom : null,
    };
  }

  async function performSearch() {
    updateQueryParams();
    const result = await advertsQuery.fetch({ variables: getVariables() });
    adverts = result.data?.searchAdverts || [];
  }

  function handleCategoryChange(event: Event) {
    selectedCategory = (event.target as HTMLSelectElement).value;
    customFields = {};
    if (selectedCategory && categoryFields[selectedCategory]) {
      categoryFields[selectedCategory].forEach((field) => {
        customFields[field.name] = "";
      });
    }
  }

  function handleCustomFieldChange(
    field: { name: any; type: string },
    event: Event,
  ) {
    const value = (event.target as HTMLInputElement).value;
    customFields = {
      ...customFields,
      [field.name]: field.type === "number" ? Number(value) : value,
    };
  }

  onMount(async () => {
    parseQueryParams();
    await tick();
    performSearch();
    initialized = true;
  });

  afterNavigate(async () => {
    parseQueryParams();
    await tick();
  });

  $: if (initialized) {
    selectedCategory;
    searchTerm;
    minPrice;
    maxPrice;
    minRating;
    sortOption;
    sortOrder;
    customFields;
    performSearch();
  }

  $: if (initialized) {
    const _ = locationRange;
    clearTimeout(rangeDebounceTimeout);
    rangeDebounceTimeout = setTimeout(() => {
      performSearch();
    }, 300);
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
            <label for="category" class="block mb-2">Category</label>
            <select
              id="category"
              on:change={handleCategoryChange}
              bind:value={selectedCategory}
              class="w-full p-2 border rounded-md"
            >
              <option value="">All Categories</option>
              {#each categories as cat}
                <option value={cat.value}>{cat.label}</option>
              {/each}
            </select>
          </div>
          {#if selectedCategory && categoryFields[selectedCategory]}
            {#each categoryFields[selectedCategory] as field}
              <div class="mb-4">
                <label for={field.name} class="block mb-2">{field.label}</label>
                {#if field.type === "select"}
                  <select
                    id={field.name}
                    on:change={(e) => handleCustomFieldChange(field, e)}
                    class="w-full p-2 border rounded-md"
                  >
                    <option value="">Any</option>
                    {#each field.options as option}
                      <option value={option}>{option}</option>
                    {/each}
                  </select>
                {:else}
                  <input
                    id={field.name}
                    type={field.type}
                    class="w-full p-2 border rounded-md"
                    on:input={(e) => handleCustomFieldChange(field, e)}
                    bind:value={customFields[field.name]}
                  />
                {/if}
              </div>
            {/each}
          {/if}
          <div class="mb-4">
            <label for="minPrice" class="block mb-2">Min Price</label>
            <input
              id="minPrice"
              type="number"
              bind:value={minPrice}
              class="w-full p-2 border rounded-md"
            />
          </div>
          <div class="mb-4">
            <label for="maxPrice" class="block mb-2">Max Price</label>
            <input
              id="maxPrice"
              type="number"
              bind:value={maxPrice}
              class="w-full p-2 border rounded-md"
            />
          </div>
          <div class="mb-4">
            <label for="minRating" class="block mb-2">Min Rating</label>
            <input
              id="minRating"
              type="number"
              min="0"
              max="5"
              step="0.1"
              bind:value={minRating}
              class="w-full p-2 border rounded-md"
            />
          </div>
          <div class="mb-4">
            <label for="locationRange" class="block mb-2"
              >Location Range (km): {locationRange}</label
            >
            <input
              id="locationRange"
              type="range"
              min="0"
              max="2000"
              step="10"
              bind:value={locationRange}
              class="w-full"
            />
          </div>
          <div class="mb-4">
            <label for="sortOption" class="block mb-2">Sort By</label>
            <select
              id="sortOption"
              bind:value={sortOption}
              class="w-full p-2 border rounded-md"
            >
              <option value="price">Price</option>
              <option value="rating">Rating</option>
              <option value="dateCreated">Date Created</option>
            </select>
          </div>
          <div class="mb-4">
            <label for="sortOrder" class="block mb-2">Sort Order</label>
            <select
              id="sortOrder"
              bind:value={sortOrder}
              class="w-full p-2 border rounded-md"
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
            class="w-full p-2 border rounded-md"
          />
          <button
            class="px-4 py-2 bg-blue-600 text-white rounded-md"
            on:click={performSearch}>Search</button
          >
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {#if adverts.length === 0}
            <div class="col-span-full text-center py-4">
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

<script lang="ts">
    import { onMount } from 'svelte';
  
    // Types
    type Category = {
      id: string;
      name: string;
      fields: AdvertField[];
    };
  
    type AdvertField = {
      id: string;
      name: string;
      type: 'text' | 'number' | 'boolean';
    };
  
    type Advert = {
      id: string;
      title: string;
      description: string;
      price: number;
      category: string;
      location: string;
      image: string;
      fields: Record<string, string | number | boolean>;
      rating: number; // Max 5
      dateCreated: string;
    };
  
    // State
    let categories: Category[] = [];
    let selectedCategory: Category | null = null;
    let adverts: Advert[] = [];
    let minPrice = 0;
    let maxPrice = 1000;
    let locationRange = 50;
    let searchTerm = '';
    let minRating = 0;
    let customFields: Record<string, string | number | boolean> = {};
    let sortOption = 'price';
    let sortOrder = 'asc';
  
    // Fetch categories and adverts
    onMount(async () => {
      // In a real application, these would be API calls
      categories = [
        {
          id: '1',
          name: 'Electronics',
          fields: [
            { id: 'brand', name: 'Brand', type: 'text' },
            { id: 'condition', name: 'Condition', type: 'text' },
          ],
        },
        {
          id: '2',
          name: 'Vehicles',
          fields: [
            { id: 'make', name: 'Make', type: 'text' },
            { id: 'model', name: 'Model', type: 'text' },
            { id: 'year', name: 'Year', type: 'number' },
          ],
        },
      ];
  
      adverts = [
        {
          id: '1',
          title: 'iPhone 12',
          description: 'Great condition iPhone 12',
          price: 500,
          category: '1',
          location: 'New York',
          image: '/placeholder.svg?height=200&width=300',
          fields: { brand: 'Apple', condition: 'Used' },
          rating: 4.5, // Max 5
          dateCreated: '2023-10-01',
        },
        {
          id: '2',
          title: 'Toyota Camry',
          description: '2018 Toyota Camry in excellent condition',
          price: 15000,
          category: '2',
          location: 'Los Angeles',
          image: '/placeholder.svg?height=200&width=300',
          fields: { make: 'Toyota', model: 'Camry', year: 2018 },
          rating: 4.9, // Max 5
          dateCreated: '2023-09-25',
        },
      ];
    });
  
    // Filter adverts based on search criteria
    $: filteredAdverts = adverts.filter((advert) => {
      const matchesCategory = !selectedCategory || advert.category === selectedCategory.id;
      const matchesPrice = advert.price >= minPrice && advert.price <= maxPrice;
      const matchesSearch = advert.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                            advert.description.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesCustomFields = Object.entries(customFields).every(([key, value]) => {
        if (value === '') return true;
        return advert.fields[key] === value;
      });
      const matchesRating = advert.rating >= minRating && advert.rating <= 5; // Ensure rating is max 5
  
      return matchesCategory && matchesPrice && matchesSearch && matchesCustomFields && matchesRating;
    });
  
    // Sort adverts based on the selected option
    $: sortedAdverts = [...filteredAdverts].sort((a, b) => {
      let comparison = 0;
  
      if (sortOption === 'price') {
        comparison = a.price - b.price;
      } else if (sortOption === 'rating') {
        comparison = a.rating - b.rating;
      } else if (sortOption === 'dateCreated') {
        comparison = new Date(a.dateCreated).getTime() - new Date(b.dateCreated).getTime();
      }
  
      return sortOrder === 'asc' ? comparison : -comparison;
    });
  
    function handleCategoryChange(event: Event) {
      const categoryId = (event.target as HTMLSelectElement).value;
      selectedCategory = categories.find((c) => c.id === categoryId) || null;
      customFields = {};
    }
  
    function handleCustomFieldChange(field: AdvertField, event: Event) {
      const value = (event.target as HTMLInputElement).value;
      customFields[field.id] = field.type === 'number' ? Number(value) : value;
    }
  </script>
  
  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-3xl font-bold mb-6">Search Adverts</h1>
  
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <aside class="md:col-span-1">
        <div class="bg-gray-100 p-4 rounded-lg">
          <h2 class="text-xl font-semibold mb-4">Filters</h2>
  
          <div class="mb-4">
            <label for="category" class="block mb-2 font-medium">Category</label>
            <select
              id="category"
              class="w-full p-2 border rounded"
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
                <label for={field.id} class="block mb-2 font-medium">{field.name}</label>
                <input
                  type={field.type === 'number' ? 'number' : 'text'}
                  id={field.id}
                  class="w-full p-2 border rounded"
                  on:input={(event) => handleCustomFieldChange(field, event)}
                />
              </div>
            {/each}
          {/if}
  
          <div class="mb-4">
            <label for="minPrice" class="block mb-2 font-medium">Min Price</label>
            <input
              type="number"
              id="minPrice"
              class="w-full p-2 border rounded"
              bind:value={minPrice}
            />
          </div>
  
          <div class="mb-4">
            <label for="maxPrice" class="block mb-2 font-medium">Max Price</label>
            <input
              type="number"
              id="maxPrice"
              class="w-full p-2 border rounded"
              bind:value={maxPrice}
            />
          </div>
  
          <div class="mb-4">
            <label for="minRating" class="block mb-2 font-medium">Min Rating</label>
            <input
              type="number"
              id="minRating"
              class="w-full p-2 border rounded"
              min="0"
              max="5"
              step="0.1"
              bind:value={minRating}
            />
          </div>
  
          <div class="mb-4">
            <label for="locationRange" class="block mb-2 font-medium">
              Location Range (km): {locationRange}
            </label>
            <input
              type="range"
              id="locationRange"
              min="0"
              max="500"
              step="10"
              class="w-full"
              bind:value={locationRange}
            />
          </div>
  
          <div class="mb-4">
            <label for="sortOption" class="block mb-2 font-medium">Sort By</label>
            <select id="sortOption" class="w-full p-2 border rounded" bind:value={sortOption}>
              <option value="price">Price</option>
              <option value="rating">Rating</option>
              <option value="dateCreated">Date Created</option>
            </select>
          </div>
  
          <div class="mb-4">
            <label for="sortOrder" class="block mb-2 font-medium">Sort Order</label>
            <select id="sortOrder" class="w-full p-2 border rounded" bind:value={sortOrder}>
              <option value="asc">Ascending</option>
              <option value="desc">Descending</option>
            </select>
          </div>
        </div>
      </aside>
  
      <div class="md:col-span-3">
        <div class="mb-6">
          <input
            type="text"
            placeholder="Search adverts..."
            class="w-full p-2 border rounded"
            bind:value={searchTerm}
          />
        </div>
  
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {#each sortedAdverts as advert (advert.id)}
            <div class="bg-white shadow rounded-lg overflow-hidden">
              <img src={advert.image} alt={advert.title} class="w-full h-48 object-cover" />
              <div class="p-4">
                <h3 class="text-xl font-semibold mb-2">{advert.title}</h3>
                <p class="text-gray-600 mb-2">{advert.description}</p>
                <p class="text-lg font-bold mb-2">${advert.price}</p>
                <p class="text-sm text-gray-500">Rating: {advert.rating}</p>
                <p class="text-sm text-gray-500">{advert.location}</p>
              </div>
            </div>
          {/each}
        </div>
  
        {#if sortedAdverts.length === 0}
          <p class="text-center text-gray-500 mt-6">No adverts found matching your criteria.</p>
        {/if}
      </div>
    </div>
  </main>
  
  <style>
    /* Add any additional styles here */
  </style>
  
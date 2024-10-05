<!-- <script lang="ts">
import type { PageData } from "./$houdini";
export let data: PageData;
import Advert from "$lib/components/Advert.svelte";

$: ({ User } = data);
$: adverts = $User.data?.user.adverts || [];
</script>

{#if $User.fetching}
  loading...
{:else}
  {JSON.stringify($User.data)}

    
  {#each adverts as advert}
    <Advert
      {advert}
      userPage={true}
    />
  {/each}
{/if} -->


<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';
  import { Star, Phone, Mail, CheckCircle, Heart, MapPin } from 'lucide-svelte';
  import type { PageData } from "./$houdini";
  export let data: PageData;
  import Advert from "$lib/components/Advert.svelte";

  $: ({ User } = data);
  $: allAdverts = $User.data?.user.adverts || [];
  $: finishedAdverts = allAdverts.filter(advert => !advert.available);
  $: currentAdverts = allAdverts.filter(advert => advert.available);

  $: userData = $User.data?.user;
  let activeTab = 'current';


  function switchTab(tab: 'current' | 'finished') {
    activeTab = tab;
  }

 
</script>

{JSON.stringify(allAdverts)}

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 transition-colors duration-300">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    {#if $User.fetching}
      <div class="text-center text-gray-600 dark:text-gray-400">Loading user data...</div>
    {:else if $User.errors}
      <div class="text-center text-red-500 dark:text-red-400">Failed to load user data.</div>
    {:else if $User.data && userData}
      <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden" in:fade={{ duration: 300 }}>
        <div class="p-6">
          <div class="flex flex-col md:flex-row items-center mb-6">
            <!-- <img src={user.profileImage} alt="{userData.name} {userData.surname}" class="w-32 h-32 rounded-full object-cover mb-4 md:mb-0 md:mr-6" /> -->
            <div class="text-center md:text-left">
              <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                {userData.name} {userData.surname}
              </h1>
              <div class="flex items-center justify-center md:justify-start">
                <Star class="w-6 h-6 text-yellow-400 mr-1" />
                <!-- <span class="text-2xl font-semibold text-gray-900 dark:text-white">{user.rating.toFixed(1)}</span> -->
              </div>
            </div>
          </div>
                             
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
            <div class="flex items-center">
              <Phone class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
              <span class="text-gray-700 dark:text-gray-300">{userData.phone}</span>
            </div>
            <div class="flex items-center">
              <Mail class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
              <span class="text-gray-700 dark:text-gray-300">{userData.email}</span>
            </div>
          </div>
          
          <div class="bg-gray-100 dark:bg-gray-700 rounded-lg p-4 mb-8">
            <div class="flex items-center">
              <CheckCircle class="w-6 h-6 text-green-500 mr-2" />
              <span class="text-lg font-semibold text-gray-900 dark:text-white">
                {finishedAdverts.length} Finished Adverts
              </span>
            </div>
          </div>
          
          <div class="mb-6">
            <div class="flex border-b border-gray-200 dark:border-gray-700">
              <button
                class={`py-2 px-4 font-medium text-sm focus:outline-none ${
                  activeTab === 'current'
                    ? 'border-b-2 border-blue-500 text-blue-500'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
                }`}
                on:click={() => switchTab('current')}
              >
                Current Adverts
              </button>
              <button
                class={`py-2 px-4 font-medium text-sm focus:outline-none ${
                  activeTab === 'finished'
                    ? 'border-b-2 border-blue-500 text-blue-500'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
                }`}
                on:click={() => switchTab('finished')}
              >
                Finished Adverts
              </button>
            </div>
          </div>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {#each (activeTab === 'current' ? currentAdverts : finishedAdverts) as advert (advert.id)}
              <Advert
                {advert}
                userPage={true}
              />
              <!-- <div 
                class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-all duration-300 hover:shadow-lg"
                in:fly={{ y: 20, duration: 300 }}
              >
                <div 
                  class="relative h-48 overflow-hidden"
                  on:mousemove={(e) => handleImageChange(e, advert)}
                >
                  <div class="flex h-full" style="width: {Math.min(advert.images.length, 5) * 100}%;">
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
                    <h2 class="text-lg font-semibold text-gray-900 dark:text-white truncate">{advert.title}</h2>
                    <button
                      class="text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 transition-colors duration-300"
                      on:click={() => toggleSaveAdvert(advert)}
                      title={advert.isFavorited ? 'Remove from saved' : 'Save for later'}
                    >
                      <Heart size={20} fill={advert.isFavorited ? 'currentColor' : 'none'} />
                    </button>
                  </div>
                  <p class="text-xl font-bold text-gray-900 dark:text-white mb-2">${advert.price.toFixed(2)}</p>
                  <div class="flex items-center mb-1 text-sm text-gray-600 dark:text-gray-300">
                    <MapPin size={16} class="mr-1" />
                    <span class="truncate">{advert.location}</span>
                  </div>
                  <p class="text-sm text-gray-500 dark:text-gray-400">
                    {activeTab === 'current' ? 'Posted on' : 'Sold on'} {formatDate(advert.createdAt)}
                  </p>
                </div>
              </div> -->
            {/each}
          </div>
          
          {#if (activeTab === 'current' ? currentAdverts : finishedAdverts).length === 0}
            <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
              No {activeTab} adverts to display.
            </p>
          {/if}
        </div>
      </div>
    {:else}
      <div class="text-center text-gray-600 dark:text-gray-400">Loading user data...</div>
    {/if}
  </div>
</div>

<style>
  /* Hide scrollbar for Chrome, Safari and Opera */
  .overflow-hidden::-webkit-scrollbar {
    display: none;
  }

  /* Hide scrollbar for IE, Edge and Firefox */
  .overflow-hidden {
    -ms-overflow-style: none;  /* IE and Edge */
    scrollbar-width: none;  /* Firefox */
  }
</style>
<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { Star, Phone, Mail, CheckCircle, Heart, MapPin, User, ShoppingBag, MessageSquare, AtSign } from 'lucide-svelte';
  import type { PageData } from "./$houdini";
    import { formatDate } from '$lib/helpers';
  export let data: PageData;

  $: ({ User: UserInfo } = data);
  $: userData = $UserInfo.data?.user;
  let activeTab = 'profile';
  let activeReviewTab = 'received';

  function switchTab(tab: 'profile' | 'adverts') {
    activeTab = tab;
  }

  function switchReviewTab(tab: 'received' | 'written') {
    activeReviewTab = tab;
  }

  function renderStars(rating: number) {
    return Array(5).fill(0).map((_, i) => 
      i < Math.floor(rating) ? '★' : i < rating ? '½' : '☆'
    ).join('');
  }

</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 transition-colors duration-300">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    {#if $UserInfo.fetching}
      <div class="text-center text-gray-600 dark:text-gray-400">Loading user data...</div>
    {:else if $UserInfo.errors}
      <div class="text-center text-red-500 dark:text-red-400">Failed to load user data.</div>
    {:else if $UserInfo.data && userData}
      <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden" in:fade={{ duration: 300 }}>
        <div class="p-6">
          <div class="flex flex-col md:flex-row items-center mb-6">
            <img src={userData.avatarUrl} alt="{userData.name} {userData.surname}" class="w-32 h-32 rounded-full object-cover mb-4 md:mb-0 md:mr-6" />
            <div class="text-center md:text-left">
              <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                {userData.name} {userData.surname}
              </h1>
              <div class="flex items-center justify-center md:justify-start mb-2">
                <Star class="w-6 h-6 text-yellow-400 mr-1" />
                <span class="text-2xl font-semibold text-gray-900 dark:text-white">{userData.rating.toFixed(1)}</span>
                <span class="text-sm text-gray-500 dark:text-gray-400 ml-2">
                  (based on {userData.adverts ? userData.adverts.length : 0} reviews)
                </span>
              </div>
              <div class="flex items-center justify-center md:justify-start">
                <CheckCircle class="w-5 h-5 text-green-500 mr-2" />
                <span class="text-sm font-semibold text-gray-900 dark:text-white">
                  {userData.adverts ? userData.adverts.filter(a => !a.available).length : 0} Finished Adverts
                </span>
              </div>
            </div>
          </div>
          
          <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === 'profile'
                  ? 'border-b-2 border-blue-500 text-blue-500'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
              }`}
              on:click={() => switchTab('profile')}
            >
              <User class="w-5 h-5 mr-2" />
              Profile
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === 'adverts'
                  ? 'border-b-2 border-blue-500 text-blue-500'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'
              }`}
              on:click={() => switchTab('adverts')}
            >
              <ShoppingBag class="w-5 h-5 mr-2" />
              Adverts
            </button>
          </div>
          
          {#if activeTab === 'profile'}
            <div in:fade>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                {#if userData.phone}
                <div class="flex items-center">
                  <Phone class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
                  <span class="text-gray-700 dark:text-gray-300">{userData.phone}</span>
                </div>
                {/if}
                {#if userData.email}
                <div class="flex items-center">
                  <Mail class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
                  <span class="text-gray-700 dark:text-gray-300">{userData.email}</span>
                </div>
                {/if}
                {#if userData.telegramUsername}
                <div class="flex items-center">
                  <AtSign class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
                  <a href="https://t.me/{userData.telegramUsername}" target="_blank" rel="noopener noreferrer" class="text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300">
                    @{userData.telegramUsername}
                  </a>
                </div>
                {/if}
              </div>

              <div class="mb-6 flex justify-center">
                <div class="inline-flex rounded-md shadow-sm" role="group">
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === 'received'
                        ? 'bg-blue-500 text-white'
                        : 'bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600'
                    }`}
                    on:click={() => switchReviewTab('received')}
                  >
                    Received Reviews
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === 'written'
                        ? 'bg-blue-500 text-white'
                        : 'bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600'
                    }`}
                    on:click={() => switchReviewTab('written')}
                  >
                    Written Reviews
                  </button>
                </div>
              </div>
              
              {#if activeReviewTab === 'received'}
                {#if userData.advertsWithReviews && userData.advertsWithReviews.length > 0}
                  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {#each userData.advertsWithReviews as advert}
                      {#if advert.review}
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden" in:fly={{ y: 20, duration: 300 }}>
                          <div class="flex flex-col sm:flex-row">
                            <div class="bg-gray-50 dark:bg-gray-700 p-4 flex-1">
                              <div class="flex items-start mb-2">
                                <img src="" alt={advert.review.user.name} class="w-10 h-10 rounded-full mr-3" />
                                <div>
                                  <h3 class="text-sm font-semibold text-gray-900 dark:text-white">{advert.review.user.name}</h3>
                                  <div class="text-yellow-400 text-xs">
                                    {renderStars(advert.review.rating)}
                                  </div>
                                  <p class="text-xs text-gray-500 dark:text-gray-400">
                                    {formatDate(advert.review.createdAt.toString())}
                                  </p>
                                </div>
                              </div>
                              <p class="text-sm text-gray-600 dark:text-gray-300 line-clamp-3">{advert.review.message}</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 p-4 flex-1 border-t sm:border-t-0 sm:border-l border-gray-200 dark:border-gray-600">
                              <img src="" alt={advert.title} class="w-full h-24 object-cover rounded-md mb-2" />
                              <h4 class="text-sm font-medium text-gray-900 dark:text-white">{advert.title}</h4>
                              <p class="text-xs text-gray-500 dark:text-gray-400">${advert.price.toFixed(2)}</p>
                              <p class="text-xs text-gray-500 dark:text-gray-400">{advert.location}</p>
                            </div>
                          </div>
                        </div>
                      {/if}
                    {/each}
                  </div>
                {:else}
                  <p class="text-gray-600 dark:text-gray-400 text-center">No reviews received yet.</p>
                {/if}
              {:else}
                {#if userData.reviewedAdverts && userData.reviewedAdverts.length > 0}
                  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {#each userData.reviewedAdverts as advert}
                      {#if advert.review}
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden" in:fly={{ y: 20, duration: 300 }}>
                          <div class="flex flex-col sm:flex-row">
                            <div class="bg-gray-50 dark:bg-gray-700 p-4 flex-1">
                              <div class="flex items-start mb-2">
                                <img src={userData.avatarUrl} alt={userData.name} class="w-10 h-10 rounded-full mr-3" />
                                <div>
                                  <h3 class="text-sm font-semibold text-gray-900 dark:text-white">{userData.name}</h3>
                                  <div class="text-yellow-400 text-xs">
                                    {renderStars(advert.review.rating)}
                                  </div>
                                  <p class="text-xs text-gray-500 dark:text-gray-400">
                                    {formatDate(advert.review.createdAt.toString())}
                                  </p>
                                </div>
                              </div>
                              <p class="text-sm text-gray-600 dark:text-gray-300 line-clamp-3">{advert.review.message}</p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 p-4 flex-1 border-t sm:border-t-0 sm:border-l border-gray-200 dark:border-gray-600">
                              <img src="" alt={advert.title} class="w-full h-24 object-cover rounded-md mb-2" />
                              <h4 class="text-sm font-medium text-gray-900 dark:text-white">{advert.title}</h4>
                              <p class="text-xs text-gray-500 dark:text-gray-400">${advert.price.toFixed(2)}</p>
                              <p class="text-xs text-gray-500 dark:text-gray-400">{advert.location}</p>
                            </div>
                          </div>
                        </div>
                      {/if}
                    {/each}
                  </div>
                {:else}
                  <p class="text-gray-600 dark:text-gray-400 text-center">No reviews written yet.</p>
                {/if}
              {/if}
            </div>
          {:else if activeTab === 'adverts'}
            <div in:fade>
              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {#each userData.adverts as advert (advert.id)}
                  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden">
                    <img src="" alt={advert.title} class="w-full h-48 object-cover" />
                    <div class="p-4">
                      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">{advert.title}</h3>
                      <p class="text-xl font-bold text-gray-900 dark:text-white mb-2">${advert.price.toFixed(2)}</p>
                      <div class="flex items-center mb-2">
                        <MapPin class="w-4 h-4 text-gray-500 dark:text-gray-400 mr-1" />
                        <span class="text-sm text-gray-600 dark:text-gray-300">{advert.location}</span>
                      </div>
                      <p class="text-sm text-gray-500 dark:text-gray-400">
                        {advert.available ? 'Available' : 'Sold'} • Posted on {formatDate(advert.createdAt.toString())}
                      </p>
                    </div>
                  </div>
                {/each}
              </div>
              
              {#if userData.adverts.length === 0}
                <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
                  No adverts to display.
                </p>
              {/if}
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="text-center text-gray-600 dark:text-gray-400">No user data available.</div>
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
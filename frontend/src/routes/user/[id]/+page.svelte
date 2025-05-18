<script lang="ts">
  import { fade } from "svelte/transition";
  import * as m from "$lib/paraglide/messages.js";
  import {
    Star,
    Phone,
    Mail,
    CheckCircle,
    User,
    ShoppingBag,
    UserX,
  } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import { renderStars } from "$lib/helpers";
  import ProfileAdvert from "$lib/components/ProfileAdvert.svelte";
  import ProfileReview from "$lib/components/ProfileReview.svelte";
  import { activeTabClass, inactiveTabClass } from "$lib/consts";
  import { user } from "$lib/userStore";
  import { BanUserStore } from "$houdini";

  const banUser = new BanUserStore();

  export let data: PageData;

  $: userData = data.user.data?.user;

  let activeTab: TabType = "profile";
  let activeReviewTab: ReviewTabType = "received";
  let activeAdvertTab: AdvertTabType = "active";

  function switchTab(tab: TabType, subTab?: ReviewTabType | AdvertTabType) {
    activeTab = tab;
    if (tab === "adverts") {
      activeAdvertTab = (subTab as AdvertTabType) || "active";
    } else if (tab === "profile") {
      activeReviewTab = (subTab as ReviewTabType) || "received";
    }
  }

  async function handleBan() {
    try {
      if (!userData) return;

      let res = await banUser.mutate({
        userId: userData.id,
      });

      console.log("User banned:", res);
    } catch (error) {
      console.error("Error toggling favorite:", error);
    }
  }

  $: filteredActiveAdverts =
    userData?.adverts?.filter((a) => a.available) || [];
  $: filteredSoldAdverts = userData?.adverts?.filter((a) => !a.available) || [];
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div
      class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden relative"
      in:fade={{ duration: 300 }}
    >
      {#if $user.role === "ADMIN" && userData}
        <button
          on:click={handleBan}
          class="absolute top-4 right-4 flex items-center bg-red-600 hover:bg-red-700 text-white font-semibold px-4 py-2 rounded-md focus:outline-none focus:ring-2 focus:ring-red-500"
        >
          <UserX class="w-5 h-5 mr-2" />
          <span>{m.ban_user()}</span>
        </button>
      {/if}

      <div class="p-6">
        {#if userData}
          <div class="flex flex-col md:flex-row items-center mb-6">
            {#if userData.avatarUrl}
              <img
                src={userData.avatarUrl}
                alt="{userData.name} {userData.surname}"
                class="w-32 h-32 rounded-full object-cover mb-4 md:mb-0 md:mr-6"
              />
            {/if}
            <div class="text-center md:text-left">
              <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                {userData.name}
                {userData.surname}
              </h1>

              {#if userData.advertsWithReviews.length > 0}
                <div
                  class="flex items-center justify-center md:justify-start mb-2"
                >
                  {#each renderStars(userData.rating) as star}
                    <Star
                      class={star.isFilled
                        ? "text-yellow-400 fill-current"
                        : "text-gray-300"}
                      size="16"
                    />
                  {/each}
                  <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                    ({userData.rating.toFixed(1)}) (based on {userData
                      .advertsWithReviews.length} reviews)
                  </span>
                </div>
              {/if}

              <div class="flex items-center justify-center md:justify-start">
                <CheckCircle class="w-5 h-5 text-green-500 mr-2" />
                <span
                  class="text-sm font-semibold text-gray-900 dark:text-white"
                >
                  {userData.adverts?.filter((a) => !a.available).length || 0} Finished
                  {m.adverts()}
                </span>
              </div>
            </div>
          </div>

          <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "profile" ? activeTabClass : inactiveTabClass
              }`}
              on:click={() => switchTab("profile")}
            >
              <User class="w-5 h-5 mr-2" />
              {m.profile()}
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "adverts" ? activeTabClass : inactiveTabClass
              }`}
              on:click={() => switchTab("adverts")}
            >
              <ShoppingBag class="w-5 h-5 mr-2" />
              {m.adverts()}
            </button>
          </div>

          {#if activeTab === "profile"}
            <div in:fade>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                {#if userData.phone}
                  <div class="flex items-center">
                    <Phone
                      class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2"
                    />
                    <span class="text-gray-700 dark:text-gray-300">
                      {userData.phone}
                    </span>
                  </div>
                {/if}
                {#if userData.email}
                  <div class="flex items-center">
                    <Mail
                      class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2"
                    />
                    <span class="text-gray-700 dark:text-gray-300">
                      {userData.email}
                    </span>
                  </div>
                {/if}
              </div>

              <div class="mb-6 flex justify-center">
                <div
                  class="inline-flex rounded-md shadow-sm"
                  role="group"
                  aria-label="Review Tabs"
                >
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === "received"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("profile", "received")}
                  >
                    {m.received_reviews()}
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === "written"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("profile", "written")}
                  >
                    {m.written_reviews()}
                  </button>
                </div>
              </div>

              {#if activeReviewTab === "received" ? userData.advertsWithReviews?.length > 0 : userData.reviewedAdverts?.length > 0}
                {#each activeReviewTab === "received" ? userData.advertsWithReviews : userData.reviewedAdverts as advert}
                  {#if advert.review}
                    <ProfileReview
                      {advert}
                      reviewer={activeReviewTab == "written"
                        ? userData
                        : advert.review.user}
                    />
                  {/if}
                {/each}
              {:else if activeReviewTab === "written" ? userData.reviewedAdverts?.length === 0 : userData.advertsWithReviews?.length === 0}
                <p class="text-gray-600 dark:text-gray-400 text-center">
                  No reviews {activeReviewTab} yet.
                </p>
              {/if}
            </div>
          {:else}
            <div in:fade>
              <div class="mb-6 flex justify-center">
                <div
                  class="inline-flex rounded-md shadow-sm"
                  role="group"
                  aria-label="Advert Tabs"
                >
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "active"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("adverts", "active")}
                  >
                    {m.active_adverts()}
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "sold"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("adverts", "sold")}
                  >
                    {m.sold_adverts()}
                  </button>
                </div>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {#each activeAdvertTab === "active" ? filteredActiveAdverts : filteredSoldAdverts as advert (advert.id)}
                  <ProfileAdvert {advert} />
                {/each}
              </div>

              {#if (activeAdvertTab === "active" ? filteredActiveAdverts : filteredSoldAdverts).length === 0}
                <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
                  No {activeAdvertTab} adverts to display.
                </p>
              {/if}
            </div>
          {/if}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .overflow-hidden::-webkit-scrollbar {
    display: none;
  }

  .overflow-hidden {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>

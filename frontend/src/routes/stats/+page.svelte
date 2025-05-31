<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { Users, ShoppingBag, TrendingUp, Calendar } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import * as m from "$lib/paraglide/messages.js";

  export let data: PageData;

  $: ({ stats } = data);

  let dataStats = {
    users: {
      today: 0,
      total: 0,
    },
    adverts: {
      today: 0,
      total: 0,
    },
    isLoading: true,
  };

  async function fetchStats() {
    try {
      await new Promise((resolve) => setTimeout(resolve, 1000));

      dataStats = {
        users: {
          today: $stats.data?.stats.todayUserCount || 0,
          total: $stats.data?.stats.userCount || 0,
        },
        adverts: {
          today: $stats.data?.stats.todayAdvertCount || 0,
          total: $stats.data?.stats.advertCount || 0,
        },
        isLoading: false,
      };
    } catch (error) {
      console.error("Error fetching stats:", error);
      dataStats.isLoading = false;
    }
  }

  function formatNumber(num: number): string {
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
  }

  onMount(() => {
    fetchStats();
  });
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
>
  <div class="max-w-7xl mx-auto">
    <div class="text-center mb-12" in:fade={{ duration: 300, delay: 100 }}>
      <h1
        class="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-2"
      >
        {m.marketplace_statistics()}
      </h1>
      <p class="text-lg text-gray-600 dark:text-gray-400">
        {m.overview_of_users_and_adverts()}
      </p>
    </div>

    {#if dataStats.isLoading}
      <div class="flex justify-center items-center h-64">
        <div
          class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"
        ></div>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden"
          in:fly={{ y: 20, duration: 300, delay: 200 }}
        >
          <div class="p-6">
            <div class="flex items-center justify-between mb-6">
              <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
                {m.users()}
              </h2>
              <div class="bg-blue-100 dark:bg-blue-900 p-3 rounded-full">
                <Users class="w-6 h-6 text-blue-500 dark:text-blue-400" />
              </div>
            </div>

            <div class="grid grid-cols-2 gap-6">
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">
                  {m.today()}
                </p>
                <div class="flex items-end">
                  <p class="text-3xl font-bold text-gray-900 dark:text-white">
                    {formatNumber(dataStats.users.today)}
                  </p>
                </div>
              </div>

              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">
                  {m.all_time()}
                </p>
                <p class="text-3xl font-bold text-gray-900 dark:text-white">
                  {formatNumber(dataStats.users.total)}
                </p>
              </div>
            </div>
          </div>

          <div class="bg-gray-50 dark:bg-gray-700 px-6 py-3">
            <div
              class="flex items-center text-sm text-gray-500 dark:text-gray-400"
            >
              <Calendar class="w-4 h-4 mr-2" />
              <span>{m.last_updated()}: {new Date().toLocaleString()}</span>
            </div>
          </div>
        </div>

        <div
          class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden"
          in:fly={{ y: 20, duration: 300, delay: 300 }}
        >
          <div class="p-6">
            <div class="flex items-center justify-between mb-6">
              <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
                {m.adverts()}
              </h2>
              <div class="bg-purple-100 dark:bg-purple-900 p-3 rounded-full">
                <ShoppingBag
                  class="w-6 h-6 text-purple-500 dark:text-purple-400"
                />
              </div>
            </div>

            <div class="grid grid-cols-2 gap-6">
              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">
                  {m.today()}
                </p>
                <div class="flex items-end">
                  <p class="text-3xl font-bold text-gray-900 dark:text-white">
                    {formatNumber(dataStats.adverts.today)}
                  </p>
                </div>
              </div>

              <div>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">
                  {m.all_time()}
                </p>
                <p class="text-3xl font-bold text-gray-900 dark:text-white">
                  {formatNumber(dataStats.adverts.total)}
                </p>
              </div>
            </div>
          </div>

          <div class="bg-gray-50 dark:bg-gray-700 px-6 py-3">
            <div
              class="flex items-center text-sm text-gray-500 dark:text-gray-400"
            >
              <Calendar class="w-4 h-4 mr-2" />
              <span>{m.last_updated()}: {new Date().toLocaleString()}</span>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

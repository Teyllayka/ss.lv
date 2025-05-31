<script lang="ts">
  import { onMount } from "svelte";
  import { graphql } from "$houdini";
  import Advert from "$lib/components/Advert.svelte";
  import { browser } from "$app/environment";
  import * as m from "$lib/paraglide/messages.js";

  const adverts = graphql(`
    query Adverts($offset: Int!) @cache(policy: NoCache) {
      getAdverts(limit: 12, offset: $offset) @paginate {
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

  let allAdverts: any[] = [];
  let isLoadingMore = false;
  let isMore = true;

  $: isMore = allAdverts.length > 0 && allAdverts.length % 12 === 0;

  const loadMore = async () => {
    if (isLoadingMore || !$adverts.data) return;
    if (!isMore) return;

    isLoadingMore = true;
    try {
      await adverts.loadNextPage();

      if ($adverts.data?.getAdverts) {
        const newAdverts = $adverts.data.getAdverts.filter(
          (advert) => !allAdverts.some((existing) => existing.id === advert.id),
        );

        allAdverts = [...allAdverts, ...newAdverts];

        if (newAdverts.length < 12) {
          isMore = false;
        }
      }
    } catch (error) {
      console.error("Error loading more adverts:", error);
    } finally {
      isLoadingMore = false;
    }
  };

  const throttle = (fn: (...args: any[]) => any, delay: number) => {
    let lastCall = 0;
    return function (...args: any[]) {
      const now = Date.now();
      if (now - lastCall < delay) return;
      lastCall = now;
      return fn(...args);
    };
  };

  const handleScroll = () => {
    if (!browser) return;

    const scrollPosition = window.scrollY + window.innerHeight;
    const pageHeight = document.body.scrollHeight;

    if (
      scrollPosition > pageHeight * 0.6 &&
      !isLoadingMore &&
      !$adverts.fetching
    ) {
      loadMore();
    }
  };

  const throttledScroll = throttle(handleScroll, 200);

  onMount(() => {
    (async () => {
      try {
        await adverts.fetch({ variables: { offset: 0 } });
        if ($adverts.data?.getAdverts) {
          allAdverts = $adverts.data.getAdverts;
        }
      } catch (err) {
        console.error("Error fetching adverts on mount:", err);
      }
    })();

    if (browser) {
      window.addEventListener("scroll", throttledScroll);
    }

    return () => {
      if (browser) {
        window.removeEventListener("scroll", throttledScroll);
      }
    };
  });
</script>

<svelte:head>
  <title>{m.recent_adverts()}</title>
  <meta name="description" content={m.recent_adverts()} />
</svelte:head>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto">
    <h1
      class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center"
    >
      {m.recent_adverts()}
    </h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
      {#if $adverts.fetching && allAdverts.length === 0}
        <div class="col-span-full text-center py-4">Loading...</div>
      {:else if $adverts.errors}
        <div class="col-span-full text-center py-4 text-red-500">
          {m.error_loading_adverts()}
          <p class="text-sm">{JSON.stringify($adverts.errors)}</p>
        </div>
      {:else}
        {#each allAdverts as advert (advert.id)}
          <Advert {advert} userPage={false} />
        {/each}
      {/if}
    </div>

    {#if isLoadingMore || ($adverts.fetching && allAdverts.length > 0)}
      <div class="text-center py-4 mt-4">
        <div
          class="inline-block h-6 w-6 animate-spin rounded-full border-4 border-solid border-current border-r-transparent"
        ></div>
        <p class="mt-2 text-gray-600 dark:text-gray-400">
          {m.loading_more_adverts()}
        </p>
      </div>
    {/if}
  </div>
</div>

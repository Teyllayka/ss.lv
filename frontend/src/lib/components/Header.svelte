<script lang="ts">
import { preventDefault } from "svelte/legacy";

import {
	Menu,
	X,
	Search,
	MapPin,
	Heart,
	User,
	Plus,
	Shield,
	Settings,
	Building2,
} from "lucide-svelte";
import { fly } from "svelte/transition";
import { clickOutside } from "$lib/helpers";
import { getContext, onMount } from "svelte";
import type { Writable } from "svelte/store";
import { user } from "$lib/userStore";
import * as m from "$lib/paraglide/messages.js";
import { goto } from "$app/navigation";
import { page } from "$app/stores";

const region: Writable<string> = getContext("region");

let isMenuOpen = $state(false);
let searchQuery = $state("");
let isCategoriesOpen = $state(false);
let isRegionsOpen = $state(false);

const regions = ["North", "South", "East", "West", "Central"];
const categories = [
	"Electronics",
	"Fashion",
	"Home & Garden",
	"Sports",
	"Vehicles",
	"Toys & Games",
];

let csrfToken = $state("");

onMount(async () => {
	// const response = await fetch('/csrf-token'); // Adjust to your API endpoint
	// const data = await response.json();
	// csrfToken = data.csrfToken;
	csrfToken = "aa";
});

function toggleMenu() {
	isMenuOpen = !isMenuOpen;
}

async function handleSearch() {
	const url = `/search?q=${encodeURIComponent(searchQuery.trim())}&region=${encodeURIComponent($region)}`;
	await goto(url, {
		keepFocus: true,
	});
}

function selectRegion(rg: string) {
	region.set(rg);
	isRegionsOpen = false;
}

function toggleCategories() {
	isCategoriesOpen = !isCategoriesOpen;
	if (isCategoriesOpen) isRegionsOpen = false;
}

function toggleRegions() {
	isRegionsOpen = !isRegionsOpen;
	if (isRegionsOpen) isCategoriesOpen = false;
}
</script>

<header class="bg-white dark:bg-gray-800 shadow-md">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div
      class="flex justify-between items-center py-4 md:justify-start md:space-x-10"
    >
      <div class="flex justify-start lg:w-0 lg:flex-1">
        <a href="/" class="flex items-center">
          <span class="ml-2 text-xl font-bold text-gray-800 dark:text-white"
            >Adee</span
          >
        </a>
      </div>

      <div class="-mr-2 -my-2 md:hidden">
        <button
          onclick={toggleMenu}
          type="button"
          class="bg-white dark:bg-gray-800 rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
        >
          <span class="sr-only">Open menu</span>
          {#if isMenuOpen}
            <X class="h-6 w-6" aria-hidden="true" />
          {:else}
            <Menu class="h-6 w-6" aria-hidden="true" />
          {/if}
        </button>
      </div>

      {#if $page.url.pathname !== '/search'}
      <nav class="hidden md:flex space-x-10 items-center">
        <div class="relative">
          <button
            onclick={toggleCategories}
            type="button"
            class="text-gray-500 group bg-white dark:bg-gray-800 rounded-md inline-flex items-center text-base font-medium hover:text-gray-900 dark:hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            <span>{m.header_categoies()}</span>
            <svg
              class="ml-2 h-5 w-5 text-gray-400 group-hover:text-gray-500"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              aria-hidden="true"
            >
              <path
                fill-rule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clip-rule="evenodd"
              />
            </svg>
          </button>

          {#if isCategoriesOpen}
            <div
              transition:fly={{ y: -10, duration: 200 }}
              class="origin-top-left absolute left-0 mt-2 w-56 rounded-md shadow-lg bg-white dark:bg-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none z-10"
              use:clickOutside
              onclick_outside={() => (isCategoriesOpen = false)}
            >
              <div class="py-1">
                {#each categories as category}
                  <a
                    href="#{category.toLowerCase().replace(' & ', '-')}"
                    class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-600"
                  >
                    {category}
                  </a>
                {/each}
              </div>
            </div>
          {/if}
        </div>

        <div class="flex-1 flex items-center">
          <form onsubmit={preventDefault(handleSearch)} class="w-full">
            <input type="hidden" name="csrfmiddlewaretoken" value={csrfToken} />
            <div class="relative">
              <input
                type="text"
                bind:value={searchQuery}
                placeholder={m.header_search()}
                class="w-full bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-full py-2 px-4 pl-10 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
              <button
                type="submit"
                class="absolute inset-y-0 left-0 pl-3 flex items-center"
              >
                <Search class="h-5 w-5 text-gray-400" />
              </button>
            </div>
          </form>
        </div>

        <div class="relative">
          <button
            onclick={toggleRegions}
            type="button"
            class="text-gray-500 group bg-white dark:bg-gray-800 rounded-md inline-flex items-center text-base font-medium hover:text-gray-900 dark:hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            <MapPin class="h-5 w-5 mr-2" />
            <span class="inline-block min-w-[100px] text-left">{$region}</span>
            <svg
              class="ml-2 h-5 w-5 text-gray-400 group-hover:text-gray-500"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              aria-hidden="true"
            >
              <path
                fill-rule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clip-rule="evenodd"
              />
            </svg>
          </button>

          {#if isRegionsOpen}
            <div
              transition:fly={{ y: -10, duration: 200 }}
              class="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white dark:bg-gray-700 ring-1 ring-black ring-opacity-5 focus:outline-none z-10"
              use:clickOutside
              onclick_outside={() => (isRegionsOpen = false)}
            >
              <div class="py-1">
                {#each regions as region}
                  <button
                    onclick={() => selectRegion(region)}
                    class="block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-600"
                  >
                    {region}
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </nav>
      {/if}

      <div class="hidden md:flex items-center justify-end md:flex-1 lg:w-0">
        <a
          href="/favorites"
          class="whitespace-nowrap text-base font-medium text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
        >
          <Heart class="h-6 w-6" />
        </a>
        <a
          href="/me"
          class="ml-8 whitespace-nowrap text-base font-medium text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
        >
          {#if $user.role == "ADMIN"}
            <Settings class="h-6 w-6" />
          {:else if $user.role == "MODERATOR"}
            <Shield class="h-6 w-6" />
          {:else if $user.isCompany}
            <Building2 class="h-6 w-6" />
          {:else}
            <User class="h-6 w-6" />
          {/if}
        </a>
        <a
          href="/create"
          class="ml-8 whitespace-nowrap inline-flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700"
        >
          <Plus class="h-5 w-5 mr-2" />
          {m.header_create()}
        </a>
      </div>
    </div>
  </div>

  {#if isMenuOpen}
    <div
      transition:fly={{ y: -100, duration: 300 }}
      class="absolute top-0 inset-x-0 p-2 transition transform origin-top-right md:hidden"
    >
      <div
        class="rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 bg-white dark:bg-gray-800 divide-y-2 divide-gray-50 dark:divide-gray-700"
      >
        <div class="pt-5 pb-6 px-5">
          <div class="flex items-center justify-between">
            <div>
              <!-- <img
                class="h-8 w-auto"
                src="/placeholder.svg"
                alt="Marketplace Logo"
              /> -->
            </div>
            <div class="-mr-2">
              <button
                onclick={toggleMenu}
                type="button"
                class="bg-white dark:bg-gray-800 rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
              >
                <span class="sr-only">Close menu</span>
                <X class="h-6 w-6" aria-hidden="true" />
              </button>
            </div>
          </div>
          <div class="mt-6">
            <nav class="grid gap-y-8">
              {#each categories as category}
                <a
                  href="#{category.toLowerCase().replace(' & ', '-')}"
                  class="-m-3 p-3 flex items-center rounded-md hover:bg-gray-50 dark:hover:bg-gray-700"
                >
                  <span
                    class="ml-3 text-base font-medium text-gray-900 dark:text-white"
                    >{category}</span
                  >
                </a>
              {/each}
            </nav>
          </div>
        </div>
        <div class="py-6 px-5 space-y-6">
          <div class="grid grid-cols-2 gap-y-4 gap-x-8">
            <a
              href="/favorites"
              class="text-base font-medium text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300"
            >
              Favorites
            </a>
            <a
              href="/profile"
              class="text-base font-medium text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300"
            >
              Profile
            </a>
            {#each regions as region}
              <button
                onclick={() => selectRegion(region)}
                class="text-base font-medium text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300"
              >
                {region}
              </button>
            {/each}
          </div>
          <div>
            <a
              href="/create-ad"
              class="w-full flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700"
            >
              Create Ad
            </a>
          </div>
        </div>
      </div>
    </div>
  {/if}
</header>

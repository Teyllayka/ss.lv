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
    BarChart2,
    Sun,
    Moon,
  } from "lucide-svelte";
  import { fly } from "svelte/transition";
  import { clickOutside } from "$lib/helpers";
  import { getContext, onMount, tick } from "svelte";
  import type { Writable } from "svelte/store";
  import { user } from "$lib/userStore";
  import * as m from "$lib/paraglide/messages.js";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import "leaflet/dist/leaflet.css";

  let isDark = false;
  function toggleTheme() {
    isDark = !isDark;
    if (isDark) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  }

  const region: Writable<string> = getContext("region");
  const locationStore: Writable<[number, number]> = getContext("location");

  let isMenuOpen = false;
  let searchQuery = "";
  let csrfToken = "";

  onMount(async () => {
    csrfToken = "aa";
  });

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }

  async function handleSearch() {
    const url = `/search?q=${encodeURIComponent(
      searchQuery.trim()
    )}&region=${encodeURIComponent($region)}`;
    await goto(url, { keepFocus: true });
  }

  // LOCATION LOGIC
  let locationName: string = "Detecting location...";
  let showLocationModal = false;
  let map: any = null;
  let marker: any = null;
  let mapInitialized = false;

  onMount(() => {
    const unsubscribe = locationStore.subscribe(async ([lat, lon]) => {
      if (lat !== 0 && lon !== 0) {
        try {
          const response = await fetch(
            `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lon}&accept-language=en`
          );
          const data = await response.json();
          locationName =
            data.address.city ||
            data.address.town ||
            data.address.village ||
            "Unknown location";
        } catch (e) {
          locationName = "Location not found";
        }
      } else {
        locationName = "Set Location";
      }
    });
    return unsubscribe;
  });

  $: if (showLocationModal && !mapInitialized) {
    tick().then(async () => {
      const L = (await import("leaflet")).default;
      const mapElement = document.getElementById("location-map");
      if (!mapElement) return;
      let currentCoords: [number, number] = [0, 0];
      locationStore.subscribe((coords) => {
        currentCoords = coords;
      })();
      if (currentCoords[0] === 0 && currentCoords[1] === 0) {
        currentCoords = [51.505, -0.09];
      }
      map = L.map(mapElement, {
        attributionControl: false,
      }).setView(currentCoords, 13);
      L.tileLayer(
        "http://mt1.google.com/vt/lyrs=m&x={x}&y={y}&z={z}",
        {}
      ).addTo(map);
      map.on("click", function (e: any) {
        if (marker) {
          marker.setLatLng(e.latlng);
        } else {
          marker = L.marker(e.latlng).addTo(map);
        }
        locationStore.set([e.latlng.lat, e.latlng.lng]);
      });
      mapInitialized = true;
    });
  }

  function confirmLocation() {
    showLocationModal = false;
    mapInitialized = false;
  }
</script>

<header class="bg-white dark:bg-gray-800 shadow-md">
  <div class="max-w-7xl mx-auto px-4 sm:px-6">
    <div class="relative flex items-center justify-between py-4">
      <!-- LEFT: logo and location -->
      <div class="flex items-center space-x-4">
        <!-- Brand / Logo -->
        <a href="/" class="flex items-center">
          <span class="text-xl font-bold text-gray-800 dark:text-white"
            >Adee</span
          >
        </a>

        <!-- Location button positioned between logo and search -->
        <button
          on:click={() => (showLocationModal = true)}
          type="button"
          class="text-gray-500 group bg-white dark:bg-gray-800 rounded-md inline-flex items-center text-base font-medium hover:text-gray-900 dark:hover:text-white focus:outline-none"
        >
          <MapPin class="h-5 w-5 mr-1" />
          <span>{locationName}</span>
        </button>
      </div>

      {#if !$page.url.pathname.includes("/search")}
        <div class="flex-1 max-w-md mx-8">
          <form
            on:submit={preventDefault(handleSearch)}
            class="relative w-full"
          >
            <input
              type="text"
              bind:value={searchQuery}
              placeholder={m.header_search()}
              class="w-full bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-full py-2 pl-10 pr-4 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
            <button
              type="submit"
              class="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400"
            >
              <Search class="h-5 w-5" />
            </button>
          </form>
        </div>
      {/if}

      <!-- RIGHT: user controls and burger -->
      <div class="flex items-center space-x-4">
        <button
          on:click={toggleTheme}
          class="text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none"
          aria-label="Toggle Theme"
        >
          {#if isDark}
            <Sun class="h-6 w-6" />
          {:else}
            <Moon class="h-6 w-6" />
          {/if}
        </button>

        {#if $user.isLoggedIn}
          <!-- Desktop controls -->
          <div class="hidden md:flex items-center space-x-4">
            <a
              href="/favorites"
              class="text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
            >
              <Heart class="h-6 w-6" />
            </a>

            {#if $user.role == "ADMIN" || $user.role == "MODERATOR"}
              <a
                href="/stats"
                class="text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
              >
                <BarChart2 class="h-6 w-6" />
              </a>
            {/if}

            <a
              href="/me"
              class="text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
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
              class="inline-flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700"
            >
              <Plus class="h-5 w-5 mr-1" />
              {m.header_create()}
            </a>
          </div>

          <!-- Mobile burger icon on right side -->
          <button
            type="button"
            class="md:hidden text-gray-500 hover:text-gray-700 focus:outline-none"
            on:click={toggleMenu}
          >
            <Menu class="h-6 w-6" />
          </button>
        {:else}
          <a
            href="/login"
            class="inline-flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700"
          >
            Login
          </a>
        {/if}
      </div>
    </div>
  </div>

  <!-- MOBILE MENU (only if logged in) -->
  {#if isMenuOpen && $user.isLoggedIn}
    <div class="md:hidden">
      <div class="px-2 pt-2 pb-3 space-y-1 bg-white dark:bg-gray-800 shadow">
        <a
          href="/favorites"
          class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          Favorites
        </a>
        {#if $user.role == "ADMIN" || $user.role == "MODERATOR"}
          <a
            href="/stats"
            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            Stats
          </a>
        {/if}
        <a
          href="/me"
          class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          Profile
        </a>
        <a
          href="/create"
          class="block w-full text-left px-3 py-2 rounded-md text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700"
        >
          Create
        </a>
      </div>
    </div>
  {/if}

  <!-- LOCATION MODAL -->
  {#if showLocationModal}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
    >
      <div class="bg-white dark:bg-gray-800 p-4 rounded w-96">
        <div id="location-map" class="w-full h-64"></div>
        <div class="mt-4 flex justify-end space-x-2">
          <button
            on:click={confirmLocation}
            class="px-4 py-2 bg-indigo-600 text-white rounded"
          >
            Confirm
          </button>
          <button
            on:click={() => (showLocationModal = false)}
            class="px-4 py-2 bg-gray-300 text-black rounded"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}
</header>

<script lang="ts">
  import {
    Menu,
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
    MessageSquare,
    X,
  } from "lucide-svelte";
  import { getContext, onMount, tick } from "svelte";
  import { type Writable } from "svelte/store";
  import { user } from "$lib/userStore";
  import * as m from "$lib/paraglide/messages.js";
  import { goto } from "$app/navigation";
  import "leaflet/dist/leaflet.css";
  import { browser } from "$app/environment";
  import { socket } from "$lib/socket";
  import { languageTag, setLanguageTag } from "$lib/paraglide/runtime";
  import { page } from "$app/stores";
  import { afterNavigate } from "$app/navigation";

  afterNavigate(() => {
    isMenuOpen = false;
  });

  let lang = languageTag();
  let isSearchExpanded = false;

  function changeLanguage(e: any) {
    const newLang = (e.target as HTMLSelectElement).value as "en" | "ru" | "lv";
    setLanguageTag(newLang);

    const segments = $page.url.pathname.split("/").filter(Boolean);
    if (["ru", "lv"].includes(segments[0])) segments.shift();
    const prefix = newLang === "en" ? "" : `/${newLang}`;
    const rest = segments.length ? `/${segments.join("/")}` : "";
    const newPath = prefix + rest + $page.url.search;

    goto(newPath, { replaceState: true });
  }
  let csrfToken = null;

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

  interface UnreadMessages {
    unreadMessages: number;
  }
  const areUnreadMessages: Writable<UnreadMessages> =
    getContext("areUnreadMessages");
  const region: Writable<string> = getContext("region");
  const locationStore: Writable<[number, number]> = getContext("location");

  let isMenuOpen = false;
  let searchQuery = "";

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }

  function toggleSearch() {
    isSearchExpanded = !isSearchExpanded;
    if (isSearchExpanded) {
      setTimeout(() => {
        const searchInput = document.getElementById("search-input");
        if (searchInput) searchInput.focus();
      }, 100);
    }
  }

  async function handleSearch() {
    const url = `/search?q=${encodeURIComponent(
      searchQuery.trim(),
    )}&region=${encodeURIComponent($region)}`;
    await goto(url, { keepFocus: true });
    isSearchExpanded = false;
  }

  let locationName: string = "Detecting location...";
  let showLocationModal = false;
  let map: any = null;
  let marker: any = null;
  let mapInitialized = false;

  onMount(() => {
    const saved = localStorage.getItem("location");
    if (saved) {
      try {
        const [lat, lon] = JSON.parse(saved);
        if (lat !== 0 || lon !== 0) {
          locationStore.set([lat, lon]);
        }
      } catch (e) {
        console.warn("Failed to parse saved location:", e);
      }
    }

    isDark = localStorage.getItem("theme") === "dark";

    const unsubscribe = locationStore.subscribe(async ([lat, lon]) => {
      if (lat !== 0 && lon !== 0) {
        try {
          const response = await fetch(
            `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lon}&accept-language=en`,
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

    csrfToken = "aa";

    function handleNewMessage(data: any) {
      if (!$page.url.pathname.includes(`/chats${data.chat_id}`)) {
        areUnreadMessages.update((prev) => ({
          unreadMessages: prev.unreadMessages + 1,
        }));
      }
    }

    socket.on("user-" + $user.id, handleNewMessage);

    fetch("/api/are-unread", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${csrfToken}`,
      },
    })
      .then((response) => response.json())
      .then((data) => {
        areUnreadMessages.set({ unreadMessages: data });
      });

    return () => {
      socket.off("user-" + $user.id, handleNewMessage);
    };

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
      L.Icon.Default.mergeOptions({
        iconRetinaUrl: "/images/marker-icon-2x.png",
        iconUrl: "/images/marker-icon.png",
        shadowUrl: "/images/marker-shadow.png",
      });

      map = L.map(mapElement, {
        attributionControl: false,
      }).setView(currentCoords, 13);
      L.tileLayer(
        "http://mt1.google.com/vt/lyrs=m&x={x}&y={y}&z={z}",
        {},
      ).addTo(map);
      map.on("click", function (e: any) {
        if (marker) {
          marker.setLatLng(e.latlng);
        } else {
          marker = L.marker(e.latlng).addTo(map);
        }
        locationStore.set([e.latlng.lat, e.latlng.lng]);
        localStorage.setItem(
          "location",
          JSON.stringify([e.latlng.lat, e.latlng.lng]),
        );
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
  <div class="max-w-7xl mx-auto px-4 sm:px-2 md:px-4">
    <div class="relative flex items-center justify-between h-16">
      <div class="flex-shrink-0 flex items-center">
        <a href="/" class="flex items-center">
          <span class="text-xl font-bold text-gray-800 dark:text-white"
            >Adee</span
          >
        </a>

        <button
          on:click={toggleTheme}
          class="ml-1 p-2 text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none"
          aria-label="Toggle Theme"
        >
          {#if isDark}
            <Sun class="h-5 w-5" />
          {:else}
            <Moon class="h-5 w-5" />
          {/if}
        </button>

        <div class="hidden sm:flex sm:items-center sm:ml-2">
          <button
            on:click={() => (showLocationModal = true)}
            type="button"
            class="text-gray-500 group bg-white dark:bg-gray-800 rounded-md inline-flex items-center text-sm font-medium hover:text-gray-900 dark:hover:text-white focus:outline-none"
          >
            <MapPin class="h-4 w-4 mr-1" />
            <span class="hidden md:inline text-sm">{locationName}</span>
            <span class="md:hidden text-xs">{locationName}</span>
          </button>
        </div>
      </div>

      {#if !$page.url.pathname.includes("/search")}
        <div
          class="mx-auto absolute left-1/2 transform -translate-x-1/2
         w-5/12 lg:w-5/12 md:w-4/12 sm:w-3/12 hidden sm:block
         px-4"
        >
          <form on:submit|preventDefault={handleSearch} class="relative w-full">
            <input
              type="text"
              bind:value={searchQuery}
              placeholder={m.search()}
              class="w-full bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-full py-1.5 pl-9 pr-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
            <button
              type="submit"
              class="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400"
            >
              <Search class="h-4 w-4" />
            </button>
          </form>
        </div>
      {/if}

      <div class="flex items-center sm:hidden">
        {#if !isSearchExpanded}
          <button
            on:click={() => (showLocationModal = true)}
            type="button"
            class="p-2 rounded-md text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none mr-1"
            aria-label="Set Location"
          >
            <MapPin class="h-5 w-5" />
          </button>

          <button
            on:click={toggleSearch}
            class="p-2 rounded-md text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none"
            aria-label="Search"
          >
            <Search class="h-5 w-5" />
          </button>

          <button
            type="button"
            class="ml-1 p-2 text-gray-500 hover:text-gray-700 focus:outline-none"
            on:click={toggleMenu}
          >
            <Menu class="h-5 w-5" />
          </button>
        {/if}
      </div>

      <div class="hidden sm:flex sm:items-center">
        <div class="flex items-center space-x-1 md:space-x-2">
          <select
            bind:value={lang}
            on:change={changeLanguage}
            class="bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded py-1 pl-2 pr-6 text-sm focus:outline-none appearance-none"
            aria-label="Select language"
          >
            <option value="en">EN</option>
            <option value="lv">LV</option>
            <option value="ru">RU</option>
          </select>

          {#if $user.isLoggedIn && browser}
            <div class="hidden sm:flex items-center space-x-1 md:space-x-2">
              <a
                href="/favorites"
                class="p-1.5 text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                aria-label="Favorites"
              >
                <Heart class="h-5 w-5" />
              </a>

              <a
                href="/chats"
                class="p-1.5 text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white relative"
                aria-label="Chat"
              >
                <MessageSquare class="h-5 w-5" />
                {#if $areUnreadMessages.unreadMessages > 0}
                  <span
                    class="absolute -top-1 -right-1 h-3.5 w-3.5 bg-red-500 rounded-full flex items-center justify-center text-white text-xs font-bold"
                    >!</span
                  >
                {/if}
              </a>

              {#if $user.role == "ADMIN" || $user.role == "MODERATOR"}
                <a
                  href="/stats"
                  class="p-1.5 text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                  aria-label="Stats"
                >
                  <BarChart2 class="h-5 w-5" />
                </a>
              {/if}

              <a
                href="/me"
                class="p-1.5 text-gray-500 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                aria-label="Profile"
              >
                {#if $user.role == "ADMIN"}
                  <Settings class="h-5 w-5" />
                {:else if $user.role == "MODERATOR"}
                  <Shield class="h-5 w-5" />
                {:else if $user.isCompany}
                  <Building2 class="h-5 w-5" />
                {:else}
                  <User class="h-5 w-5" />
                {/if}
              </a>

              <a
                href="/create"
                class="inline-flex items-center justify-center px-3 py-1.5 text-sm border border-transparent rounded-md shadow-sm font-medium text-white bg-blue-500 hover:bg-blue-600"
              >
                <Plus class="h-4 w-4 mr-1 md:mr-1.5" />
                <span class="hidden md:inline">{m.create()}</span>
                <span class="md:hidden">New</span>
              </a>
            </div>
          {:else}
            <a
              href="/login"
              class="inline-flex items-center justify-center px-3 py-1.5 text-sm border border-transparent rounded-md shadow-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700"
            >
              {m.login()}
            </a>
          {/if}
        </div>
      </div>
    </div>
  </div>

  {#if isSearchExpanded}
    <div class="fixed inset-0 z-50 bg-white dark:bg-gray-800 p-4">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-medium text-gray-900 dark:text-white">
          {m.search()}
        </h2>
        <button
          on:click={toggleSearch}
          class="p-2 rounded-md text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none"
        >
          <X class="h-5 w-5" />
        </button>
      </div>
      <form on:submit|preventDefault={handleSearch} class="relative w-full">
        <input
          id="search-input"
          type="text"
          bind:value={searchQuery}
          placeholder={m.search()}
          class="w-full bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg py-3 pl-10 pr-4 focus:outline-none focus:ring-2 focus:ring-indigo-500"
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

  {#if isMenuOpen}
    <div
      class="sm:hidden fixed inset-0 z-40 bg-white dark:bg-gray-800 overflow-y-auto"
    >
      <div class="p-4">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-medium text-gray-900 dark:text-white">
            Menu
          </h2>
          <button
            on:click={toggleMenu}
            class="p-2 rounded-md text-gray-500 hover:text-gray-900 dark:hover:text-white focus:outline-none"
          >
            <X class="h-5 w-5" />
          </button>
        </div>

        <div class="mt-3 space-y-1">
          {#if $user.isLoggedIn}
            <a
              href="/favorites"
              class="flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              <Heart class="h-5 w-5 mr-3" />
              {m.favorites()}
            </a>

            <a
              href="/chats"
              class="flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              <MessageSquare class="h-5 w-5 mr-3" />
              {m.chat()}
              {#if $areUnreadMessages.unreadMessages > 0}
                <span
                  class="ml-2 bg-red-500 text-white text-xs font-bold px-2 py-0.5 rounded-full"
                >
                  {$areUnreadMessages.unreadMessages}
                </span>
              {/if}
            </a>

            {#if $user.role == "ADMIN" || $user.role == "MODERATOR"}
              <a
                href="/stats"
                class="flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <BarChart2 class="h-5 w-5 mr-3" />
                {m.stats()}
              </a>
            {/if}

            <a
              href="/me"
              class="flex items-center px-3 py-3 rounded-md text-base font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              {#if $user.role == "ADMIN"}
                <Settings class="h-5 w-5 mr-3" />
              {:else if $user.role == "MODERATOR"}
                <Shield class="h-5 w-5 mr-3" />
              {:else if $user.isCompany}
                <Building2 class="h-5 w-5 mr-3" />
              {:else}
                <User class="h-5 w-5 mr-3" />
              {/if}
              {m.profile()}
            </a>

            <a
              href="/create"
              class="flex items-center mt-4 px-3 py-3 rounded-md text-base font-medium text-white bg-blue-500 hover:bg-blue-600"
            >
              <Plus class="h-5 w-5 mr-3" />
              {m.create()}
            </a>
          {:else}
            <a
              href="/login"
              class="flex items-center mt-4 px-3 py-3 rounded-md text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700 justify-center"
            >
              {m.login()}
            </a>
          {/if}

          <div class="mt-4 py-2">
            <p
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              {m.language()}
            </p>
            <div class="grid grid-cols-3 gap-2">
              <button
                class={`py-2 px-3 rounded text-center ${lang === "en" ? "bg-indigo-600 text-white" : "bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200"}`}
                on:click={() => {
                  const event = { target: { value: "en" } };
                  changeLanguage(event);
                }}
              >
                EN
              </button>
              <button
                class={`py-2 px-3 rounded text-center ${lang === "lv" ? "bg-indigo-600 text-white" : "bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200"}`}
                on:click={() => {
                  const event = { target: { value: "lv" } };
                  changeLanguage(event);
                }}
              >
                LV
              </button>
              <button
                class={`py-2 px-3 rounded text-center ${lang === "ru" ? "bg-indigo-600 text-white" : "bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200"}`}
                on:click={() => {
                  const event = { target: { value: "ru" } };
                  changeLanguage(event);
                }}
              >
                RU
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}

  {#if showLocationModal}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
    >
      <div
        class="bg-white dark:bg-gray-800 p-4 rounded-lg w-full max-w-md mx-4"
      >
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-3">
          {m.select_location()}
        </h3>
        <div
          id="location-map"
          class="w-full h-64 rounded overflow-hidden"
        ></div>
        <div class="mt-4 flex justify-end space-x-2">
          <button
            on:click={() => (showLocationModal = false)}
            class="px-4 py-2 bg-gray-300 text-black rounded"
          >
            {m.cancel()}
          </button>
          <button
            on:click={confirmLocation}
            class="px-4 py-2 bg-indigo-600 text-white rounded"
          >
            {m.confirm()}
          </button>
        </div>
      </div>
    </div>
  {/if}
</header>

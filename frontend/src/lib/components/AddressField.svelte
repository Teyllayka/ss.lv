<script lang="ts">
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { capitalizeFirstLetter } from "$lib/helpers";
  import { tick } from "svelte";

  const API_URL = "https://nominatim.openstreetmap.org/search?format=json&q=";

  const {
    id,
    name,
    placeholder,
    errors = [],
    value = $bindable(),
    disableAutoFill = false,
    disabled = false,
    onLocationSelect = () => {},
  } = $props();

  let query = $state(value);
  let suggestions = $state<any[]>([]);
  let isFocused = $state(false);
  let selectedSuggestion = $state<any | null>(null);
  let inputEl: HTMLInputElement | null = null;

  let debounceTimer: any;

  let e = $derived(errors.find((x: any) => x.field === name));

  async function fetchSuggestions() {
    if (query.trim().length < 3) {
      suggestions = [];
      return;
    }

    try {
      const response = await fetch(`${API_URL}${encodeURIComponent(query)}`);
      if (response.ok) {
        suggestions = await response.json();
      } else {
        console.error("Failed to fetch suggestions");
        suggestions = [];
      }
    } catch (error) {
      console.error("Error fetching suggestions:", error);
      suggestions = [];
    }
  }

  function debounceFetchSuggestions() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      fetchSuggestions();
    }, 300);
  }

  function handleSuggestionClick(suggestion: any) {
    query = suggestion.display_name;
    onLocationSelect(suggestion);
    selectedSuggestion = suggestion;
    suggestions = [];
  }

  function handleInputFocus() {
    isFocused = true;
  }

  function handleInputBlur() {
    setTimeout(() => {
      isFocused = false;
    }, 200);
  }

  $effect(() => {
    if (e) {
      tick().then(() => {
        if (inputEl) {
          inputEl.focus();
          inputEl.scrollIntoView({ behavior: "smooth", block: "center" });
        }
      });
    }
  });
</script>

<div class="field relative">
  <input
    type="text"
    {name}
    id={id || name}
    bind:value={query}
    on:input={debounceFetchSuggestions}
    on:focus={handleInputFocus}
    on:blur={handleInputBlur}
    bind:this={inputEl}
    class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white border-solid border-2
      {e
      ? 'border-red-500 ring-red-500'
      : 'border-gray-300 dark:border-gray-600'}"
    {placeholder}
    autocomplete={disableAutoFill ? "off" : undefined}
    {disabled}
  />
  <label
    class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
    for={name}>{placeholder}</label
  >
  {#if e}
    <p
      class="text-red-500 text-xs mt-1"
      in:fly={{ y: 10, duration: 300, easing: cubicOut }}
    >
      {capitalizeFirstLetter(e?.message) || ""}
    </p>
  {/if}

  {#if isFocused && suggestions.length > 0}
    <div
      class="suggest-wrapper absolute z-10 w-full mt-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg"
      in:fly={{ y: 10, duration: 300, easing: cubicOut }}
    >
      <div class="suggest max-h-60 overflow-y-auto">
        {#each suggestions as suggestion (suggestion.place_id)}
          <button
            type="button"
            class="w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150 ease-in-out dark:text-gray-200 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
            on:click={() => handleSuggestionClick(suggestion)}
          >
            {suggestion.display_name}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <input
    type="hidden"
    name={`${name}_json`}
    value={selectedSuggestion
      ? JSON.stringify(selectedSuggestion)
      : suggestions[0]
        ? JSON.stringify(suggestions[0])
        : ""}
  />
</div>

<style lang="postcss">
  .suggest-wrapper {
    @apply max-h-60 overflow-y-auto;
  }
  .suggest button:last-child {
    @apply rounded-b-lg;
  }
</style>

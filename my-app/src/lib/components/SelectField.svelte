<script lang="ts">
    import { fly } from "svelte/transition";
    import { cubicOut } from "svelte/easing";
    export let name;
    export let placeholder;
    export let options: any[] = [];
    export let errors: any[] = [];
    export let value: string | undefined | null = null;
    export let onChange: (e: any) => void = () => {};
  
    let e: any = null;
    $: {
      e = errors.find((x) => x.field == name);
    }
</script>
  
<style>
    /* Custom select styling */
    select {
      appearance: none;
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='%236b7280'%3E%3Cpath stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M19 9l-7 7-7-7'%3E%3C/path%3E%3C/svg%3E");
      background-repeat: no-repeat;
      background-position: right 0.75rem center;
      background-size: 1em;
    }
  
    /* For dark mode */
    :global(.dark) select {
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='%239ca3af'%3E%3Cpath stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M19 9l-7 7-7-7'%3E%3C/path%3E%3C/svg%3E");
    }
  
    /* Hide default arrow in IE */
    select::-ms-expand {
      display: none;
    }
  
    /* For Safari */
    @supports (-webkit-appearance: none) {
      select {
        -webkit-appearance: none;
      }
    }
  </style>

<select
id={name}
{value}
{name}
required
on:change={onChange}
class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out text-gray-800 dark:text-white {e?.message
      ? 'border-red-500'
      : 'border-gray-300 dark:border-gray-600'}"
>
<option value="">{placeholder}</option>
{#each options as option}
  <option value={option.value}>{option.label}</option>
{/each}
</select>


{#if e}
    <p
      class="text-red-500 text-xs mt-1"
      in:fly={{ y: 10, duration: 300, easing: cubicOut }}
    >
      {e?.message || ""}
    </p>
{/if}
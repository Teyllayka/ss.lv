<script lang="ts">
import { run } from "svelte/legacy";

import { fly } from "svelte/transition";
import { cubicOut } from "svelte/easing";
import { capitalizeFirstLetter } from "$lib/helpers";
interface Props {
	name: any;
	type: any;
	placeholder: any;
	errors?: any | any[];
	value?: string | undefined | null;
	disableAutoFill?: boolean;
	disabled?: boolean;
}

let {
	name,
	type,
	placeholder,
	errors = [],
	value = null,
	disableAutoFill = false,
	disabled = false,
}: Props = $props();

let e: any = $state(null);
run(() => {
	e = errors.find((x: any) => x.field === name);
});
</script>

<div class="field relative">
  <input
    {type}
    {name}
    id={name}
    class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white border-solid border-2
    {e
      ? 'border-red-500 ring-red-500'
      : 'border-gray-300 dark:border-gray-600'}"
    value={value || null}
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
</div>

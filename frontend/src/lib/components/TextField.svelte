<script lang="ts">
import { run } from "svelte/legacy";

import { fly } from "svelte/transition";
import { cubicOut } from "svelte/easing";
import { capitalizeFirstLetter } from "../helpers";
interface Props {
	name: any;
	placeholder: any;
	errors?: any[];
	value?: string | undefined | null;
	disabled?: boolean;
}

let {
	name,
	placeholder,
	errors = [],
	value = null,
	disabled = false,
}: Props = $props();

let e: any = $state(null);
run(() => {
	e = errors.find((x) => x.field === name);
});
</script>

<textarea
  id={name}
  {name}
  {value}
  required
  {disabled}
  rows="4"
  class="w-full min-h-[50px] px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white border-solid border-2 {e
    ? 'border-red-500 ring-red-500'
    : 'border-gray-300 dark:border-gray-600'}"
  {placeholder}
></textarea>
<label
  for={name}
  class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
>
  {placeholder}
</label>

{#if e}
  <p
    class="text-red-500 text-xs mt-1"
    in:fly={{ y: 10, duration: 300, easing: cubicOut }}
  >
    {capitalizeFirstLetter(e?.message) || ""}
  </p>
{/if}

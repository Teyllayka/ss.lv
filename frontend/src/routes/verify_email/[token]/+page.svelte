<script lang="ts">
  import { fade } from "svelte/transition";
  import { CheckCircle, XCircle } from "lucide-svelte";
  import * as m from "$lib/paraglide/messages.js";

  export let data;
</script>

<div
  class="min-h-[56vh] grow bg-gray-100 dark:bg-gray-900 flex items-center justify-center px-4 sm:px-6 lg:px-8"
>
  <div
    class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-8 max-w-md w-full text-center"
    in:fade={{ duration: 300 }}
  >
    {#if data && data.data && data.data.verifyEmail == "Email verified"}
      <CheckCircle class="h-16 w-16 text-green-500 mx-auto mb-4" />
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        {m.email_verified_successfully()}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        {m.thank_you_for_verifying_email()}
      </p>
      <a
        href="/"
        class="inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
      >
        {m.return_to_home()}
      </a>
    {:else if data && data.errors && data.errors.length > 0}
      <XCircle class="h-16 w-16 text-red-500 mx-auto mb-4" />
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        {m.email_verification_failed()}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        {m.error_verifying_email()}
      </p>
      <a
        href="/"
        class="inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
      >
        {m.return_to_home()}
      </a>
    {:else}
      <div
        class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-500 mx-auto mb-4"
      ></div>
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        {m.verifying_email()}
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        {m.please_wait()}
      </p>
    {/if}
  </div>
</div>

<script lang="ts">
import { fade } from "svelte/transition";
import { CheckCircle, XCircle } from "lucide-svelte";

export let data;
</script>

<div
  class="min-h-[56vh] bg-gray-100 dark:bg-gray-900 flex items-center justify-center px-4 sm:px-6 lg:px-8"
>
  <div
    class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-8 max-w-md w-full text-center"
    in:fade={{ duration: 300 }}
  >
    {#if data && data.data && data.data.verifyEmail == "Email verified"}
      <CheckCircle class="h-16 w-16 text-green-500 mx-auto mb-4" />
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        Email Verified Successfully
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        Thank you for verifying your email address. Your account is now fully
        activated.
      </p>
      <a
        href="/"
        class="inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
      >
        Return to Home
      </a>
    {:else if data && data.errors && data.errors.length > 0}
      <XCircle class="h-16 w-16 text-red-500 mx-auto mb-4" />
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        Email Verification Failed
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        An error occurred while verifying your email. Please try again later.
      </p>
      <a
        href="/"
        class="inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
      >
        Return to Home
      </a>
    {:else}
      <div
        class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-500 mx-auto mb-4"
      ></div>
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
        Verifying Email
      </h1>
      <p class="text-gray-600 dark:text-gray-400 mb-6">
        Please wait while we verify your email address...
      </p>
    {/if}
  </div>
</div>

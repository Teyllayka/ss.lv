<script>
import { fade, fly } from "svelte/transition";
import { cubicOut } from "svelte/easing";

let email = "";
let isLoading = false;
let isSubmitted = false;
let error = "";

import { enhance } from "$app/forms";
export let form;
import InputField from "$lib/components/InputField.svelte";

let csrfToken = '';
</script>
  
  <div class="min-h-[58vh] bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300 flex justify-center items-center">
    <div     class="bg-white dark:bg-gray-800 shadow-lg rounded-xl p-8 w-full max-w-md transition-colors duration-300"
    in:fade={{ duration: 300 }}>
        <h2 class="text-3xl font-bold text-gray-800 dark:text-white mb-6">
       Forgot Password
      </h2>
        
        {#if !isSubmitted}
          <form  method="POST" use:enhance class="space-y-6">
            <input type="hidden" name="csrfmiddlewaretoken" value={csrfToken} />

            <div class="relative" in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}>
                <InputField
                name="email"
                type="email"
                placeholder="Email"
                errors={form?.errors || []}
                value={form?.data.email}
              />
            </div>
  
            <button 
              type="submit" 
              class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
              disabled={isLoading}
              in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}
            >
              {#if isLoading}
                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline-block" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Sending...
              {:else}
                Reset Password
              {/if}
            </button>
          </form>
        {:else}
          <div class="text-center" in:fly={{ y: 20, duration: 300, easing: cubicOut }}>
            <svg class="mx-auto h-12 w-12 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <h2 class="mt-2 text-lg font-medium text-gray-900 dark:text-white">Check your email</h2>
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
              We've sent a password reset link to {email}. Please check your inbox and follow the instructions to reset your password.
            </p>
          </div>
        {/if}
  
        <p class="mt-8 text-center text-sm text-gray-600 dark:text-gray-400" in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}>
          Remember your password? 
          <a href="/login" class="font-medium text-blue-500 dark:text-blue-400 hover:underline">
            Sign in
          </a>
        </p>
    </div>
  </div>
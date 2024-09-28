<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { enhance } from "$app/forms";
  export let form;
  import InputField from "$lib/components/InputField.svelte";
</script>


<div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col justify-center items-center p-4 transition-colors duration-300">
  <div 
    class="bg-white dark:bg-gray-800 shadow-lg rounded-xl p-8 w-full max-w-md transition-colors duration-300"
    in:fade={{ duration: 300, delay: 300 }}
    out:fade={{ duration: 300 }}
  >
    <h2 class="text-3xl font-bold text-gray-800 dark:text-white mb-6">Welcome Back</h2>
    
    <form method="POST" use:enhance class="space-y-6">
      <div class="relative" in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}>
        <input 
          type="email" 
          id="email" 
          required 
          class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white"
          placeholder="Email"
        />
        <label 
          for="email" 
          class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
        >
          Email
        </label>
      </div>

      <div class="relative" in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}>
        <input 
          type="password" 
          id="password" 
          
          required 
          class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white"
          placeholder="Password"
        />
        <label 
          for="password" 
          class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
        >
          Password
        </label>
      </div>

      <div class="flex items-center justify-between" in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}>
        <label class="flex items-center text-sm text-gray-600 dark:text-gray-400">
          <input type="checkbox" class="form-checkbox h-4 w-4 text-blue-500 transition duration-150 ease-in-out border-gray-300 dark:border-gray-600 rounded" />
          <span class="ml-2">Remember me</span>
        </label>
        <a href="/forgot-password" class="text-sm text-blue-500 dark:text-blue-400 hover:underline">Forgot password?</a>
      </div>

      <button 
        type="submit" 
        class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700 focus:ring-blue-500 focus:ring-offset-gray-200 dark:focus:ring-offset-gray-800 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
        
        in:fly={{ y: 20, duration: 300, delay: 600, easing: cubicOut }}
      >
        <!-- {#if isLoading}
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline-block" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Signing In...
        {:else} -->
          Sign In
        <!-- {/if} -->
      </button>
    </form>

    <p class="mt-8 text-center text-sm text-gray-600 dark:text-gray-400" in:fly={{ y: 20, duration: 300, delay: 700, easing: cubicOut }}>
      Don't have an account? 
      <a href="/signup" class="font-medium text-blue-500 dark:text-blue-400 hover:underline">
        Sign up now
      </a>
    </p>
  </div>
</div>

<style lang="postcss">
  :global(body) {
    overflow-x: hidden;
  }
</style>
<!-- 

<form method="POST" use:enhance>
  <InputField
    name="email"
    type="email"
    placeholder="email"
    errors={form?.errors || []}
    value={form?.data.email}
  />

  <InputField
    name="password"
    type="password"
    placeholder="password"
    errors={form?.errors || []}
  />

  <button type="submit">Login</button>
</form> -->

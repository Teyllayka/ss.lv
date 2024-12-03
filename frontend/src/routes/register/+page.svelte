<script>
import { fade, fly } from "svelte/transition";
import { cubicOut } from "svelte/easing";
import { enhance } from "$app/forms";
export let form;
import InputField from "$lib/components/InputField.svelte";

let registrationType = "user";

let isLoading = false;

let csrfToken = "";
</script>

<div
  class="min-h-[58vh] bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div
    class="max-w-md mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
    in:fade={{ duration: 300, delay: 300 }}
  >
    <div class="px-4 py-5 sm:p-6">
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center"
      >
        Create an Account
      </h1>

      <form method="POST" use:enhance class="space-y-6">
        <input type="hidden" name="csrfmiddlewaretoken" value={csrfToken} />
        <div class="flex justify-center space-x-4 mb-6">
          <button
            type="button"
            class="{registrationType === 'user'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700'} px-4 py-2 rounded-md transition-colors duration-200"
            on:click={() => (registrationType = "user")}
          >
            User
          </button>
          <button
            type="button"
            class="{registrationType === 'company'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-200 text-gray-700'} px-4 py-2 rounded-md transition-colors duration-200"
            on:click={() => (registrationType = "company")}
          >
            Company
          </button>
        </div>

        {#if registrationType === "user"}
          <div class="grid grid-cols-2 gap-4">
            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}
            >
              <InputField
                name="name"
                type="text"
                placeholder="Name"
                errors={form?.errors || []}
                value={form?.data.name}
              />
            </div>

            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}
            >
              <InputField
                name="surname"
                type="text"
                placeholder="Surname"
                errors={form?.errors || []}
                value={form?.data.surname}
              />
            </div>
          </div>
        {:else}
          <div
            class="relative"
            in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}
          >
            <InputField
              name="companyName"
              type="text"
              placeholder="Company Name"
              errors={form?.errors || []}
              value={form?.data.companyName}
            />
          </div>
        {/if}

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}
        >
          <InputField
            name="email"
            type="email"
            placeholder="Email"
            errors={form?.errors || []}
            value={form?.data.email}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 600, easing: cubicOut }}
        >
          <InputField
            name="password"
            type="password"
            placeholder="Password"
            errors={form?.errors || []}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 700, easing: cubicOut }}
        >
          <InputField
            name="repeatPassword"
            type="password"
            placeholder="Confirm Password"
            errors={form?.errors || []}
          />
        </div>

        <button
          type="submit"
          class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
          disabled={isLoading}
          in:fly={{ y: 20, duration: 300, delay: 800, easing: cubicOut }}
        >
          {#if isLoading}
            <svg
              class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline-block"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
            Registering...
          {:else}
            Register
          {/if}
        </button>
      </form>

      <p
        class="mt-8 text-center text-sm text-gray-600 dark:text-gray-400"
        in:fly={{ y: 20, duration: 300, delay: 900, easing: cubicOut }}
      >
        Already have an account?
        <a
          href="/login"
          class="font-medium text-blue-500 dark:text-blue-400 hover:underline"
        >
          Sign in
        </a>
      </p>
    </div>
  </div>
</div>

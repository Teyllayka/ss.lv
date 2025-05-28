<script lang="ts">
    import { fade } from "svelte/transition";
    import { Lock, XCircle } from "lucide-svelte";
    import { enhance } from "$app/forms";
    import * as m from "$lib/paraglide/messages.js";
    import InputField from "$lib/components/InputField.svelte";
    import { goto } from "$app/navigation";

    export let data;
    export let form;
</script>

<div
    class="min-h-[56vh] grow bg-gray-100 dark:bg-gray-900 flex items-center justify-center px-4 sm:px-6 lg:px-8"
>
    <div
        class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-8 max-w-md w-full text-center"
        in:fade={{ duration: 300 }}
    >
        {#if data?.errors && data.errors.length}
            <XCircle class="h-16 w-16 text-red-500 mx-auto mb-4" />
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
                {m.password_reset_failed()}
            </h1>
            <p class="text-gray-600 dark:text-gray-400 mb-6">
                {form?.error || m.reset_link_invalid()}
            </p>
            <a
                href="/forgot-password"
                class="inline-block bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-150 ease-in-out"
            >
                {m.request_new_reset()}
            </a>
        {:else}
            <Lock class="h-16 w-16 text-blue-500 mx-auto mb-4" />
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">
                {m.reset_your_password()}
            </h1>
            <p class="text-gray-600 dark:text-gray-400 mb-6">
                {m.enter_new_password()}
            </p>

            <form
                method="post"
                action="?/resetPassword"
                use:enhance={() => {
                    return async ({ update, result }) => {
                        console.log("Form submitted", result);
                        if (result.type === "redirect") {
                            goto(result.location);
                        }
                    };
                }}
                class="space-y-6"
            >
                <InputField
                    name="password"
                    type="password"
                    placeholder={m.new_password()}
                    errors={form?.errors || []}
                    value={form?.data?.password}
                    required
                />

                <InputField
                    name="confirmPassword"
                    type="password"
                    placeholder={m.confirm_password()}
                    errors={form?.errors || []}
                    value={form?.data?.confirmPassword}
                    required
                />

                <button
                    type="submit"
                    class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700 focus:ring-blue-500 focus:ring-offset-gray-200 dark:focus:ring-offset-gray-800 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
                >
                    {m.reset_password()}
                </button>
            </form>
        {/if}
    </div>
</div>

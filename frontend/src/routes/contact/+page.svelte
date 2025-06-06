<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { Mail, Phone, MapPin } from "lucide-svelte";
  import InputField from "$lib/components/InputField.svelte";
  import TextField from "$lib/components/TextField.svelte";
  import emailjs from "@emailjs/browser";
  import * as m from "$lib/paraglide/messages.js";

  export let form;
  let csrfToken = "";

  let isSubmitted = false;

  const serviceID = "service_399hfz4";
  const templateID = "template_b5ulg39";
  const userID = "user_HUoPQRsnR9OU761JxnBoi";

  function sendEmail(event: { preventDefault: () => void; target: any }) {
    event.preventDefault();
    emailjs
      .sendForm(serviceID, templateID, event.target, {
        publicKey: userID,
      })
      .then(
        (result) => {
          isSubmitted = true;
        },
        (error) => {
          console.error("Error sending email:", error.text);
        },
      );
  }
</script>

<div
  class="min-h-[58vh] bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div
    class="max-w-4xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
    in:fade={{ duration: 300 }}
  >
    <div class="px-4 py-5 sm:p-6">
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center"
      >
        {m.contact_us()}
      </h1>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        {#if !isSubmitted}
          <form method="post" on:submit={sendEmail} class="space-y-6">
            <input type="hidden" name="csrfmiddlewaretoken" value={csrfToken} />
            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 100, easing: cubicOut }}
            >
              <InputField
                type="text"
                name="from_name"
                placeholder={m.name()}
                errors={form?.errors || []}
              />
            </div>

            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 200, easing: cubicOut }}
            >
              <InputField
                type="email"
                name="from_email"
                placeholder={m.email()}
                errors={form?.errors || []}
              />
            </div>

            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}
            >
              <InputField
                type="text"
                name="from_category"
                placeholder={m.subject()}
                errors={form?.errors || []}
              />
            </div>

            <div
              class="relative"
              in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}
            >
              <TextField
                name="message"
                placeholder={m.message()}
                errors={form?.errors || []}
              />
            </div>

            <button
              type="submit"
              class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
              in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}
            >
              {m.send_message()}
            </button>
          </form>
        {:else}
          <div
            class="text-center"
            in:fly={{ y: 20, duration: 300, easing: cubicOut }}
          >
            <svg
              class="mx-auto h-12 w-12 text-green-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
            <h2 class="mt-2 text-lg font-medium text-gray-900 dark:text-white">
              {m.message_sent()}
            </h2>
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
              {m.thanks_for_contacting_us()}
            </p>
          </div>
        {/if}

        <div
          class="space-y-6"
          in:fly={{ x: 20, duration: 300, delay: 600, easing: cubicOut }}
        >
          <h2 class="text-2xl font-semibold text-gray-900 dark:text-white">
            {m.get_in_touch()}
          </h2>
          <p class="text-gray-600 dark:text-gray-300">
            {m.have_questions()}
          </p>
          <div class="space-y-4">
            <div
              class="flex items-center space-x-3 text-gray-700 dark:text-gray-300"
            >
              <Mail size={20} />
              <span>support@yourmarketplace.com</span>
            </div>
            <div
              class="flex items-center space-x-3 text-gray-700 dark:text-gray-300"
            >
              <Phone size={20} />
              <span>+1 (555) 123-4567</span>
            </div>
            <div
              class="flex items-center space-x-3 text-gray-700 dark:text-gray-300"
            >
              <MapPin size={20} />
              <span>123 Market Street, City, State 12345</span>
            </div>
          </div>
          <div class="mt-8">
            <h3
              class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
            >
              {m.business_hours()}
            </h3>
            <p class="text-gray-600 dark:text-gray-300">
              {@html m.working_hours()}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

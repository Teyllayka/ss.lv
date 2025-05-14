<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import {
    X,
    Star,
    Mail,
    CheckCircle,
    Edit,
    User,
    ShoppingBag,
    AlertCircle,
    Camera,
    Phone,
  } from "lucide-svelte";
  import type { PageData } from "./$houdini";
  import { renderStars } from "$lib/helpers";
  import InputField from "$lib/components/InputField.svelte";
  import * as m from "$lib/paraglide/messages.js";
  import { enhance } from "$app/forms";
  import ProfileAdvert from "$lib/components/ProfileAdvert.svelte";
  import ProfileReview from "$lib/components/ProfileReview.svelte";
  import { activeTabClass, inactiveTabClass } from "$lib/consts";
  import { user } from "$lib/userStore";
  import { goto } from "$app/navigation";

  export let form;
  export let data: PageData;

  function preventFormReset(formElement: any) {
    enhance(formElement, ({ formElement }) => {
      return async ({ result, update }) => {
        if (result.type === "success") {
          await update({ reset: false });

          const passwordField = formElement.querySelector(
            'input[name="password"]',
          ) as HTMLInputElement;
          if (passwordField) passwordField.value = "";
        } else {
          await update();
        }
      };
    });
  }

  $: me = data?.me?.data?.me || null;
  $: userData = { ...me, ...form?.data } as UserData;

  let activeTab: TabType = "profile";
  let activeReviewTab: ReviewTabType = "received";
  let activeAdvertTab: AdvertTabType = "active";

  function switchTab(tab: TabType, subTab?: ReviewTabType | AdvertTabType) {
    activeTab = tab;
    if (tab === "adverts") {
      activeAdvertTab = (subTab as AdvertTabType) || "active";
    } else if (tab === "profile") {
      activeReviewTab = (subTab as ReviewTabType) || "received";
    }
  }

  let sent: boolean = false;

  function sendVerificationEmail() {
    fetch("/me?/verify", {
      method: "POST",
      body: JSON.stringify({}),
    })
      .then((res) => res.json())
      .then((data) => {
        if (data.status == 200) {
          sent = true;
        }
      });
  }

  export async function logout() {
    const res = await fetch("/api/logout", { method: "POST" });
    if (res.ok) {
      user.logout();
      goto("/");
    } else {
      console.error("Logout failed", await res.text());
    }
  }

  let showSuccessMessage = false;
  $: showSuccessMessage = form?.success || false;

  const roleStyles: { [key: string]: string } = {
    ADMIN: "bg-indigo-100 text-indigo-800",
    MODERATOR: "bg-green-100 text-green-800",
  };

  $: filteredActiveAdverts =
    userData?.adverts?.filter((a) => a.available) || [];
  $: filteredSoldAdverts = userData?.adverts?.filter((a) => !a.available) || [];

  let newAvatarFile: File | null = null;
  let avatarPreview = userData?.avatarUrl || "";

  function handleAvatarChange(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (file) {
      newAvatarFile = file;
      avatarPreview = URL.createObjectURL(file);
    }
  }

  function removeAvatar() {
    newAvatarFile = null;
    avatarPreview = "";
  }
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    {#if me && userData}
      <div
        class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
        in:fade={{ duration: 300 }}
      >
        <div class="p-6">
          <div class="flex flex-col md:flex-row items-center mb-6">
            {#if userData.avatarUrl}
              <img
                alt="{userData.name} {userData.surname}"
                src={userData?.avatarUrl}
                class="w-32 h-32 rounded-full object-cover mb-4 md:mb-0 md:mr-6"
              />
            {/if}
            <div class="text-center md:text-left">
              <h1
                class="text-3xl font-bold text-gray-900 dark:text-white mb-2 flex items-center"
              >
                {userData.name}
                {userData.surname}
                {#if userData.role in roleStyles}
                  <span
                    class={`ml-3 inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${roleStyles[userData.role]}`}
                    style="align-self: center;"
                  >
                    {userData.role}
                  </span>
                {/if}
              </h1>
              {#if userData.advertsWithReviews.length > 0}
                <div
                  class="flex items-center justify-center md:justify-start mb-2"
                >
                  {#each renderStars(userData.rating) as star, index}
                    <Star
                      class={star.isFilled
                        ? "text-yellow-400 fill-current"
                        : "text-gray-300"}
                      size="16"
                    />
                  {/each}
                  <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
                    ({userData.rating.toFixed(1)}) ({m.based_on({
                      count: userData.advertsWithReviews.length,
                    })})
                  </span>
                </div>
              {/if}
              <div class="flex items-center justify-center md:justify-start">
                <CheckCircle class="w-5 h-5 text-green-500 mr-2" />
                <span
                  class="text-sm font-semibold text-gray-900 dark:text-white"
                >
                  {userData.adverts
                    ? userData.adverts.filter((a) => !a.available).length
                    : 0}{" "}
                  {m.finished_adverts()}
                </span>
              </div>
            </div>
          </div>

          <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "profile" ? activeTabClass : inactiveTabClass
              }`}
              on:click={() => switchTab("profile")}
            >
              <User class="w-5 h-5 mr-2" />
              {m.profile()}
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "adverts" ? activeTabClass : inactiveTabClass
              }`}
              on:click={() => switchTab("adverts")}
            >
              <ShoppingBag class="w-5 h-5 mr-2" />
              {m.adverts()}
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "edit" ? activeTabClass : inactiveTabClass
              }`}
              on:click={() => switchTab("edit")}
            >
              <Edit class="w-5 h-5 mr-2" />
              {m.edit()}
            </button>
          </div>

          {#if activeTab === "profile"}
            <div in:fade>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                {#if userData.phone}
                  <div class="flex items-center">
                    <Phone
                      class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2"
                    />
                    <span class="text-gray-700 dark:text-gray-300">
                      {userData.phone}
                    </span>
                  </div>
                {/if}
                {#if userData.email}
                  <div class="flex items-center">
                    <Mail
                      class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2"
                    />
                    <span class="text-gray-700 dark:text-gray-300">
                      {userData.email}
                    </span>
                    {#if userData.emailVerified}
                      <span
                        class="ml-2 px-2 py-1 text-xs font-semibold text-green-800 bg-green-100 rounded-full"
                        >{m.verified()}</span
                      >
                    {:else}
                      <span
                        class="ml-2 px-2 py-1 text-xs font-semibold text-red-800 bg-red-100 rounded-full"
                        >{m.not_verified()}</span
                      >
                    {/if}
                  </div>
                {/if}
                <!-- {#if userData.telegramUsername}
                  <div class="flex items-center">
                    <AtSign
                      class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2"
                    />
                    <a
                      href="https://t.me/{userData.telegramUsername}"
                      target="_blank"
                      rel="noopener noreferrer"
                      class="text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300"
                    >
                      @{userData.telegramUsername}
                    </a>
                  </div>
                {/if} -->
              </div>

              {#if !userData.emailVerified}
                <div
                  class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-4"
                  role="alert"
                >
                  <div class="flex">
                    <AlertCircle class="w-6 h-6 mr-2" />
                    <p>{m.email_not_verified()}</p>
                  </div>
                  <button
                    type="button"
                    on:click={sendVerificationEmail}
                    disabled={sent}
                    class={`mt-2 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                      sent
                        ? "text-green-700 bg-green-100 hover:bg-green-200 focus:ring-green-500"
                        : "text-yellow-700 bg-yellow-100 hover:bg-yellow-200 focus:ring-yellow-500"
                    }`}
                  >
                    <Mail class="w-4 h-4 mr-2" />
                    {sent
                      ? "Verification Email Sent"
                      : "Resend Verification Email"}
                  </button>
                </div>
              {/if}

              <!-- {#if !userData.telegramUsername}
                <div
                  class="bg-blue-100 border-l-4 border-blue-500 text-blue-700 p-4"
                  role="alert"
                >
                  <div class="flex">
                    <AtSign class="w-6 h-6 mr-2" />
                    <p>Link your Telegram account for easier communication.</p>
                  </div>
                  <button
                    type="button"
                    class="mt-2 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-blue-700 bg-blue-100 hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                  >
                    <AtSign class="w-4 h-4 mr-2" />
                    Link Telegram Account
                  </button>
                </div>
              {/if} -->

              <div class="mb-6 mt-6 flex justify-center">
                <div
                  class="inline-flex rounded-md shadow-sm"
                  role="group"
                  aria-label="Review Tabs"
                >
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === "received"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("profile", "received")}
                  >
                    {m.received_reviews()}
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === "written"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("profile", "written")}
                  >
                    {m.written_reviews()}
                  </button>
                </div>
              </div>

              {#if activeReviewTab === "received" ? userData.advertsWithReviews?.length > 0 : userData.reviewedAdverts?.length > 0}
                {#each activeReviewTab === "received" ? userData.advertsWithReviews : userData.reviewedAdverts as advert}
                  {#if advert.review}
                    <ProfileReview
                      {advert}
                      reviewer={activeReviewTab == "written"
                        ? userData
                        : advert.review.user}
                    />
                  {/if}
                {/each}
              {:else if activeReviewTab === "written" ? userData.reviewedAdverts?.length === 0 : userData.advertsWithReviews?.length === 0}
                <p class="text-gray-600 dark:text-gray-400 text-center">
                  No reviews {activeReviewTab} yet.
                </p>
              {/if}
            </div>
          {:else if activeTab === "adverts"}
            <div in:fade>
              <div class="mb-6 flex justify-center">
                <div
                  class="inline-flex rounded-md shadow-sm"
                  role="group"
                  aria-label="Advert Tabs"
                >
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "active"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("adverts", "active")}
                  >
                    {m.active_adverts()}
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "sold"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchTab("adverts", "sold")}
                  >
                    {m.sold_adverts()}
                  </button>
                </div>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {#each activeAdvertTab === "active" ? filteredActiveAdverts : filteredSoldAdverts as advert (advert.id)}
                  <ProfileAdvert {advert} />
                {/each}
              </div>

              {#if (activeAdvertTab === "active" ? filteredActiveAdverts : filteredSoldAdverts).length === 0}
                <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
                  No {activeAdvertTab} adverts to display.
                </p>
              {/if}
            </div>
          {:else if activeTab === "edit"}
            <div in:fade>
              {#if showSuccessMessage}
                <div
                  class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-4"
                  role="alert"
                  transition:fly={{ y: -20, duration: 300 }}
                >
                  <div class="flex justify-between items-center">
                    <div class="flex items-center">
                      <CheckCircle class="w-5 h-5 mr-2" />
                      <p>{m.user_updated()}</p>
                    </div>
                    <button
                      on:click={() => (showSuccessMessage = false)}
                      class="text-green-700 hover:text-green-900"
                    >
                      <X class="w-5 h-5" />
                    </button>
                  </div>
                </div>
              {/if}
              <form
                use:preventFormReset
                method="POST"
                action="?/updateProfile"
                class="space-y-6"
                enctype="multipart/form-data"
              >
                <div class="space-y-4">
                  <label
                    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
                  >
                    {m.profile_picture()}
                  </label>

                  <div class="flex items-center justify-center">
                    <label
                      for="newAvatar"
                      class="flex flex-col items-center justify-center w-32 h-32 border-2 border-gray-300 border-dashed rounded-full cursor-pointer bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 transition-colors"
                    >
                      {#if avatarPreview}
                        <img
                          src={avatarPreview}
                          alt="Avatar Preview"
                          class="w-full h-full object-cover rounded-full"
                        />
                      {:else}
                        <div class="flex flex-col items-center justify-center">
                          <Camera class="w-8 h-8 text-gray-400 mb-1" />
                          <span
                            class="text-xs text-gray-500 dark:text-gray-400 text-center"
                          >
                            Click or drop <br />to upload
                          </span>
                        </div>
                      {/if}
                      <input
                        id="newAvatar"
                        name="newAvatar"
                        type="file"
                        accept="image/*"
                        on:change={handleAvatarChange}
                        class="hidden"
                      />
                    </label>
                  </div>

                  {#if avatarPreview}
                    <div class="flex justify-center">
                      <button
                        type="button"
                        on:click={removeAvatar}
                        class="inline-flex items-center px-2 py-1 bg-red-500 text-white text-xs rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-400"
                      >
                        <X class="w-4 h-4 mr-1" />
                        {m.remove()}
                      </button>
                    </div>
                  {/if}

                  <input
                    type="hidden"
                    name="existingAvatar"
                    value={userData.avatarUrl}
                  />
                </div>

                {#if !userData.companyName}
                  <InputField
                    name="name"
                    placeholder={m.name()}
                    type="text"
                    value={userData.name}
                    errors={form?.errors || []}
                  />
                  <InputField
                    name="surname"
                    placeholder={m.surname()}
                    type="text"
                    value={userData.surname}
                    errors={form?.errors || []}
                  />
                {:else}
                  <InputField
                    name="companyName"
                    placeholder={m.company_name()}
                    type="text"
                    errors={form?.errors || []}
                    value={userData.companyName}
                  />
                {/if}
                <InputField
                  name="phone"
                  placeholder={m.phone()}
                  type="text"
                  errors={form?.errors || []}
                  value={userData.phone}
                />

                <InputField
                  name="password"
                  placeholder={m.password()}
                  type="password"
                  errors={form?.errors || []}
                  disableAutoFill={true}
                />

                <div class="flex justify-between">
                  <button
                    type="button"
                    on:click={logout}
                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                  >
                    {m.logout()}
                  </button>
                  <button
                    type="submit"
                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                  >
                    {m.save_changes()}
                  </button>
                </div>
              </form>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="text-center text-gray-600 dark:text-gray-400">
        {m.no_user_data_available()}
      </div>
    {/if}
  </div>
</div>

<style>
  .overflow-hidden::-webkit-scrollbar {
    display: none;
  }

  .overflow-hidden {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>

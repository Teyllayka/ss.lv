<script lang="ts">
import { fade, fly } from "svelte/transition";
import {
	Star,
	Phone,
	Mail,
	CheckCircle,
	Heart,
	MapPin,
	Edit,
	User,
	ShoppingBag,
	MessageSquare,
	AlertCircle,
	AtSign,
} from "lucide-svelte";
import type { PageData } from "./$houdini";
import { formatDate, renderStars } from "$lib/helpers";
import InputField from "$lib/components/InputField.svelte";
import * as m from "$lib/paraglide/messages.js";
    import { enhance } from "$app/forms";

export let data: PageData;

$: ({ me } = data);
$: userData = $me.data?.me;

let activeTab: "profile" | "adverts" | "edit" = "profile";
let activeReviewTab: "received" | "written" = "received";
let activeAdvertTab: "active" | "sold" = "active";

function switchTab(tab: "profile" | "adverts" | "edit") {
	activeTab = tab;
	if (tab === "adverts") {
		activeAdvertTab = "active";
	} else if (tab === "profile") {
		activeReviewTab = "received";
	}
}

let sent: boolean = false;

function sendVerificationEmail() {
	fetch("?/verify", {
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

function linkTelegramAccount() {}

function logout() {
	fetch("/api/logout", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({}),
	})
		.then((res) => res.json())
		.then((data) => {
			if (data.status == 200) {
				window.location.href = "/";
			}
		});
}

function switchReviewTab(tab: "received" | "written") {
	activeReviewTab = tab;
}

function switchAdvertTab(tab: "active" | "sold") {
	activeAdvertTab = tab;
}
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 transition-colors duration-300"
>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    {#if $me.fetching}
      <div class="text-center text-gray-600 dark:text-gray-400">
        Loading user data...
      </div>
    {:else if $me.errors}
      <div class="text-center text-red-500 dark:text-red-400">
        Failed to load user data.
      </div>
    {:else if $me.data && userData}
      <div
        class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
        in:fade={{ duration: 300 }}
      >
        <div class="p-6">
          <div class="flex flex-col md:flex-row items-center mb-6">
            <img
              alt="{userData.name} {userData.surname}"
              class="w-32 h-32 rounded-full object-cover mb-4 md:mb-0 md:mr-6"
            />
            <div class="text-center md:text-left">
              <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                {userData.name}
                {userData.surname}
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
                    ({userData.rating.toFixed(1)}) (based on{" "}
                    {userData.advertsWithReviews.length} reviews)
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
                  Finished Adverts
                </span>
              </div>
            </div>
          </div>

          <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "profile"
                  ? "border-b-2 border-blue-500 text-blue-500"
                  : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              }`}
              on:click={() => switchTab("profile")}
            >
              <User class="w-5 h-5 mr-2" />
              Profile
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "adverts"
                  ? "border-b-2 border-blue-500 text-blue-500"
                  : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              }`}
              on:click={() => switchTab("adverts")}
            >
              <ShoppingBag class="w-5 h-5 mr-2" />
              Adverts
            </button>
            <button
              class={`py-2 px-4 font-medium text-sm focus:outline-none flex items-center ${
                activeTab === "edit"
                  ? "border-b-2 border-blue-500 text-blue-500"
                  : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              }`}
              on:click={() => switchTab("edit")}
            >
              <Edit class="w-5 h-5 mr-2" />
              Edit
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
                        >Verified</span
                      >
                    {:else}
                      <span
                        class="ml-2 px-2 py-1 text-xs font-semibold text-red-800 bg-red-100 rounded-full"
                        >Not Verified</span
                      >
                    {/if}
                  </div>
                {/if}
                {#if userData.telegramUsername}
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
                {/if}
              </div>
              {#if !userData.emailVerified}
                <div
                  class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-2"
                  role="alert"
                >
                  <div class="flex">
                    <AlertCircle class="w-6 h-6 mr-2" />
                    <p>
                      Your email is not verified. Please check your inbox for a
                      verification email.
                    </p>
                  </div>
                </div>
              {/if}
              {#if !userData.telegramUsername}
                <div
                  class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4"
                  role="alert"
                >
                  <div class="flex">
                    <AlertCircle class="w-6 h-6 mr-2" />
                    <p>
                      Do not forget to link your Telegram account. It may be
                      useful for communication with other users.
                    </p>
                  </div>
                </div>
              {/if}

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
                    on:click={() => switchReviewTab("received")}
                  >
                    Received Reviews
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeReviewTab === "written"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchReviewTab("written")}
                  >
                    Written Reviews
                  </button>
                </div>
              </div>

              {#if activeReviewTab === "received"}
                {#if userData.advertsWithReviews && userData.advertsWithReviews.length > 0}
                  {#each userData.advertsWithReviews as advert}
                    {#if advert.review}
                      <div
                        class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden mb-4"
                        in:fade
                      >
                        <div class="p-4">
                          <div class="flex flex-col md:flex-row">
                            <div class="flex-grow">
                              <div class="flex items-start mb-2">
                                <div
                                  class="w-10 h-10 rounded-full bg-gray-300 mr-3 flex items-center justify-center"
                                >
                                  <span
                                    class="text-xl font-semibold text-gray-700"
                                    >{advert.review.user?.name?.[0] ?? ""}</span
                                  >
                                </div>
                                <div>
                                  <h3 class="text-sm font-semibold">
                                    {advert.review.user?.name}
                                  </h3>
                                  <div class="flex items-center">
                                    {#each renderStars(advert.review.rating) as star, index}
                                      <Star
                                        class={star.isFilled
                                          ? "text-yellow-400 fill-current"
                                          : "text-gray-300"}
                                        size="16"
                                      />
                                    {/each}
                                    <span class="ml-2 text-sm text-gray-500">
                                      {formatDate(
                                        advert.review.createdAt.toString()
                                      )}
                                    </span>
                                  </div>
                                </div>
                              </div>
                              <p
                                class="text-sm text-gray-600 dark:text-gray-300"
                              >
                                {advert.review.message}
                              </p>
                            </div>
                            <div class="mt-4 md:mt-0 md:ml-4 flex-shrink-0">
                              <div
                                class="bg-gray-100 dark:bg-gray-700 rounded-lg p-2 w-full md:w-48"
                              >
                                <img
                                  alt={advert.title}
                                  class="w-full h-24 object-cover mb-2 rounded"
                                />
                                <h4 class="text-sm font-semibold truncate">
                                  {advert.title}
                                </h4>
                                <p class="text-xs text-gray-500">
                                  ${advert.price.toFixed(2)}
                                </p>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    {/if}
                  {/each}
                {:else}
                  <p class="text-gray-600 dark:text-gray-400 text-center">
                    No reviews received yet.
                  </p>
                {/if}
              {:else if activeReviewTab === "written"}
                {#if userData.reviewedAdverts && userData.reviewedAdverts.length > 0}
                  {#each userData.reviewedAdverts as advert}
                    {#if advert.review}
                      <div
                        class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden mb-4"
                        in:fade
                      >
                        <div class="p-4">
                          <div class="flex flex-col md:flex-row">
                            <div class="flex-grow">
                              <div class="flex items-start mb-2">
                                <div
                                  class="w-10 h-10 rounded-full bg-gray-300 mr-3 flex items-center justify-center"
                                >
                                  <span
                                    class="text-xl font-semibold text-gray-700"
                                    >{userData.name?.[0] ?? ""}</span
                                  >
                                </div>
                                <div>
                                  <h3 class="text-sm font-semibold">
                                    {userData.name}
                                  </h3>
                                  <div class="flex items-center">
                                    {#each renderStars(advert.review.rating) as star, index}
                                      <Star
                                        class={star.isFilled
                                          ? "text-yellow-400 fill-current"
                                          : "text-gray-300"}
                                        size="16"
                                      />
                                    {/each}
                                    <span class="ml-2 text-sm text-gray-500">
                                      {formatDate(
                                        advert.review.createdAt.toString()
                                      )}
                                    </span>
                                  </div>
                                </div>
                              </div>
                              <p
                                class="text-sm text-gray-600 dark:text-gray-300"
                              >
                                {advert.review.message}
                              </p>
                            </div>
                            <div class="mt-4 md:mt-0 md:ml-4 flex-shrink-0">
                              <div
                                class="bg-gray-100 dark:bg-gray-700 rounded-lg p-2 w-full md:w-48"
                              >
                                <img
                                  alt={advert.title}
                                  class="w-full h-24 object-cover mb-2 rounded"
                                />
                                <h4 class="text-sm font-semibold truncate">
                                  {advert.title}
                                </h4>
                                <p class="text-xs text-gray-500">
                                  ${advert.price.toFixed(2)}
                                </p>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    {/if}
                  {/each}
                {:else}
                  <p class="text-gray-600 dark:text-gray-400 text-center">
                    No reviews written yet.
                  </p>
                {/if}
              {/if}
            </div>
          {:else if activeTab === "adverts"}
            <div in:fade>
              <div class="mb-6 flex justify-center">
                <div
                  class="inline-flex rounded-md shadow-sm"
                  role="group"
                  aria-label="Review Tabs"
                >
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-l-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "active"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchAdvertTab("active")}
                  >
                    Active Adverts
                  </button>
                  <button
                    type="button"
                    class={`px-4 py-2 text-sm font-medium rounded-r-lg focus:z-10 focus:outline-none ${
                      activeAdvertTab === "sold"
                        ? "bg-blue-500 text-white"
                        : "bg-white text-gray-900 hover:bg-gray-100 dark:bg-gray-700 dark:text-white dark:hover:bg-gray-600"
                    }`}
                    on:click={() => switchAdvertTab("sold")}
                  >
                    Sold Adverts
                  </button>
                </div>
              </div>

              {#if activeAdvertTab === "active"}
                <div
                  class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6"
                >
                  {#each userData.adverts.filter((a) => a.available) as advert (advert.id)}
                    <div
                      class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden"
                      in:fade
                    >
                      <img
                        alt={advert.title}
                        class="w-full h-48 object-cover"
                      />
                      <div class="p-4">
                        <h3
                          class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
                        >
                          {advert.title}
                        </h3>
                        <div class="flex justify-between items-center">
                          <p
                            class="text-xl font-bold text-gray-900 dark:text-white mb-2"
                          >
                            ${advert.price.toFixed(2)}
                          </p>
                          <span
                            class="px-2 py-1 text-xs font-semibold text-green-800 bg-green-100 rounded-full"
                            >Active</span
                          >
                        </div>
                        <div class="flex items-center mb-2">
                          <MapPin
                            class="w-4 h-4 text-gray-500 dark:text-gray-400 mr-1"
                          />
                          <span
                            class="text-sm text-gray-600 dark:text-gray-300"
                          >
                            {advert.location}
                          </span>
                        </div>
                        <p
                          class="text-sm text-gray-500 dark:text-gray-400 mb-3"
                        >
                          {formatDate(advert.createdAt.toString())}
                        </p>
                      </div>
                    </div>
                  {/each}
                </div>

                {#if userData.adverts.filter((a) => a.available).length === 0}
                  <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
                    No active adverts to display.
                  </p>
                {/if}
              {:else if activeAdvertTab === "sold"}
                <div
                  class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6"
                >
                  {#each userData.adverts.filter((a) => !a.available) as advert (advert.id)}
                    <div
                      class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden"
                      in:fade
                    >
                      <img
                        alt={advert.title}
                        class="w-full h-48 object-cover"
                      />
                      <div class="p-4">
                        <h3
                          class="text-lg font-semibold text-gray-900 dark:text-white mb-2"
                        >
                          {advert.title}
                        </h3>
                        <div class="flex justify-between items-center">
                          <p
                            class="text-xl font-bold text-gray-900 dark:text-white mb-2"
                          >
                            ${advert.price.toFixed(2)}
                          </p>
                          <span
                            class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-200 rounded-full"
                            >Sold</span
                          >
                        </div>

                        <p
                          class="text-sm text-gray-500 dark:text-gray-400 mb-1"
                        >
                          {formatDate(advert.createdAt.toString())}
                        </p>
                        {#if advert.review}
                          <div>
                            <div class="flex items-center">
                              {#each renderStars(advert.review.rating) as star, index}
                                <Star
                                  class={star.isFilled
                                    ? "text-yellow-400 fill-current"
                                    : "text-gray-300"}
                                  size="16"
                                />
                              {/each}

                              <span
                                class="ml-2 text-sm text-gray-600 dark:text-gray-400"
                                >{advert.review.rating.toFixed(1)}</span
                              >
                            </div>
                            <p
                              class="text-sm text-gray-600 dark:text-gray-300 mt-1"
                            >
                              {advert.review.message}
                            </p>
                          </div>
                        {/if}
                      </div>
                    </div>
                  {/each}
                </div>

                {#if userData.adverts.filter((a) => !a.available).length === 0}
                  <p class="text-gray-600 dark:text-gray-400 text-center mt-4">
                    No sold adverts to display.
                  </p>
                {/if}
              {/if}
            </div>
          {:else if activeTab === "edit"}
            <div in:fade>
              <form use:enhance 
              method="POST"
              action="?/updateProfile"
               class="space-y-6">
                {#if !userData.companyName}
                  <InputField
                    name="name"
                    placeholder="Name"
                    type="text"
                    value={userData.name}
                  />
                  <InputField
                    name="surname"
                    placeholder="Surname"
                    type="text"
                    value={userData.surname}
                  />
                {:else}
                  <InputField
                    name="companyName"
                    placeholder="Company Name"
                    type="text"
                    value={userData.companyName}
                  />
                {/if}
                <InputField
                  name="phone"
                  placeholder="Phone"
                  type="text"
                  value={userData.phone}
                />

                <InputField
                  name="password"
                  placeholder="Password"
                  type="password"
                  disableAutoFill={true}
                />



                {#if !userData.emailVerified}
                  <div
                    class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4"
                    role="alert"
                  >
                    <div class="flex">
                      <AlertCircle class="w-6 h-6 mr-2" />
                      <p>Your email is not verified.</p>
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

                {#if !userData.telegramUsername}
                  <div
                    class="bg-blue-100 border-l-4 border-blue-500 text-blue-700 p-4"
                    role="alert"
                  >
                    <div class="flex">
                      <AtSign class="w-6 h-6 mr-2" />
                      <p>
                        Link your Telegram account for easier communication.
                      </p>
                    </div>
                    <button
                      type="button"
                      class="mt-2 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-blue-700 bg-blue-100 hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                    >
                      <AtSign class="w-4 h-4 mr-2" />
                      Link Telegram Account
                    </button>
                  </div>
                {/if}

                <div class="flex justify-between">
                  <button
                    type="button"
                    on:click={logout}
                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                  >
                    Logout
                  </button>
                  <button
                    type="submit"
                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                  >
                    Save Changes
                  </button>
                </div>
              </form>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="text-center text-gray-600 dark:text-gray-400">
        No user data available.
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

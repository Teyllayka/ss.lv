<!-- <script lang="ts">
import type { PageData } from "./$houdini";
export let data: PageData;
import Advert from "$lib/components/Advert.svelte";

function verify() {
	fetch("?/verify", {
		method: "POST",
		body: JSON.stringify({}),
	})
		.then((res) => res.json())
		.then((data) => {
			console.log(data);
		});
}

$: ({ me } = data);

$: adverts = $me.data?.me.adverts;

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
</script>

{#if $me.fetching}
  loading...
{:else if $me.errors}
  err...
{:else}
  {JSON.stringify($me.data)}

  {#if !$me?.data?.me.emailVerified}
    <h1>Your email is not verified</h1>
    <button on:click={() => verify()}>verify</button>
  {/if}

  {#if adverts}
      {#each adverts as advert}
        <Advert
          {advert}
          userPage={true}
        />
      {/each}
  {/if}
{/if}

<button on:click={logout}>logout</button> -->

<script lang="ts">
  import {
    Star,
    Phone,
    Mail,
    CheckCircle,
    AlertCircle,
    MapPin,
    Edit,
    User,
    ShoppingBag,
    MessageSquare,
  } from "lucide-svelte";

  // This would typically come from your API or state management solution
  let userData = {
    id: 1,
    name: "John Doe",
    avatarUrl: "/placeholder.svg?height=100&width=100",
    rating: 4.7,
    reviewCount: 15,
    finishedAdvertsCount: 23,
    phone: "+1 (555) 123-4567",
    email: "john.doe@example.com",
    isEmailVerified: false,
    location: "New York, NY",
    adverts: [
      {
        id: 1,
        title: "Vintage Camera",
        price: 199.99,
        image: "/placeholder.svg?height=100&width=150",
        status: "active",
      },
      {
        id: 2,
        title: "Mountain Bike",
        price: 499.99,
        image: "/placeholder.svg?height=100&width=150",
        status: "sold",
        rating: 4.5,
        review: "Great product, fast shipping!",
      },
      {
        id: 3,
        title: "Leather Jacket",
        price: 149.99,
        image: "/placeholder.svg?height=100&width=150",
        status: "active",
      },
    ],
    receivedReviews: [
      {
        id: 1,
        reviewer: "Alice",
        rating: 5,
        comment: "Great seller! Fast shipping and item as described.",
        date: "2023-05-15",
        advert: {
          id: 1,
          title: "Vintage Camera",
          price: 199.99,
          image: "/placeholder.svg?height=100&width=150",
        },
      },
      {
        id: 2,
        reviewer: "Bob",
        rating: 4,
        comment: "Good experience overall. Would buy from again.",
        date: "2023-05-10",
        advert: {
          id: 2,
          title: "Mountain Bike",
          price: 499.99,
          image: "/placeholder.svg?height=100&width=150",
        },
      },
    ],
    writtenReviews: [
      {
        id: 3,
        reviewee: "Charlie",
        rating: 5,
        comment: "Excellent buyer! Prompt payment and great communication.",
        date: "2023-05-08",
        advert: {
          id: 3,
          title: "Leather Jacket",
          price: 149.99,
          image: "/placeholder.svg?height=100&width=150",
        },
      },
      {
        id: 4,
        reviewee: "David",
        rating: 4,
        comment: "Smooth transaction. Would sell to again.",
        date: "2023-05-03",
        advert: {
          id: 4,
          title: "Antique Clock",
          price: 299.99,
          image: "/placeholder.svg?height=100&width=150",
        },
      },
    ],
  };

  let activeTab = "profile";
  let activeReviewTab = "received";
  let activeAdvertTab = "active";

  function handleChangeInfo() {
    // Here you would typically open a modal or navigate to an edit page
    console.log("Opening edit form...");
  }

  function renderStars(rating: number) {
    const stars = Array.from({ length: 5 }, (_, i) => ({
      isFilled: i < Math.floor(rating),
    }));
    return stars;
  }
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
>
  <div
    class="max-w-4xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
  >
    <div class="p-6">
      <h2 class="text-2xl font-bold text-center mb-6">My Profile</h2>

      <div class="flex flex-col md:flex-row items-center mb-6">
        <div
          class="w-32 h-32 rounded-full overflow-hidden mb-4 md:mb-0 md:mr-6"
        >
          <img
            src={userData.avatarUrl}
            alt={userData.name}
            class="w-full h-full object-cover"
          />
        </div>
        <div class="text-center md:text-left">
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            {userData.name}
          </h1>
          <div class="flex items-center justify-center md:justify-start mb-2">
            {#each renderStars(userData.rating) as star, index}
              <Star
                class={star.isFilled
                  ? "text-yellow-400 fill-current"
                  : "text-gray-300"}
                size="16"
              />
            {/each}
            <span class="ml-2 text-sm text-gray-600 dark:text-gray-400">
              ({userData.rating.toFixed(1)}) (based on {userData.reviewCount} reviews)
            </span>
          </div>
          <div class="flex items-center justify-center md:justify-start">
            <CheckCircle class="w-5 h-5 text-green-500 mr-2" />
            <span class="text-sm font-semibold text-gray-900 dark:text-white">
              {userData.finishedAdvertsCount} Finished Adverts
            </span>
          </div>
        </div>
      </div>

      <div class="mb-6">
        <div class="flex border-b border-gray-200 dark:border-gray-700">
          <button
            class="py-2 px-4 font-medium text-sm focus:outline-none {activeTab ===
            'profile'
              ? 'border-b-2 border-blue-500 text-blue-500'
              : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
            on:click={() => (activeTab = "profile")}
          >
            <User class="w-4 h-4 inline-block mr-2" />
            Profile
          </button>
          <button
            class="py-2 px-4 font-medium text-sm focus:outline-none {activeTab ===
            'reviews'
              ? 'border-b-2 border-blue-500 text-blue-500'
              : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
            on:click={() => (activeTab = "reviews")}
          >
            <MessageSquare class="w-4 h-4 inline-block mr-2" />
            Reviews
          </button>
          <button
            class="py-2 px-4 font-medium text-sm focus:outline-none {activeTab ===
            'adverts'
              ? 'border-b-2 border-blue-500 text-blue-500'
              : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
            on:click={() => (activeTab = "adverts")}
          >
            <ShoppingBag class="w-4 h-4 inline-block mr-2" />
            Adverts
          </button>
        </div>
      </div>

      {#if activeTab === "profile"}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
          <div class="flex items-center">
            <Phone class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
            <span class="text-gray-700 dark:text-gray-300"
              >{userData.phone}</span
            >
          </div>
          <div class="flex items-center">
            <Mail class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
            <span class="text-gray-700 dark:text-gray-300"
              >{userData.email}</span
            >
            {#if userData.isEmailVerified}
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
          <div class="flex items-center">
            <MapPin class="w-5 h-5 text-gray-500 dark:text-gray-400 mr-2" />
            <span class="text-gray-700 dark:text-gray-300"
              >{userData.location}</span
            >
          </div>
        </div>

        {#if !userData.isEmailVerified}
          <div
            class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-6"
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

        <div class="flex justify-center">
          <button
            on:click={handleChangeInfo}
            class="flex items-center px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
          >
            <Edit class="w-4 h-4 mr-2" />
            Change Info
          </button>
        </div>
      {:else if activeTab === "reviews"}
        <div class="mb-6">
          <div
            class="flex justify-center border-b border-gray-200 dark:border-gray-700"
          >
            <button
              class="py-2 px-4 font-medium text-sm focus:outline-none {activeReviewTab ===
              'received'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
              on:click={() => (activeReviewTab = "received")}
            >
              Received Reviews
            </button>
            <button
              class="py-2 px-4 font-medium text-sm focus:outline-none {activeReviewTab ===
              'written'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
              on:click={() => (activeReviewTab = "written")}
            >
              Written Reviews
            </button>
          </div>
        </div>

        {#if activeReviewTab === "received"}
          {#each userData.receivedReviews as review (review.id)}
            <div
              class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden mb-4"
            >
              <div class="p-4">
                <div class="flex flex-col md:flex-row">
                  <div class="flex-grow">
                    <div class="flex items-start mb-2">
                      <div
                        class="w-10 h-10 rounded-full bg-gray-300 mr-3 flex items-center justify-center"
                      >
                        <span class="text-xl font-semibold text-gray-700"
                          >{review.reviewer[0]}</span
                        >
                      </div>
                      <div>
                        <h3 class="text-sm font-semibold">{review.reviewer}</h3>
                        <div class="flex items-center">
                          {#each renderStars(review.rating) as star, index}
                            <Star
                              class={star.isFilled
                                ? "text-yellow-400 fill-current"
                                : "text-gray-300"}
                              size="16"
                            />
                          {/each}
                          <span class="ml-2 text-sm text-gray-500"
                            >{review.date}</span
                          >
                        </div>
                      </div>
                    </div>
                    <p class="text-sm text-gray-600 dark:text-gray-300">
                      {review.comment}
                    </p>
                  </div>
                  <div class="mt-4 md:mt-0 md:ml-4 flex-shrink-0">
                    <div
                      class="bg-gray-100 dark:bg-gray-700 rounded-lg p-2 w-full md:w-48"
                    >
                      <img
                        src={review.advert.image}
                        alt={review.advert.title}
                        class="w-full h-24 object-cover mb-2 rounded"
                      />
                      <h4 class="text-sm font-semibold truncate">
                        {review.advert.title}
                      </h4>
                      <p class="text-xs text-gray-500">
                        ${review.advert.price.toFixed(2)}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        {:else}
          {#each userData.writtenReviews as review (review.id)}
            <div
              class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden mb-4"
            >
              <div class="p-4">
                <div class="flex flex-col md:flex-row">
                  <div class="flex-grow">
                    <div class="flex items-start mb-2">
                      <div
                        class="w-10 h-10 rounded-full bg-gray-300 mr-3 flex items-center justify-center"
                      >
                        <span class="text-xl font-semibold text-gray-700"
                          >{review.reviewee[0]}</span
                        >
                      </div>
                      <div>
                        <h3 class="text-sm font-semibold">{review.reviewee}</h3>
                        <div class="flex items-center">
                          {#each renderStars(review.rating) as star, index}
                            <Star
                              class={star.isFilled
                                ? "text-yellow-400 fill-current"
                                : "text-gray-300"}
                              size="16"
                            />
                          {/each}
                          <span class="ml-2 text-sm text-gray-500"
                            >{review.date}</span
                          >
                        </div>
                      </div>
                    </div>
                    <p class="text-sm text-gray-600 dark:text-gray-300">
                      {review.comment}
                    </p>
                  </div>
                  <div class="mt-4 md:mt-0 md:ml-4 flex-shrink-0">
                    <div
                      class="bg-gray-100 dark:bg-gray-700 rounded-lg p-2 w-full md:w-48"
                    >
                      <img
                        src={review.advert.image}
                        alt={review.advert.title}
                        class="w-full h-24 object-cover mb-2 rounded"
                      />
                      <h4 class="text-sm font-semibold truncate">
                        {review.advert.title}
                      </h4>
                      <p class="text-xs text-gray-500">
                        ${review.advert.price.toFixed(2)}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        {/if}
      {:else if activeTab === "adverts"}
        <div class="mb-6">
          <div
            class="flex justify-center border-b border-gray-200 dark:border-gray-700"
          >
            <button
              class="py-2 px-4 font-medium text-sm focus:outline-none {activeAdvertTab ===
              'active'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
              on:click={() => (activeAdvertTab = "active")}
            >
              Current Adverts
            </button>
            <button
              class="py-2 px-4 font-medium text-sm focus:outline-none {activeAdvertTab ===
              'sold'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200'}"
              on:click={() => (activeAdvertTab = "sold")}
            >
              Sold Adverts
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {#each userData.adverts.filter( (advert) => (activeAdvertTab === "active" ? advert.status === "active" : advert.status === "sold") ) as advert (advert.id)}
            <div
              class="bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
            >
              <div class="p-4">
                <img
                  src={advert.image}
                  alt={advert.title}
                  class="w-full h-32 object-cover mb-4 rounded"
                />
                <h3 class="font-semibold mb-2">{advert.title}</h3>
                <div class="flex justify-between items-center">
                  <span class="font-bold">${advert.price.toFixed(2)}</span>
                  {#if advert.status === "active"}
                    <span
                      class="px-2 py-1 text-xs font-semibold text-green-800 bg-green-100 rounded-full"
                      >Active</span
                    >
                  {:else}
                    <span
                      class="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-200 rounded-full"
                      >Sold</span
                    >
                  {/if}
                </div>
                {#if advert.status === "sold" && advert.rating}
                  <div class="mt-2">
                    <div class="flex items-center">
                      {#each renderStars(advert.rating) as star, index}
                        <Star
                          class={star.isFilled
                            ? "text-yellow-400 fill-current"
                            : "text-gray-300"}
                          size="16"
                        />
                      {/each}

                      <span
                        class="ml-2 text-sm text-gray-600 dark:text-gray-400"
                        >{advert.rating.toFixed(1)}</span
                      >
                    </div>
                    {#if advert.review}
                      <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
                        {advert.review}
                      </p>
                    {/if}
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<script lang="ts">
import { fade, fly } from "svelte/transition";
import { cubicOut } from "svelte/easing";
import { Camera, X } from "lucide-svelte";
import { enhance } from "$app/forms";
import InputField from "$lib/components/InputField.svelte";
import SelectField from "$lib/components/SelectField.svelte";
import TextField from "$lib/components/TextField.svelte";
export let form;

let category: keyof typeof categoryFields | "" = "";
let isLoading = false;
let errors: Record<string, string> = {};
let dynamicFields: Record<string, string> = {};
let mainPhoto: string | null = null;
let additionalPhotos: string[] = [];

const categories = [
	{ value: "electronics", label: "Electronics" },
	{ value: "vehicles", label: "Vehicles" },
	{ value: "furniture", label: "Furniture" },
	{ value: "clothing", label: "Clothing" },
];

const categoryFields = {
	electronics: [
		{ name: "brand", label: "Brand", type: "text" },
		{
			name: "condition",
			label: "Condition",
			type: "select",
			options: ["New", "Like New", "Used", "For Parts"],
		},
	],
	vehicles: [
		{ name: "make", label: "Make", type: "text" },
		{ name: "model", label: "Model", type: "text" },
		{ name: "year", label: "Year", type: "number" },
		{ name: "mileage", label: "Mileage", type: "number" },
		{
			name: "fuelType",
			label: "Fuel Type",
			type: "select",
			options: ["Petrol", "Diesel", "Electric", "Hybrid"],
		},
	],
	furniture: [
		{ name: "material", label: "Material", type: "text" },
		{
			name: "condition",
			label: "Condition",
			type: "select",
			options: ["New", "Like New", "Used", "Antique"],
		},
	],
	clothing: [
		{ name: "size", label: "Size", type: "text" },
		{
			name: "gender",
			label: "Gender",
			type: "select",
			options: ["Men", "Women", "Unisex", "Kids"],
		},
		{
			name: "condition",
			label: "Condition",
			type: "select",
			options: ["New", "Like New", "Used"],
		},
	],
};

function handleCategoryChange() {
	dynamicFields = {};
	if (category && categoryFields[category]) {
		categoryFields[category].forEach((field) => {
			dynamicFields[field.name] = "";
		});
	}
}

function handleMainPhotoChange(event: Event) {
	const file = (event.target as HTMLInputElement).files?.[0];
	if (file) {
		mainPhoto = URL.createObjectURL(file);
	}
}

function handleAdditionalPhotosChange(event: Event) {
	const files = (event.target as HTMLInputElement).files;
	if (files) {
		for (let i = 0; i < files.length; i++) {
			additionalPhotos = [...additionalPhotos, URL.createObjectURL(files[i])];
		}
	}
}

function removeAdditionalPhoto(index: number) {
	additionalPhotos = additionalPhotos.filter((_, i) => i !== index);
}
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div
    class="max-w-3xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
    in:fade={{ duration: 300 }}
  >
    <div class="px-4 py-5 sm:p-6">
      <h1
        class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center"
      >
        Create New Advert
      </h1>

      <form
        method="post"
        use:enhance
        class="space-y-6"
        enctype="multipart/form-data"
      >
        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 100, easing: cubicOut }}
        >
          <InputField
            name="title"
            type="text"
            placeholder="Title"
            errors={form?.errors || []}
            value={form?.data.title}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 200, easing: cubicOut }}
        >
          <InputField
            name="location"
            type="text"
            placeholder="Location"
            errors={form?.errors || []}
            value={form?.data.location}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 100, easing: cubicOut }}
        >
          <InputField
            name="price"
            type="number"
            placeholder="Price"
            errors={form?.errors || []}
            value={form?.data.price}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}
        >
          <TextField
            name="description"
            placeholder="Description"
            errors={form?.errors || []}
            value={form?.data.description || ""}
          />
        </div>

        <div
          class="relative"
          in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}
        >
          <!-- <SelectField
            name="category"
            placeholder="Select a category"
            onChange={handleCategoryChange}
            options={categories}
            errors={form?.errors || []}
            value={form?.data.category}
          /> -->
          <select
            id="category"
            name="category"
            bind:value={category}
            on:change={handleCategoryChange}
            required
            class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out text-gray-800 dark:text-white {errors.category
              ? 'border-red-500'
              : 'border-gray-300 dark:border-gray-600'}"
          >
            <option value="">Select a category</option>
            {#each categories as cat}
              <option value={cat.value}>{cat.label}</option>
            {/each}
          </select>
          {#if errors.category}
            <p
              class="text-red-500 text-xs mt-1"
              in:fly={{ y: 10, duration: 300, easing: cubicOut }}
            >
              {errors.category}
            </p>
          {/if}
        </div>

        {#if category && categoryFields[category]}
          <div
            class="space-y-6"
            in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}
          >
            {#each categoryFields[category] as field}
              <div class="relative">
                {#if field.type === "select"}
                  <SelectField
                    name={field.name}
                    placeholder={`Select ${field.label}`}
                    options={field.options}
                    errors={form?.errors || []}
                    value={form?.data[field.name]}
                  />
                {:else if field.type === "text" || field.type == "number"}
                  <InputField
                    name={field.name}
                    type={field.type}
                    placeholder={field.label}
                    errors={form?.errors || []}
                    value={form?.data[field.name]}
                  />
                {/if}
              </div>
            {/each}
          </div>
        {/if}

        <div
          class="space-y-4"
          in:fly={{ y: 20, duration: 300, delay: 600, easing: cubicOut }}
        >
          <div>
            <label
              for="mainPhoto"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Main Photo
            </label>
            <div class="flex items-center justify-center w-full">
              <label
                for="mainPhoto"
                class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
              >
                {#if mainPhoto}
                  <img
                    src={mainPhoto}
                    alt="main"
                    class="w-full h-full object-cover rounded-lg"
                  />
                {:else}
                  <div
                    class="flex flex-col items-center justify-center pt-5 pb-6"
                  >
                    <Camera class="w-10 h-10 mb-3 text-gray-400" />
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
                      <span class="font-semibold">Click to upload</span> or drag
                      and drop
                    </p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">
                      PNG, JPG (MAX. 800x400px)
                    </p>
                  </div>
                {/if}
                <input
                  id="mainPhoto"
                  type="file"
                  name="mainPhoto"
                  accept="image/*"
                  on:change={handleMainPhotoChange}
                  class="hidden"
                />
              </label>
            </div>
            {#if errors.mainPhoto}
              <p
                class="text-red-500 text-xs mt-1"
                in:fly={{ y: 10, duration: 300, easing: cubicOut }}
              >
                {errors.mainPhoto}
              </p>
            {/if}
          </div>

          <div>
            <label
              for="additionalPhotos"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Additional Photos
            </label>
            <div class="flex items-center justify-center w-full">
              <label
                for="additionalPhotos"
                class="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
              >
                <div
                  class="flex flex-col items-center justify-center pt-5 pb-6"
                >
                  <Camera class="w-8 h-8 mb-3 text-gray-400" />
                  <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
                    <span class="font-semibold">Click to upload</span> or drag and
                    drop
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    PNG, JPG (MAX. 800x400px)
                  </p>
                </div>
                <input
                  id="additionalPhotos"
                  type="file"
                  accept="image/*"
                  on:change={handleAdditionalPhotosChange}
                  multiple
                  class="hidden"
                />
              </label>
            </div>
          </div>

          {#if additionalPhotos.length > 0}
            <div class="grid grid-cols-3 gap-4 mt-4">
              {#each additionalPhotos as photo, index}
                <div class="relative">
                  <img
                    src={photo}
                    alt={`Additional photo ${index + 1}`}
                    class="w-full h-32 object-cover rounded-lg"
                  />
                  <button
                    type="button"
                    class="absolute top-2 right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                    on:click={() => removeAdditionalPhoto(index)}
                  >
                    <X class="w-4 h-4" />
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <button
          type="submit"
          class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
          disabled={isLoading}
          in:fly={{ y: 20, duration: 300, delay: 700, easing: cubicOut }}
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
            Creating Advert...
          {:else}
            Create Advert
          {/if}
        </button>
      </form>
    </div>
  </div>
</div>

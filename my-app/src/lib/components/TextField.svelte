<textarea 
id="description" 
bind:value={description} 
required 
rows="4"
class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white {errors.description ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
placeholder="Description"
></textarea>
<label 
for="description" 
class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
>
Description
</label>
{#if errors.description}
<p class="text-red-500 text-xs mt-1" in:fly={{ y: 10, duration: 300, easing: cubicOut }}>{errors.description}</p>
{/if}
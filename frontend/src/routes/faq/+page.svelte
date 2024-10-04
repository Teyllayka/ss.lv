<script lang="ts">
import { fade, slide } from "svelte/transition";
import { cubicOut } from "svelte/easing";

const faqs = [
	{
		question: "How do I create an account?",
		answer:
			"To create an account, click on the 'Register' button in the top right corner of the page. You'll be asked to provide some basic information such as your name, email address, and a password. Once you've filled out the form, click 'Create Account' to complete the process.",
	},
	{
		question: "How can I list an item for sale?",
		answer:
			"After logging in, click on the 'Create Ad' button in the header. Fill out the form with details about your item, including title, description, price, and photos. Once you're satisfied with your listing, click 'Post Ad' to make it live on the marketplace.",
	},
	{
		question: "Is it free to list items?",
		answer:
			"Yes, it's completely free to list items on our marketplace. We don't charge any listing fees. We only take a small commission when an item is sold.",
	},
	{
		question: "How do I contact a seller?",
		answer:
			"When viewing an item, you'll see a 'Contact Seller' button. Click this to send a message directly to the seller. You can ask questions about the item or make arrangements for purchase and pickup.",
	},
	{
		question: "What payment methods are accepted?",
		answer:
			"We support a variety of payment methods including credit/debit cards, PayPal, and bank transfers. The exact payment methods available may depend on your location and the seller's preferences.",
	},
	{
		question: "How is shipping handled?",
		answer:
			"Shipping is typically arranged between the buyer and seller. Some sellers offer shipping, while others may prefer local pickup. Always check the item description for shipping information, and feel free to ask the seller if you have any questions.",
	},
	{
		question: "What should I do if I have a problem with a purchase?",
		answer:
			"If you encounter any issues with a purchase, first try to resolve it with the seller through our messaging system. If you can't reach a resolution, please contact our customer support team, and we'll be happy to assist you.",
	},
	{
		question: "How can I change my account settings?",
		answer:
			"To change your account settings, log in and click on your profile icon in the top right corner. Select 'Settings' from the dropdown menu. Here you can update your personal information, change your password, and manage your notification preferences.",
	},
];

let openIndex = -1;

function toggleQuestion(index: number) {
	openIndex = openIndex === index ? -1 : index;
}
</script>
  
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300">
    <div class="max-w-3xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden" in:fade={{ duration: 300 }}>
      <div class="px-4 py-5 sm:p-6">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center">Frequently Asked Questions</h1>
        
        <div class="space-y-4">
          {#each faqs as faq, index}
            <div class="border-b border-gray-200 dark:border-gray-700 pb-4">
              <button
                on:click={() => toggleQuestion(index)}
                class="flex justify-between items-center w-full text-left focus:outline-none"
              >
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">{faq.question}</h2>
                <svg
                  class="w-5 h-5 text-gray-500 dark:text-gray-400 transform transition-transform duration-200 {openIndex === index ? 'rotate-180' : ''}"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              {#if openIndex === index}
                <p transition:slide={{ duration: 300, easing: cubicOut }} class="mt-2 text-gray-600 dark:text-gray-300">
                  {faq.answer}
                </p>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    </div>
  
    <div class="mt-8 text-center" in:fade={{ duration: 300, delay: 300 }}>
      <p class="text-gray-600 dark:text-gray-400">
        Can't find the answer you're looking for?
      </p>
      <a
        href="/contact"
        class="inline-block mt-2 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition duration-300 ease-in-out"
      >
        Contact Support
      </a>
    </div>
  </div>
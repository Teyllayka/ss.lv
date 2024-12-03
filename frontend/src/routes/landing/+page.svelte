<script>
import { fade, fly, scale } from "svelte/transition";
import { elasticOut } from "svelte/easing";

let showFAQ = Array(4).fill(false);

function toggleFAQ(index) {
	showFAQ[index] = !showFAQ[index];
	showFAQ = showFAQ;
}
</script>
  
  <main class="bg-gray-100 dark:bg-gray-900 text-gray-800 dark:text-gray-200">
    <!-- Hero Section -->
    <section class="bg-gradient-to-r from-purple-600 to-indigo-600 text-white py-20">
      <div class="container mx-auto px-4">
        <div class="flex flex-col md:flex-row items-center">
          <div class="md:w-1/2 mb-8 md:mb-0" in:fly="{{ y: 50, duration: 1000 }}">
            <h1 class="text-4xl md:text-6xl font-bold mb-4">Your One-Stop Marketplace</h1>
            <p class="text-xl mb-6">Find, buy, and sell anything with ease. Join our thriving community today!</p>
            <button class="bg-white text-purple-600 font-bold py-3 px-8 rounded-full hover:bg-gray-200 transition duration-300">
              Get Started
            </button>
          </div>
          <div class="md:w-1/2" in:fly="{{ x: 50, duration: 1000 }}">
            <img src="/placeholder.svg?height=400&width=600" alt="Marketplace illustration" class="rounded-lg shadow-lg">
          </div>
        </div>
      </div>
    </section>
  
    <!-- Features Section -->
    <section class="py-20">
      <div class="container mx-auto px-4">
        <h2 class="text-3xl font-bold text-center mb-12">Why Choose Our Marketplace?</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          {#each [
            { title: 'Wide Selection', description: 'Find everything you need in one place', icon: '🛍️' },
            { title: 'Secure Transactions', description: 'Shop with confidence using our secure payment system', icon: '🔒' },
            { title: 'Community-Driven', description: 'Join a thriving marketplace community', icon: '👥' }
          ] as feature, i}
            <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md" in:fly="{{ y: 50, duration: 1000, delay: i * 200 }}">
              <div class="text-4xl mb-4">{feature.icon}</div>
              <h3 class="text-xl font-semibold mb-2">{feature.title}</h3>
              <p>{feature.description}</p>
            </div>
          {/each}
        </div>
      </div>
    </section>
  
    <!-- How It Works Section -->
    <section class="bg-gray-200 dark:bg-gray-800 py-20">
      <div class="container mx-auto px-4">
        <h2 class="text-3xl font-bold text-center mb-12">How It Works</h2>
        <div class="flex flex-col md:flex-row justify-between items-center">
          {#each [
            { step: 1, title: 'Sign Up', description: 'Create your free account' },
            { step: 2, title: 'List or Browse', description: 'Add items or start shopping' },
            { step: 3, title: 'Connect', description: 'Communicate with buyers or sellers' },
            { step: 4, title: 'Transact', description: 'Complete secure transactions' }
          ] as step, i}
            <div class="text-center mb-8 md:mb-0" in:fly="{{ y: 50, duration: 1000, delay: i * 200 }}">
              <div class="bg-purple-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-xl font-bold mb-4 mx-auto">
                {step.step}
              </div>
              <h3 class="text-xl font-semibold mb-2">{step.title}</h3>
              <p>{step.description}</p>
            </div>
          {/each}
        </div>
      </div>
    </section>
  
    <!-- Testimonials Section -->
    <section class="py-20">
      <div class="container mx-auto px-4">
        <h2 class="text-3xl font-bold text-center mb-12">What Our Users Say</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {#each [
                { name: 'Alice Johnson', role: 'Buyer', quote: 'I found exactly what I was looking for at a great price!' },
                { name: 'Bob Smith', role: 'Seller', quote: 'Selling on this platform has been a breeze. Highly recommended!' },
                { name: 'Carol Davis', role: 'Buyer & Seller', quote: 'The community here is fantastic. I\'ve made great connections!' }
            ] as testimonial, i}
            <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md" in:scale="{{ duration: 1000, delay: i * 200 }}">
              <p class="mb-4 italic">"{testimonial.quote}"</p>
              <div class="flex items-center">
                <img src="/placeholder.svg?height=50&width=50" alt={testimonial.name} class="rounded-full mr-4">
                <div>
                  <h4 class="font-semibold">{testimonial.name}</h4>
                  <p class="text-sm text-gray-600 dark:text-gray-400">{testimonial.role}</p>
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </section>
  
    <!-- FAQ Section -->
    <section class="bg-gray-200 dark:bg-gray-800 py-20">
      <div class="container mx-auto px-4">
        <h2 class="text-3xl font-bold text-center mb-12">Frequently Asked Questions</h2>
        <div class="max-w-3xl mx-auto">
          {#each [
            { question: 'Is it free to join?', answer: 'Yes, creating an account is completely free.' },
            { question: 'How do I list an item?', answer: 'Simply log in, click on "Sell", and follow the easy steps to list your item.' },
            { question: 'Is my personal information safe?', answer: 'We use state-of-the-art security measures to protect your data.' },
            { question: 'What if I have issues with a transaction?', answer: 'Our customer support team is always ready to assist you with any issues.' }
          ] as faq, i}
            <div class="mb-4 bg-white dark:bg-gray-700 rounded-lg shadow-md overflow-hidden">
              <button
                class="flex justify-between items-center w-full p-4 text-left font-semibold"
                on:click={() => toggleFAQ(i)}
              >
                <span>{faq.question}</span>
                <span class="text-2xl">{showFAQ[i] ? '−' : '+'}</span>
              </button>
              {#if showFAQ[i]}
                <div class="p-4 bg-gray-100 dark:bg-gray-600" transition:fade>
                  <p>{faq.answer}</p>
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    </section>
  
    <!-- Call-to-Action Section -->
    <section class="bg-purple-600 text-white py-20">
      <div class="container mx-auto px-4 text-center">
        <h2 class="text-3xl font-bold mb-4" in:fly="{{ y: -50, duration: 1000 }}">Ready to Get Started?</h2>
        <p class="text-xl mb-8" in:fly="{{ y: -50, duration: 1000, delay: 200 }}">Join our marketplace today and experience the difference!</p>
        <button
          class="bg-white text-purple-600 font-bold py-3 px-8 rounded-full hover:bg-gray-200 transition duration-300"
          in:scale="{{ duration: 1000, delay: 400 }}"
        >
          Sign Up Now
        </button>
      </div>
    </section>
  </main>
  
  <style>
    /* Add any additional styles here if needed */
  </style>
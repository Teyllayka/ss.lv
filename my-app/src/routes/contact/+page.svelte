<script>
    import { fade, fly } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';
    import { Mail, Phone, MapPin } from 'lucide-svelte';
  
    let name = '';
    let email = '';
    let subject = '';
    let message = '';
    let isLoading = false;
    let isSubmitted = false;
    let errors = { name: '', email: '', subject: '', message: '' };
  
    function validateForm() {
      errors = { name: '', email: '', subject: '', message: '' };
      let isValid = true;
  
      if (!name.trim()) {
        errors.name = 'Name is required';
        isValid = false;
      }
  
      if (!email.trim()) {
        errors.email = 'Email is required';
        isValid = false;
      } else if (!/\S+@\S+\.\S+/.test(email)) {
        errors.email = 'Invalid email format';
        isValid = false;
      }
  
      if (!subject.trim()) {
        errors.subject = 'Subject is required';
        isValid = false;
      }
  
      if (!message.trim()) {
        errors.message = 'Message is required';
        isValid = false;
      }
  
      return isValid;
    }
  
    async function handleSubmit() {
      if (validateForm()) {
        isLoading = true;
  
        await new Promise(resolve => setTimeout(resolve, 2000));
  
        console.log('Message sent:', { name, email, subject, message });
  
        isLoading = false;
        isSubmitted = true;
      }
    }
  </script>
  
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300">
    <div class="max-w-4xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden" in:fade={{ duration: 300 }}>
      <div class="px-4 py-5 sm:p-6">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center">Contact Us</h1>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div>
            {#if !isSubmitted}
              <form on:submit|preventDefault={handleSubmit} class="space-y-6">
                <div class="relative" in:fly={{ y: 20, duration: 300, delay: 100, easing: cubicOut }}>
                  <input 
                    type="text" 
                    id="name" 
                    bind:value={name} 
                    required 
                    class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white {errors.name ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
                    placeholder="Name"
                  />
                  <label 
                    for="name" 
                    class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
                  >
                    Name
                  </label>
                  {#if errors.name}
                    <p class="text-red-500 text-xs mt-1" in:fly={{ y: 10, duration: 300, easing: cubicOut }}>{errors.name}</p>
                  {/if}
                </div>
  
                <div class="relative" in:fly={{ y: 20, duration: 300, delay: 200, easing: cubicOut }}>
                  <input 
                    type="email" 
                    id="email" 
                    bind:value={email} 
                    required 
                    class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white {errors.email ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
                    placeholder="Email"
                  />
                  <label 
                    for="email" 
                    class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
                  >
                    Email
                  </label>
                  {#if errors.email}
                    <p class="text-red-500 text-xs mt-1" in:fly={{ y: 10, duration: 300, easing: cubicOut }}>{errors.email}</p>
                  {/if}
                </div>
  
                <div class="relative" in:fly={{ y: 20, duration: 300, delay: 300, easing: cubicOut }}>
                  <input 
                    type="text" 
                    id="subject" 
                    bind:value={subject} 
                    required 
                    class="w-full px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white {errors.subject ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
                    placeholder="Subject"
                  />
                  <label 
                    for="subject" 
                    class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
                  >
                    Subject
                  </label>
                  {#if errors.subject}
                    <p class="text-red-500 text-xs mt-1" in:fly={{ y: 10, duration: 300, easing: cubicOut }}>{errors.subject}</p>
                  {/if}
                </div>
  
                <div class="relative" in:fly={{ y: 20, duration: 300, delay: 400, easing: cubicOut }}>
                  <textarea 
                    id="message" 
                    bind:value={message} 
                    required 
                    rows="4"
                    class="w-full min-h-[100px] px-4 py-3 bg-gray-100 dark:bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 transition-all duration-300 ease-in-out placeholder-transparent peer text-gray-800 dark:text-white {errors.message ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
                    placeholder="Message"
                  ></textarea>
                  <label 
                    for="message" 
                    class="absolute left-4 -top-5 text-sm text-gray-600 dark:text-gray-400 transition-all duration-300 ease-in-out peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-400 dark:peer-placeholder-shown:text-gray-500 peer-placeholder-shown:top-3 peer-focus:-top-5 peer-focus:text-sm peer-focus:text-blue-500 dark:peer-focus:text-blue-400"
                  >
                    Message
                  </label>
                  {#if errors.message}
                    <p class="text-red-500 text-xs mt-1" in:fly={{ y: 10, duration: 300, easing: cubicOut }}>{errors.message}</p>
                  {/if}
                </div>
  
                <button 
                  type="submit" 
                  class="w-full py-3 px-4 bg-blue-500 hover:bg-blue-600 focus:ring-blue-500 focus:ring-offset-blue-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-lg"
                  disabled={isLoading}
                  in:fly={{ y: 20, duration: 300, delay: 500, easing: cubicOut }}
                >
                  {#if isLoading}
                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline-block" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Sending...
                  {:else}
                    Send Message
                  {/if}
                </button>
              </form>
            {:else}
              <div class="text-center" in:fly={{ y: 20, duration: 300, easing: cubicOut }}>
                <svg class="mx-auto h-12 w-12 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <h2 class="mt-2 text-lg font-medium text-gray-900 dark:text-white">Message Sent!</h2>
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  Thank you for contacting us. We'll get back to you as soon as possible.
                </p>
              </div>
            {/if}
          </div>
  
          <div class="space-y-6" in:fly={{ x: 20, duration: 300, delay: 600, easing: cubicOut }}>
            <h2 class="text-2xl font-semibold text-gray-900 dark:text-white">Get in Touch</h2>
            <p class="text-gray-600 dark:text-gray-300">
              Have questions or need assistance? We're here to help! Feel free to reach out to us using the contact information below or by filling out the form.
            </p>
            <div class="space-y-4">
              <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300">
                <Mail size={20} />
                <span>support@yourmarketplace.com</span>
              </div>
              <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300">
                <Phone size={20} />
                <span>+1 (555) 123-4567</span>
              </div>
              <div class="flex items-center space-x-3 text-gray-700 dark:text-gray-300">
                <MapPin size={20} />
                <span>123 Market Street, City, State 12345</span>
              </div>
            </div>
            <div class="mt-8">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">Business Hours</h3>
              <p class="text-gray-600 dark:text-gray-300">
                Monday - Friday: 9:00 AM - 5:00 PM<br>
                Saturday: 10:00 AM - 2:00 PM<br>
                Sunday: Closed
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
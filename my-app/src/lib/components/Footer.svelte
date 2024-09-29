<script>
  import { Facebook, Twitter, Instagram, Github } from "lucide-svelte";
</script>

<footer class="bg-white dark:bg-gray-800 transition-colors duration-300">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
      <div class="space-y-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Adee
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-300">
          Your one-stop shop for everything you need.
        </p>
      </div>

      <div>
        <h4
          class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
        >
          Categories
        </h4>
        <ul class="space-y-2">
          {#each ["Electronics", "Clothing", "Home & Garden", "Sports"] as category}
            <li>
              <a
                href="#{category.toLowerCase().replace(' & ', '-')}"
                class="text-sm text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
              >
                {category}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <div>
        <h4
          class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
        >
          Customer Service
        </h4>
        <ul class="space-y-2">
          {#each ["Contact", "FAQ", "Shipping", "Returns"] as item}
            <li>
              <a
                href="{item.toLowerCase().replace(' ', '-')}"
                class="text-sm text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
              >
                {item}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <div>
        <h4
          class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
        >
          Follow Us
        </h4>
        <div class="flex space-x-4">
          <a
            href="#facebook"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">Facebook</span>
            <Facebook class="h-6 w-6" />
          </a>
          <a
            href="#twitter"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">Twitter</span>
            <Twitter class="h-6 w-6" />
          </a>
          <a
            href="#instagram"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">Instagram</span>
            <Instagram class="h-6 w-6" />
          </a>
          <a
            href="#github"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">GitHub</span>
            <Github class="h-6 w-6" />
          </a>
        </div>
      </div>
    </div>

    <div
      class="mt-8 border-t border-gray-200 dark:border-gray-700 pt-8 flex flex-col md:flex-row justify-between items-center"
    >
      <p class="text-sm text-gray-500 dark:text-gray-400">
        © {new Date().getFullYear()} Adee. All rights reserved.
      </p>
      <nav class="flex space-x-4 mt-4 md:mt-0">
        {#each ["Privacy Policy", "Terms of Service", "Cookie Policy"] as item}
          <a
            href="/{item.toLowerCase().replace(/\s+/g, '-')}"
            class="text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
          >
            {item}
          </a>
        {/each}
      </nav>
    </div>
  </div>
</footer>

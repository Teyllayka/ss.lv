<script>
  import { Facebook, Twitter, Instagram, Github } from "lucide-svelte";
  import * as m from "$lib/paraglide/messages.js";

  let footerLinks = [
    { name: m.contact(), href: "/contact" },
    { name: m.faq_footer(), href: "/faq" },
    { name: m.shipping(), href: "/shipping" },
  ];

  let pages = [
    { name: m.privacy_policy(), href: "/privacy-policy" },
    { name: m.terms_of_service(), href: "/terms-of-service" },
    { name: m.cookie_policy(), href: "/cookie-policy" },
  ];
</script>

<footer class="bg-white dark:bg-gray-800 transition-colors duration-300">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
      <div class="space-y-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Adee
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-300">
          {m.slogan()}
        </p>
      </div>

      <div>
        <h4
          class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
        >
          {m.customer_service()}
        </h4>
        <ul class="space-y-2">
          {#each footerLinks as link}
            <li>
              <a
                href={link.href}
                class="text-sm text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
              >
                {link.name}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <div>
        <h4
          class="text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider mb-4"
        >
          {m.follow_us()}
        </h4>
        <div class="flex space-x-4">
          <a
            href="#facebook"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">{m.facebook()}</span>
            <Facebook class="h-6 w-6" />
          </a>
          <a
            href="#twitter"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">{m.twitter()}</span>
            <Twitter class="h-6 w-6" />
          </a>
          <a
            href="#instagram"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">{m.instagram()}</span>
            <Instagram class="h-6 w-6" />
          </a>
          <a
            href="https://github.com/Teyllayka/ss.lv"
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
          >
            <span class="sr-only">{m.github()}</span>
            <Github class="h-6 w-6" />
          </a>
        </div>
      </div>
    </div>

    <div
      class="mt-8 border-t border-gray-200 dark:border-gray-700 pt-8 flex flex-col md:flex-row justify-between items-center"
    >
      <p class="text-sm text-gray-500 dark:text-gray-400">
        © {new Date().getFullYear()} Adee. {m.all_rights_reserved()}
      </p>
      <nav class="flex space-x-4 mt-4 md:mt-0">
        {#each pages as page}
          <a
            href={page.href}
            class="text-sm text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-200"
            >{page.name}</a
          >
        {/each}
      </nav>
    </div>
  </div>
</footer>

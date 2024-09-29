<script>
  import { fade } from "svelte/transition";
</script>

<div
  class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8 transition-colors duration-300"
>
  <div
    class="max-w-3xl mx-auto bg-white dark:bg-gray-800 shadow-md rounded-lg overflow-hidden"
    in:fade={{ duration: 300 }}
  >
    <div class="px-4 py-5 sm:p-6">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
        Cookie Policy
      </h1>

      <div class="space-y-6 text-gray-700 dark:text-gray-300">
        <p>Last updated: {new Date().toLocaleDateString()}</p>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            1. Introduction
          </h2>
          <p>
            This Cookie Policy explains how [Your Marketplace Name] ("we", "us",
            or "our") uses cookies and similar technologies to recognize you
            when you visit our website at [Your Website URL] ("Website"). It
            explains what these technologies are and why we use them, as well as
            your rights to control our use of them.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            2. What are cookies?
          </h2>
          <p>
            Cookies are small data files that are placed on your computer or
            mobile device when you visit a website. Cookies are widely used by
            website owners in order to make their websites work, or to work more
            efficiently, as well as to provide reporting information.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            3. Why do we use cookies?
          </h2>
          <p>
            We use first party and third party cookies for several reasons. Some
            cookies are required for technical reasons in order for our Website
            to operate, and we refer to these as "essential" or "strictly
            necessary" cookies. Other cookies enable us to track and target the
            interests of our users to enhance the experience on our Website.
            Third parties serve cookies through our Website for advertising,
            analytics and other purposes.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            4. Types of cookies we use
          </h2>
          <p>
            The specific types of first and third party cookies served through
            our Website and the purposes they perform are described below:
          </p>
          <ul class="list-disc pl-5 mt-2 space-y-2">
            <li>
              <strong>Essential website cookies:</strong> These cookies are strictly
              necessary to provide you with services available through our Website
              and to use some of its features, such as access to secure areas.
            </li>
            <li>
              <strong>Performance and functionality cookies:</strong> These cookies
              are used to enhance the performance and functionality of our Website
              but are non-essential to their use. However, without these cookies,
              certain functionality may become unavailable.
            </li>
            <li>
              <strong>Analytics and customization cookies:</strong> These cookies
              collect information that is used either in aggregate form to help us
              understand how our Website is being used or how effective our marketing
              campaigns are, or to help us customize our Website for you.
            </li>
            <li>
              <strong>Advertising cookies:</strong> These cookies are used to make
              advertising messages more relevant to you. They perform functions like
              preventing the same ad from continuously reappearing, ensuring that
              ads are properly displayed for advertisers, and in some cases selecting
              advertisements that are based on your interests.
            </li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            5. How can you control cookies?
          </h2>
          <p>
            You have the right to decide whether to accept or reject cookies.
            You can exercise your cookie preferences by clicking on the
            appropriate opt-out links provided in the cookie banner or privacy
            settings on our Website.
          </p>
          <p class="mt-2">
            You can also set or amend your web browser controls to accept or
            refuse cookies. If you choose to reject cookies, you may still use
            our Website though your access to some functionality and areas of
            our Website may be restricted. As the means by which you can refuse
            cookies through your web browser controls vary from
            browser-to-browser, you should visit your browser's help menu for
            more information.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            6. How often will we update this Cookie Policy?
          </h2>
          <p>
            We may update this Cookie Policy from time to time in order to
            reflect, for example, changes to the cookies we use or for other
            operational, legal or regulatory reasons. Please therefore re-visit
            this Cookie Policy regularly to stay informed about our use of
            cookies and related technologies.
          </p>
          <p class="mt-2">
            The date at the top of this Cookie Policy indicates when it was last
            updated.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            7. Where can you get further information?
          </h2>
          <p>
            If you have any questions about our use of cookies or other
            technologies, please email us at [Your Contact Email].
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            8. Contact Us
          </h2>
          <p>
            If you have any questions about this Cookie Policy, please contact
            us at:
          </p>
          <p class="mt-2">
            [Your Marketplace Name]<br />
            [Address]<br />
            [Email]<br />
            [Phone]
          </p>
        </section>
      </div>
    </div>
  </div>
</div>

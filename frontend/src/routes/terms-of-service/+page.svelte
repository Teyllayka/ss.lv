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
        Terms of Service
      </h1>

      <div class="space-y-6 text-gray-700 dark:text-gray-300">
        <p>Last updated: {new Date().toLocaleDateString()}</p>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            1. Acceptance of Terms
          </h2>
          <p>
            By accessing or using [Your Marketplace Name] (the "Service"), you
            agree to be bound by these Terms of Service ("Terms"). If you
            disagree with any part of the terms, you may not access the Service.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            2. Description of Service
          </h2>
          <p>
            [Your Marketplace Name] is an online platform that allows users to
            buy and sell goods and services. We do not own, sell, resell,
            furnish, provide, or transfer any of the products or services listed
            on our platform.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            3. User Accounts
          </h2>
          <p>
            When you create an account with us, you must provide accurate,
            complete, and current information. Failure to do so constitutes a
            breach of the Terms, which may result in immediate termination of
            your account on our Service.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            4. User Responsibilities
          </h2>
          <p>
            You are responsible for safeguarding the password you use to access
            the Service and for any activities or actions under your password.
            You agree not to disclose your password to any third party. You must
            notify us immediately upon becoming aware of any breach of security
            or unauthorized use of your account.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            5. Content
          </h2>
          <p>
            Our Service allows you to post, link, store, share and otherwise
            make available certain information, text, graphics, videos, or other
            material ("Content"). You are responsible for the Content that you
            post on or through the Service, including its legality, reliability,
            and appropriateness.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            6. Prohibited Uses
          </h2>
          <p>
            You may use the Service only for lawful purposes and in accordance
            with these Terms. You agree not to use the Service:
          </p>
          <ul class="list-disc pl-5 mt-2">
            <li>
              In any way that violates any applicable national or international
              law or regulation.
            </li>
            <li>
              To transmit, or procure the sending of, any advertising or
              promotional material, including any "junk mail", "chain letter,"
              "spam," or any other similar solicitation.
            </li>
            <li>
              To impersonate or attempt to impersonate the Company, a Company
              employee, another user, or any other person or entity.
            </li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            7. Intellectual Property
          </h2>
          <p>
            The Service and its original content (excluding Content provided by
            users), features, and functionality are and will remain the
            exclusive property of [Your Marketplace Name] and its licensors. The
            Service is protected by copyright, trademark, and other laws of both
            the [Your Country] and foreign countries. Our trademarks and trade
            dress may not be used in connection with any product or service
            without the prior written consent of [Your Marketplace Name].
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            8. Termination
          </h2>
          <p>
            We may terminate or suspend your account immediately, without prior
            notice or liability, for any reason whatsoever, including without
            limitation if you breach the Terms. Upon termination, your right to
            use the Service will immediately cease.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            9. Limitation of Liability
          </h2>
          <p>
            In no event shall [Your Marketplace Name], nor its directors,
            employees, partners, agents, suppliers, or affiliates, be liable for
            any indirect, incidental, special, consequential or punitive
            damages, including without limitation, loss of profits, data, use,
            goodwill, or other intangible losses, resulting from your access to
            or use of or inability to access or use the Service.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            10. Changes
          </h2>
          <p>
            We reserve the right, at our sole discretion, to modify or replace
            these Terms at any time. What constitutes a material change will be
            determined at our sole discretion. By continuing to access or use
            our Service after those revisions become effective, you agree to be
            bound by the revised terms.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            11. Contact Us
          </h2>
          <p>
            If you have any questions about these Terms, please contact us at:
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

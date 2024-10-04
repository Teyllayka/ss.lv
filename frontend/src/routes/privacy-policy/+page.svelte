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
        Privacy Policy
      </h1>

      <div class="space-y-6 text-gray-700 dark:text-gray-300">
        <p>Last updated: {new Date().toLocaleDateString()}</p>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            1. Introduction
          </h2>
          <p>
            Welcome to [Your Marketplace Name] ("we," "our," or "us"). We are
            committed to protecting your personal information and your right to
            privacy. This Privacy Policy explains how we collect, use, disclose,
            and safeguard your information when you visit our website or use our
            services.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            2. Information We Collect
          </h2>
          <p>
            We collect personal information that you provide to us, such as:
          </p>
          <ul class="list-disc pl-5 mt-2">
            <li>Name, email address, and contact information</li>
            <li>Shipping address</li>
            <li>
              Information you provide in product reviews or communications with
              us
            </li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            3. How We Use Your Information
          </h2>
          <p>We use your information to:</p>
          <ul class="list-disc pl-5 mt-2">
            <li>Provide, maintain, and improve our services</li>
            <li>Process transactions and send transaction notifications</li>
            <li>Respond to your comments, questions, and requests</li>
            <li>Send you marketing communications</li>
            <li>Monitor and analyze trends, usage, and activities</li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            4. Sharing of Information
          </h2>
          <p>We may share your personal information with:</p>
          <ul class="list-disc pl-5 mt-2">
            <li>Service providers who perform services on our behalf</li>
            <li>
              Business partners with whom we jointly offer products or services
            </li>
            <li>
              Law enforcement or other government entities as required by law
            </li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            5. Your Rights and Choices
          </h2>
          <p>You have the right to:</p>
          <ul class="list-disc pl-5 mt-2">
            <li>Access, update, or delete your personal information</li>
            <li>Opt-out of marketing communications</li>
            <li>Object to the processing of your personal information</li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            6. Security
          </h2>
          <p>
            We implement appropriate technical and organizational measures to
            protect your personal information against unauthorized or unlawful
            processing, accidental loss, destruction, or damage.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            7. Changes to This Privacy Policy
          </h2>
          <p>
            We may update this Privacy Policy from time to time. We will notify
            you of any changes by posting the new Privacy Policy on this page
            and updating the "Last updated" date.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
            8. Contact Us
          </h2>
          <p>
            If you have any questions about this Privacy Policy, please contact
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

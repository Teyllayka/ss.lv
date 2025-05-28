<script lang="ts">
  import { ParaglideJS } from "@inlang/paraglide-sveltekit";
  import { i18n } from "$lib/i18n";
  import "../app.css";
  import type { LayoutData } from "./$houdini";
  import { user } from "$lib/userStore";
  import Footer from "$lib/components/Footer.svelte";
  import Header from "$lib/components/Header.svelte";
  import { onMount, setContext } from "svelte";
  import { writable } from "svelte/store";
  import { page } from "$app/stores";

  export let data: LayoutData;

  const areUnreadMessages = writable({
    unreadMessages: 0,
  });

  const location = writable([0, 0]);

  let me;

  $: me = data.HeaderMe.data?.me;

  $: if (me) {
    user.set({
      emailVerified: me.emailVerified || false,
      isCompany: me.companyName != null,
      isLoggedIn: true,
      role: me.role,
      id: me.id,
      banned: me.banned,
    });
  }

  onMount(() => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (pos) => {
          location.set([pos.coords.latitude, pos.coords.longitude]);
        },
        () => location.set([0, 0]),
      );
    }

    const storedTheme = localStorage.getItem("theme");
    let isDarkMode = false;
    if (storedTheme === "dark") {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    }
  });

  setContext("location", location);
  setContext("areUnreadMessages", areUnreadMessages);
</script>

<ParaglideJS {i18n}>
  <Header />

  <slot />

  {#if !$page.url.pathname.includes("/chats/")}
    <Footer />
  {/if}
</ParaglideJS>

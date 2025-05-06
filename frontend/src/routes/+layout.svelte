<script lang="ts">
  import { ParaglideJS } from "@inlang/paraglide-sveltekit";
  import { i18n } from "$lib/i18n";
  import "../app.css";
  import type { LayoutData } from "./$houdini";
  import { user } from "$lib/userStore";
  import Footer from "$lib/components/Footer.svelte";
  import Header from "$lib/components/Header.svelte";
  import { onMount, onDestroy, setContext } from "svelte";
  import { writable } from "svelte/store";
  import { page } from "$app/stores";
  import { invalidateAll, replaceState } from "$app/navigation";

  export let data: LayoutData;
  $: HeaderMe = data.HeaderMe;

  const region = writable("Select Region");
  const location = writable([0, 0]);
  let isInvalidating = false;

  $: if (HeaderMe && $HeaderMe.data && $HeaderMe.data.me) {
    user.set({
      emailVerified: $HeaderMe.data.me.emailVerified || false,
      isCompany: $HeaderMe.data.me.companyName != null,
      isLoggedIn: $HeaderMe.data.me != null,
      role: $HeaderMe.data.me.role,
      id: $HeaderMe.data.me.id,
    });
  }

  onMount(() => {
    const unsubscribe = page.subscribe(async ($page) => {
      const refetchParam = $page.url.searchParams.get("refetch");
      if (refetchParam === "true" && !isInvalidating) {
        isInvalidating = true;

        const url = new URL(window.location.href);
        url.searchParams.delete("refetch");
        replaceState(url, {});

        await invalidateAll();
        console.log("Invalidating all data...");

        isInvalidating = false;
      }
    });

    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (pos) => {
          location.set([pos.coords.latitude, pos.coords.longitude]);
          console.log("Location set to: ", pos.coords);
        },
        () => location.set([0, 0]),
      );
    }

    const storedRegion = localStorage.getItem("region");
    region.set(storedRegion || "Select Region");
    region.subscribe((value) => localStorage.setItem("region", value));

    const storedTheme = localStorage.getItem("theme");
    let isDarkMode = false;
    if (storedTheme === "dark") {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    }

    return unsubscribe;
  });

  setContext("region", region);
  setContext("location", location);

  // Theme toggling function.
  let isDarkMode = false;
  function toggleTheme() {
    isDarkMode = !isDarkMode;
    if (isDarkMode) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  }
</script>

<ParaglideJS {i18n}>
  <Header />

  <slot />

  <Footer />
</ParaglideJS>

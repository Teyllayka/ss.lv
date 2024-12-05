<script lang="ts">
  import { ParaglideJS } from "@inlang/paraglide-sveltekit";
  import { i18n } from "$lib/i18n";
  import "../app.css";
  import type { LayoutData } from "./$houdini";
  import { user } from "$lib/userStore";
  import Footer from "$lib/components/Footer.svelte";
  import { onMount, setContext } from "svelte";
  import Header from "$lib/components/Header.svelte";
  import { writable } from "svelte/store";
  export let data: LayoutData;

  $: HeaderMe = data.HeaderMe;

  const region = writable("Select Region");

  const location = writable([0, 0]);

  onMount(() => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (pos) => {
          location.set([pos.coords.latitude, pos.coords.longitude]);
        },
        () => {
          location.set([0, 0]);
        }
      );
    }
  });

  setContext("region", region);
  setContext("location", location);

  $: if (HeaderMe && $HeaderMe.data && $HeaderMe.data.me) {
    user.set({
      emailVerified: $HeaderMe.data.me.emailVerified || false,
      isCompany: $HeaderMe.data.me.companyName != null,
      isLoggedIn: $HeaderMe.data.me != null,
      role: $HeaderMe.data.me.role,
      id: $HeaderMe.data.me.id,
    });
  }

  let isDarkMode = false;

  onMount(() => {
    const storedRegion = localStorage.getItem("region");
    region.set(storedRegion || "Select Region");

    region.subscribe((value) => {
      localStorage.setItem("region", value);
    });

    const storedTheme = localStorage.getItem("theme");
    if (storedTheme === "dark") {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    }
  });

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

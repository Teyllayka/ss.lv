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

  setContext("region", region);

  $: user.set({
    emailVerified: $HeaderMe.data?.me.emailVerified || false,
    isCompany: $HeaderMe.data?.me.companyName != null,
    isLogedIn: $HeaderMe.data?.me != null,
  });

  let isDarkMode = false;

  onMount(() => {
    if (localStorage.getItem("theme") === "dark") {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    } else {
      isDarkMode = false;
      document.documentElement.classList.remove("dark");
    }

    region.set(localStorage.getItem("region") || "Select Region");
    region.subscribe((value) => {
      localStorage.setItem("region", value);
    });
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

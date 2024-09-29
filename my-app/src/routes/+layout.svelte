<script lang="ts">
  import { goto } from "$app/navigation";
  import "../app.css";
  import { writable } from "svelte/store";
  import type { LayoutData } from "./$houdini";
  import { user } from "$lib/userStore";
  import Footer from "$lib/components/Footer.svelte";
  import { onMount } from "svelte";
  import Header from "$lib/components/Header.svelte";
  export let data: LayoutData;

  $: ({ HeaderMe } = data);

  $: user.set({
    emailVerified: $HeaderMe.data?.me.emailVerified || false,
  });

  let isDarkMode = false;

  // On component mount, check for user's preferred theme
  onMount(() => {
    if (localStorage.getItem("theme") === "dark") {
      isDarkMode = true;
      document.documentElement.classList.add("dark");
    } else {
      isDarkMode = false;
      document.documentElement.classList.remove("dark");
    }
  });

  // Function to toggle theme
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

<Header />

<slot />

<Footer />

<style lang="scss">
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-direction: row;
    margin: 36px 0px;

    button {
      border: none;
      outline: none;
      background-color: transparent;
    }

    button.login {
      background: #42d392;
      border-radius: 8px;
      font-size: 18px;
      font-weight: 500;
      line-height: 21.78px;
      text-align: left;
      color: white;
      border: none;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 6px 12px;
      box-sizing: border-box;
    }

    .search {
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 12px 24px;
      box-sizing: border-box;
      background-color: #e6e6e6;
      gap: 18px;
      border-radius: 8px;

      input {
        background-color: #e6e6e6;
        width: 500px;
        border: none;
        outline: none;
        font-size: 18px;
        font-weight: 400;
        line-height: 21.78px;
        font-family: "Inter", sans-serif;
        text-align: left;
      }
    }

    .right {
      display: flex;
      justify-content: center;
      align-items: center;
      flex-direction: row;
      gap: 28px;

      p {
        font-size: 18px;
        font-weight: 400;
        line-height: 21.78px;
        font-family: "Inter", sans-serif;
        text-align: left;
      }
    }

    .favorite {
      display: flex;
      justify-content: center;
      align-items: center;
      flex-direction: column;
    }

    img {
      border: 2px solid #e6e6e6;
      border-radius: 50%;
      width: 50px;
      height: 50px;
    }

    .left {
      display: flex;
      justify-content: center;
      align-items: center;
    }

    a {
      font-size: 18px;
      font-family: "Inter", sans-serif;
      font-weight: 500;
      line-height: 21.78px;
      text-align: left;
      color: #000000;
      text-decoration: none;
      margin-right: 20px;

      &:first-child {
        font-size: 36px;
        font-weight: 700;
        line-height: 43.57px;
        text-align: left;
        font-family: "Inter", sans-serif;
        color: #000000;
      }
    }
  }
</style>

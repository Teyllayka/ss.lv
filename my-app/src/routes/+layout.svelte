<script lang="ts">
  import { goto } from "$app/navigation";
  import "../app.css";
  import { writable } from "svelte/store";
  import type { LayoutData } from "./$houdini";
  import { user } from "$lib/userStore";
  import Footer from "$lib/components/Footer.svelte";
  import { onMount } from "svelte";
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

<header>
  <div class="left">
    <a href="/">Adee</a>
    <a href="/">Categories</a>
  </div>
  <div class="search">
    <svg
      width="25"
      height="24"
      viewBox="0 0 25 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M10 16C8.18333 16 6.646 15.3707 5.388 14.112C4.13 12.8533 3.50067 11.316 3.5 9.5C3.49933 7.684 4.12867 6.14667 5.388 4.888C6.64733 3.62933 8.18467 3 10 3C11.8153 3 13.353 3.62933 14.613 4.888C15.873 6.14667 16.502 7.684 16.5 9.5C16.5 10.2333 16.3833 10.925 16.15 11.575C15.9167 12.225 15.6 12.8 15.2 13.3L20.8 18.9C20.9833 19.0833 21.075 19.3167 21.075 19.6C21.075 19.8833 20.9833 20.1167 20.8 20.3C20.6167 20.4833 20.3833 20.575 20.1 20.575C19.8167 20.575 19.5833 20.4833 19.4 20.3L13.8 14.7C13.3 15.1 12.725 15.4167 12.075 15.65C11.425 15.8833 10.7333 16 10 16ZM10 14C11.25 14 12.3127 13.5627 13.188 12.688C14.0633 11.8133 14.5007 10.7507 14.5 9.5C14.4993 8.24933 14.062 7.187 13.188 6.313C12.314 5.439 11.2513 5.00133 10 5C8.74867 4.99867 7.68633 5.43633 6.813 6.313C5.93967 7.18967 5.502 8.252 5.5 9.5C5.498 10.748 5.93567 11.8107 6.813 12.688C7.69033 13.5653 8.75267 14.0027 10 14Z"
        fill="#B3B3B3"
      />
    </svg>
    <input type="text" name="" id="" placeholder="Search" />
  </div>
  <div class="right">
    {#if !$HeaderMe.data}
      <button class="login" on:click={() => goto("/login")}
        >Login <svg
          width="36"
          height="36"
          viewBox="0 0 36 36"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M19.5 31.5C19.075 31.5 18.719 31.356 18.432 31.068C18.145 30.78 18.001 30.424 18 30C17.999 29.576 18.143 29.22 18.432 28.932C18.721 28.644 19.077 28.5 19.5 28.5H28.5V7.5H19.5C19.075 7.5 18.719 7.356 18.432 7.068C18.145 6.78 18.001 6.424 18 6C17.999 5.576 18.143 5.22 18.432 4.932C18.721 4.644 19.077 4.5 19.5 4.5H28.5C29.325 4.5 30.0315 4.794 30.6195 5.382C31.2075 5.97 31.501 6.676 31.5 7.5V28.5C31.5 29.325 31.2065 30.0315 30.6195 30.6195C30.0325 31.2075 29.326 31.501 28.5 31.5H19.5ZM16.7625 19.5H6.00001C5.57501 19.5 5.21901 19.356 4.93201 19.068C4.64501 18.78 4.50101 18.424 4.50001 18C4.49901 17.576 4.64301 17.22 4.93201 16.932C5.22101 16.644 5.57701 16.5 6.00001 16.5H16.7625L13.95 13.6875C13.675 13.4125 13.5375 13.075 13.5375 12.675C13.5375 12.275 13.675 11.925 13.95 11.625C14.225 11.325 14.575 11.1685 15 11.1555C15.425 11.1425 15.7875 11.2865 16.0875 11.5875L21.45 16.95C21.75 17.25 21.9 17.6 21.9 18C21.9 18.4 21.75 18.75 21.45 19.05L16.0875 24.4125C15.7875 24.7125 15.4315 24.8565 15.0195 24.8445C14.6075 24.8325 14.251 24.676 13.95 24.375C13.675 24.075 13.544 23.719 13.557 23.307C13.57 22.895 13.7135 22.551 13.9875 22.275L16.7625 19.5Z"
            fill="white"
          />
        </svg>
      </button>
    {:else}
      <button class="favorite" on:click={() => goto("/favorites")}>
        <svg
          width="25"
          height="24"
          viewBox="0 0 25 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M12.5 20.325C12.2667 20.325 12.0293 20.2833 11.788 20.2C11.5467 20.1167 11.334 19.9833 11.15 19.8L9.425 18.225C7.65833 16.6083 6.06233 15.0043 4.637 13.413C3.21167 11.8217 2.49933 10.0673 2.5 8.14999C2.5 6.58333 3.025 5.27499 4.075 4.22499C5.125 3.17499 6.43333 2.64999 8 2.64999C8.88333 2.64999 9.71667 2.83733 10.5 3.21199C11.2833 3.58666 11.95 4.09933 12.5 4.74999C13.05 4.09999 13.7167 3.58766 14.5 3.21299C15.2833 2.83833 16.1167 2.65066 17 2.64999C18.5667 2.64999 19.875 3.17499 20.925 4.22499C21.975 5.27499 22.5 6.58333 22.5 8.14999C22.5 10.0667 21.7917 11.825 20.375 13.425C18.9583 15.025 17.35 16.6333 15.55 18.25L13.85 19.8C13.6667 19.9833 13.4543 20.1167 13.213 20.2C12.9717 20.2833 12.734 20.325 12.5 20.325Z"
            fill="black"
          />
        </svg>
        <p>Favorite</p>
      </button>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
      <button>
        <img on:click={() => goto("/me")} alt="profile" src="" />
      </button>
    {/if}
    <button
      on:click={toggleTheme}
      class="absolute top-4 right-4 p-2 rounded bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-none"
      aria-label="Toggle Theme"
    >
      {#if isDarkMode}
        🌞 Light Mode
      {:else}
        🌜 Dark Mode
      {/if}
    </button>
  </div>
</header>

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

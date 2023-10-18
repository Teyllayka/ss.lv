<template>

  <header>
    <section class="navigation">
      <div class="logo"><router-link to="/home" >Adee</router-link></div>
      <div class="routes">
        <router-link to="/adverts" class="link">Adverts</router-link>
        <router-link to="/create" class="link">Create Advert</router-link>
        <router-link to="/Bookmarks" class="link">Bookmarks</router-link>
        <!-- <router-link to="/advert">My Adverts</router-link> -->
        <router-link to="/me" v-if="logedIn">
          <div class="me">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M10.0002 10C12.3013 10 14.1668 8.13452 14.1668 5.83334C14.1668 3.53215 12.3013 1.66667 10.0002 1.66667C7.69898 1.66667 5.8335 3.53215 5.8335 5.83334C5.8335 8.13452 7.69898 10 10.0002 10Z" stroke="rgb(var(--v-theme-background))" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M17.1585 18.3333C17.1585 15.1083 13.9501 12.5 10.0001 12.5C6.05013 12.5 2.8418 15.1083 2.8418 18.3333" stroke="rgb(var(--v-theme-background))" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
        </router-link>
        <router-link to="/register" v-else><button class="register">Register</button></router-link>
        <v-switch
          inset
          color="text"
          v-model="darkMode"
          @change="toggleTheme()"
        ></v-switch>

      </div>
      <div class="burger-menu" @click="openMenu">burger menu</div>

    </section>
  </header>
</template>

<style scoped>

@media only screen and (max-width: 1000px) {

  .burger-menu {
    display: block !important;
  }

  .routes {
    display: none !important;
  }


}

.burger-menu {
  display: none;
}


.v-input--horizontal {
  grid-template-areas: none !important;
  grid-template-columns: none !important;
  grid-template-rows: none !important;
}

.me {
  height: 48px;
  width: 48px;
  background-color: rgb(var(--v-theme-text));
  border-radius: 50%;
display: flex;
justify-content: center;
align-items: center;
  transition: background-color 0.5s ease;
}

.me:hover {
  background-color: rgb(var(--v-theme-text_highlight));
}

.register {
  border-radius: 50px;
  padding:10px 20px;
  color: rgb(var(--v-theme-background));
}

.logo {
  color: rgb(var(--v-theme-inputText));
  font-size: 32px;
  font-weight: 700;
  transition: color 0.5s ease;
}

.logo:hover {
  color: rgb(var(--v-theme-text_highlight));
}

header {
 height: 72px;
}

section {
  padding: 0px 150px;
  height: 100%;
}

.navigation {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.routes {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: row;
  gap:40px;
  height: 100%;
}

.link {
  font-size:20px;

}

</style>

<script setup>
import { ref, watchEffect } from "vue";
import { useTheme } from "vuetify";

const theme = useTheme();
const darkMode = ref(localStorage.getItem("darkMode") === "true" || false);
const menu = ref(false)
const logedIn = localStorage.getItem("logedIn");

const toggleTheme = () => {
  theme.global.name.value = darkMode.value ? "customDarkTheme" : "customLightTheme";
  localStorage.setItem("darkMode", darkMode.value);
};


const openMenu = () => {
  menu.value = !menu.value;
  console.log(menu.value)
}

watchEffect(() => {
  localStorage.setItem("darkMode", darkMode.value);
});


</script>
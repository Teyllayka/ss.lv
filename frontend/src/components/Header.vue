<!-- https://www.figma.com/community/file/1252561852327562039 -->
<!-- https://www.figma.com/community/file/1083078921913061682 -->

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

<template>

  <header>
    <section class="navigation">
      <div class="left"><router-link to="/home" >Adee</router-link></div>
      <div class="center">
        <router-link to="/home" class="link">Home</router-link>
        <router-link to="/create" class="link">Create</router-link>
        <router-link to="/adverts" class="link">About</router-link>
        <router-link to="/contact" class="link">Contact</router-link>
      </div>
      <div class="right">
        <div class="svgs">
          <router-link :to="logedIn ? '/me' : '/register'">
            <svg class="svg-me" xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 28 28" fill="none">
              <path d="M23.3333 14V8.16669H25.6666V15.1667H23.3333M23.3333 19.8334H25.6666V17.5H23.3333M11.6666 15.1667C14.7816 15.1667 21 16.73 21 19.8334V23.3334H2.33331V19.8334C2.33331 16.73 8.55165 15.1667 11.6666 15.1667ZM11.6666 4.66669C12.9043 4.66669 14.0913 5.15835 14.9665 6.03352C15.8416 6.90869 16.3333 8.09568 16.3333 9.33335C16.3333 10.571 15.8416 11.758 14.9665 12.6332C14.0913 13.5084 12.9043 14 11.6666 14C10.429 14 9.24198 13.5084 8.36682 12.6332C7.49164 11.758 6.99998 10.571 6.99998 9.33335C6.99998 8.09568 7.49164 6.90869 8.36682 6.03352C9.24198 5.15835 10.429 4.66669 11.6666 4.66669ZM11.6666 17.3834C8.20165 17.3834 4.54998 19.0867 4.54998 19.8334V21.1167H18.7833V19.8334C18.7833 19.0867 15.1316 17.3834 11.6666 17.3834ZM11.6666 6.88335C11.0169 6.88335 10.3937 7.14148 9.93424 7.60094C9.47477 8.06041 9.21665 8.68357 9.21665 9.33335C9.21665 9.98313 9.47477 10.6063 9.93424 11.0658C10.3937 11.5252 11.0169 11.7834 11.6666 11.7834C12.3164 11.7834 12.9396 11.5252 13.3991 11.0658C13.8585 10.6063 14.1166 9.98313 14.1166 9.33335C14.1166 8.68357 13.8585 8.06041 13.3991 7.60094C12.9396 7.14148 12.3164 6.88335 11.6666 6.88335Z" fill="inherit"/>
            </svg>
          </router-link>
          <router-link to="/adverts" >
            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 28 28" fill="none">
              <path d="M24.5 24.5L19.2664 19.257M22.1667 12.25C22.1667 14.88 21.1219 17.4024 19.2622 19.2621C17.4024 21.1219 14.8801 22.1666 12.25 22.1666C9.61998 22.1666 7.09763 21.1219 5.2379 19.2621C3.37816 17.4024 2.33337 14.88 2.33337 12.25C2.33337 9.61992 3.37816 7.09757 5.2379 5.23784C7.09763 3.3781 9.61998 2.33331 12.25 2.33331C14.8801 2.33331 17.4024 3.3781 19.2622 5.23784C21.1219 7.09757 22.1667 9.61992 22.1667 12.25V12.25Z" stroke="inherit" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </router-link>
          <router-link to="/bookmarks" >
            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 28 28" fill="none">
              <path d="M8.16671 3.5C4.94554 3.5 2.33337 6.08533 2.33337 9.275C2.33337 11.8498 3.35421 17.9608 13.4027 24.1383C13.5827 24.2479 13.7893 24.3058 14 24.3058C14.2107 24.3058 14.4174 24.2479 14.5974 24.1383C24.6459 17.9608 25.6667 11.8498 25.6667 9.275C25.6667 6.08533 23.0545 3.5 19.8334 3.5C16.6122 3.5 14 7 14 7C14 7 11.3879 3.5 8.16671 3.5Z" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </router-link>
        </div>
        <v-switch
          inset
          color="text"
          v-model="darkMode"
          @change="toggleTheme()"
        ></v-switch>
      </div>
      <div class="burger-button" @click="openMenu"><svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 50 50" width="50px" height="50px"><path d="M 5 8 A 2.0002 2.0002 0 1 0 5 12 L 45 12 A 2.0002 2.0002 0 1 0 45 8 L 5 8 z M 5 23 A 2.0002 2.0002 0 1 0 5 27 L 45 27 A 2.0002 2.0002 0 1 0 45 23 L 5 23 z M 5 38 A 2.0002 2.0002 0 1 0 5 42 L 45 42 A 2.0002 2.0002 0 1 0 45 38 L 5 38 z"/></svg></div>
    
      <div v-if="menu" class="burger-menu">
        <p>aaa</p>
        <p>bbb</p>
      </div>
      
    </section>
  </header>
</template>

<style scoped>

@media only screen and (max-width: 1400px) {

  .burger-menu {
    display: block !important;
  }

  .burger-button {
    display: block !important;
  }

  .left {
    display: none !important;
  }

  .center {
    display: none !important;
  }

  .right {
    display: none !important;
  }


}

@media only screen and (max-width: 800px) {

  section {
    padding: 0px 40px !important;
  }




}

.svgs {
  display: flex;
  justify-content: center;
  align-items: center;
  gap:40px;
  margin-top:6px;
}

svg {
  stroke: rgb(var(--v-theme-text));
}

.svg-me {
  stroke: none;
  fill: rgb(var(--v-theme-text));
}

.burger-button {
  display: none;
  stroke: rgb(var(--v-theme-text));
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
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}

.center {
  display:flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap:50px;
}

.right {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap:40px;
}

.left {
  color: rgb(var(--v-theme-inputText));
  font-size: 32px;
  font-weight: 700;
  transition: color 0.5s ease;
  width:256px;
}



.link {
  font-size:20px;

}

</style>

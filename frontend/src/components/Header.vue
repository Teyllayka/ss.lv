<template>
  <header class="bg-background">
    <div><router-link to="/">Go to Home</router-link></div>
    <div><router-link to="/me">Me</router-link></div>
    <div><router-link to="/create">Create Advert</router-link></div>

    <v-switch
      inset
      color="info"
      v-model="darkMode"
      @change="toggleTheme()"
      :label="`It's ${darkMode ? 'customDarkTheme' : 'customLightTheme'}!`"
    ></v-switch>
  </header>
</template>

<script setup>
import { ref, watchEffect } from "vue";
import { useTheme } from "vuetify";

const theme = useTheme();
const darkMode = ref(localStorage.getItem("darkMode") === "true" || false);

const toggleTheme = () => {
  theme.global.name.value = darkMode.value ? "customDarkTheme" : "customLightTheme";
  localStorage.setItem("darkMode", darkMode.value);
  console.log(`Current theme is dark? ${theme.global.current.value.dark}`);
};

watchEffect(() => {
  localStorage.setItem("darkMode", darkMode.value);
});
</script>
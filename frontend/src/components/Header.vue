<template>
  <header class="bg-background">
    <div class="text-text"><router-link to="/">Go to Home</router-link></div>
    <div class="text-text"><router-link to="/me">Me</router-link></div>
    <div class="text-text"><router-link to="/create">Create Advert</router-link></div>

    <v-switch
      inset
      color="text"
      v-model="darkMode"
      @change="toggleTheme()"
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
};

watchEffect(() => {
  localStorage.setItem("darkMode", darkMode.value);
});


</script>
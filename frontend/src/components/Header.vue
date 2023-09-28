<template>
   <header>
      <div><router-link to="/">Go to Home</router-link></div>
      <div><router-link to="/me">Me</router-link></div>

      <v-switch
            inset
            color="info"
            v-model="darkMode"
            @change="toggleTheme()"
            :label="`It's ${darkMode ? 'customDarkTheme' : 'customLightTheme '}!`"
          ></v-switch>
   </header>
  
</template>

<script setup>
import { ref } from "vue";
import { useTheme } from "vuetify";

const theme = useTheme();
const darkMode = ref(false);

const toggleTheme = () => {
  theme.global.name.value = darkMode.value ? "customDarkTheme" : "customLightTheme";
  // Optional: Get value of current theme
  console.log(`Current theme is dark? ${theme.global.current.value.dark}`);
};
</script>
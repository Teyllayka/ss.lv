import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import "@/styles/global.css";
import { createVuetify } from "vuetify";

// #06262c
// #5fcea9
// #46b4a0
// #00565c

const customDarkTheme = {
  dark: true,
  colors: {
    background: "#06262c",
    text: "#5fcea9",
    text_highlight: "#46b4a0",
  },
};



// #fcfdfd
// #317a83
// #6fd6e2
// #24b9be

const customLightTheme = {
  dark: false,
  colors: {
    background: "#fcfdfd",
    text: "#6fd6e2",
    text_highlight: "#24b9be",
  },
};

let defaultTheme;

if (localStorage.getItem("darkMode") == "false") {
  defaultTheme = "customLightTheme";
} else {
  defaultTheme = "customDarkTheme";
}

console.log(localStorage.getItem("darkMode"));
console.log(defaultTheme);

export default createVuetify({
  theme: {
    defaultTheme,
    themes: {
      customDarkTheme,
      customLightTheme,
    },
  },
});

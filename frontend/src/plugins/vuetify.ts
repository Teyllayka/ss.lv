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
    surface: "#15202b",
    primary: "#3f51b5",
    secondary: "#03dac6",
    text: "#5fcea9",
    error: "#f44336",
    info: "#2196F3",
    success: "#4caf50",
    warning: "#fb8c00",
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
    surface: "#15202b",
    primary: "#3f51b5",
    secondary: "#03dac6",
    text: "#5fcea9",
    error: "#f44336",
    info: "#2196F3",
    success: "#4caf50",
    warning: "#fb8c00",
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

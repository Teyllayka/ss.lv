import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import { createVuetify } from "vuetify";

const customDarkTheme = {
  dark: true,
  colors: {
    background: "#ffffff",
    surface: "#15202b",
    primary: "#3f51b5",
    secondary: "#03dac6",
    error: "#f44336",
    info: "#2196F3",
    success: "#4caf50",
    warning: "#fb8c00",
  },
};

const customLightTheme = {
  dark: false,
  colors: {
    background: "#FF0000",
    surface: "#15202b",
    primary: "#3f51b5",
    secondary: "#03dac6",
    error: "#f44336",
    info: "#2196F3",
    success: "#4caf50",
    warning: "#fb8c00",
  },
};

let defaultTheme;

if (localStorage.getItem("darkMode") == 'false') {
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

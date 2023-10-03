import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import "@/styles/global.css";
import { createVuetify } from "vuetify";

// #06262c
// #5fcea9
// #46b4a0
// #46b4a0

// #1A1A1A

// #42D392

const customDarkTheme = {
  dark: true,
  colors: {
    background: "#1A1A1A",
    text: "#42D392",
    inputText: "#ffffff",
    bg1: "#092e1d",
    bg2: "#165337",
    bg3: "#247b54",
    bg4: "#33a672",
    text_highlight: "#0575e6",
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
    text: "#0575e6",
    inputText: "#1A1A1A",
    bg1: "#021c7a",
    bg2: "#083195",
    bg3: "#0b47af",
    bg4: "#0a5ecb",
    //text_highlight: "#05b2e6",
    text_highlight: "#42D392",
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

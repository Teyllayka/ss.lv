/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from "./App.vue";
import { DefaultApolloClient } from "@vue/apollo-composable";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";

// Composables
import { createApp, provide, h } from "vue";

// Plugins
import { registerPlugins } from "@/plugins";

const cache = new InMemoryCache();

const apolloClient = new ApolloClient({
  cache,
  //uri: "http://localhost:90/",
  uri: "https://api-12dpdsprogis.kvalifikacija.rvt.lv/",
});

const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient);
  },

  render: () => h(App),
});

registerPlugins(app);

app.mount("#app");

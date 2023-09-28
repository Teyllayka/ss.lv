import vuetify from "./plugins/vuetify";
import { loadFonts } from "./plugins/webfontloader";
import { createApp, provide, h } from "vue";
import { createPinia } from "pinia";

loadFonts();

import App from "./App.vue";
import router from "./router";

const cache = new InMemoryCache();

const apolloClient = new ApolloClient({
  cache,
  uri: "http://localhost:8000",
});

const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient);
  },

  render: () => h(App),
});

app.use(createPinia()).use(router).use(vuetify);

app.mount("#app");

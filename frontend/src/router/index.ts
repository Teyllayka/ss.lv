// Composables
import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";
import Advert from "@/views/Advert.vue";
import User from "@/views/User.vue";
import Me from "@/views/Me.vue";
import CreateAdvert from "@/views/CreateAdvert.vue";
import Register from "@/views/Register.vue";
import Login from "@/views/Login.vue";

import { provideApolloClient, useMutation } from "@vue/apollo-composable";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { ME, REFRESH } from "@/graphql/user";

async function isLoggedIn() {
  const cache = new InMemoryCache();
  const apolloClient = new ApolloClient({
    cache,
    uri: "http://localhost:8000",
  });
  provideApolloClient(apolloClient);

  const refreshToken = localStorage.getItem("refresh_token");
  const accessToken = localStorage.getItem("access_token");

  if (accessToken) {
    const { mutate: me } = useMutation(ME);
    try {
      await me({ accessToken });
      console.log("1 true");
      return true;
    } catch (error) {
      localStorage.removeItem("access_token");
      console.log("1 false");
    }
  }

  if (refreshToken) {
    const { mutate: refresh } = useMutation(REFRESH);
    try {
      const result = await refresh({ refreshToken });
      localStorage.setItem("access_token", result?.data.refresh.refreshToken);
      localStorage.setItem("refresh_token", result?.data.refresh.refreshToken);
      console.log("2 true");
      return true;
    } catch (error) {
      console.log("2 false");
      localStorage.removeItem("access_token");
      localStorage.removeItem("refresh_token");
      return false;
    }
  }
  localStorage.removeItem("access_token");
  localStorage.removeItem("refresh_token");

  console.log("bruh");

  return false;
}

const routes = [
  {
    path: "/",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "/home",
        name: "home",
        component: Home,
      },
      {
        path: "/register",
        name: "register",
        component: Register,
        beforeEnter: async (to: any, from: any, next: any) => {
          if (
            (await !isLoggedIn()) || !localStorage.getItem("access_token")
              ? true
              : false
          ) {
            next();
          } else {
            next("/home");
          }
        },
      },
      {
        path: "/login",
        name: "login",
        component: Login,
        beforeEnter: async (to: any, from: any, next: any) => {
          if (
            (await !isLoggedIn()) || !localStorage.getItem("access_token")
              ? true
              : false
          ) {
            next();
          } else {
            next("/home");
          }
        },
      },
      {
        path: "/user/:id",
        name: "user",
        component: User,
      },
      {
        path: "/user/:userId/advert/:id",
        name: "advert",
        component: Advert,
      },
      {
        path: "/me",
        name: "me",
        component: Me,
        beforeEnter: async (to: any, from: any, next: any) => {
          if (await isLoggedIn()) {
            next();
          } else {
            next("/login");
          }
        },
      },
      {
        path: "/create",
        name: "create",
        component: CreateAdvert,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

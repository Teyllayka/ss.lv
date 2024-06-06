// Composables
import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/Home.vue";
import Advert from "@/views/Advert.vue";
import User from "@/views/User.vue";
import Me from "@/views/Me.vue";
import CreateAdvert from "@/views/CreateAdvert.vue";
import Register from "@/views/Register.vue";
import Login from "@/views/Login.vue";
import Bookmarks from "@/views/Bookmarks.vue";
import Adverts from "@/views/Adverts.vue";
import NotFound from "@/views/NotFound.vue";
import TAC from "@/views/TAC.vue";
import Stats from "@/views/Stats.vue";

import { provideApolloClient, useMutation } from "@vue/apollo-composable";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { ME, REFRESH } from "@/graphql/user";
import { useQuery } from "@vue/apollo-composable";
import Edit from "@/views/Edit.vue";
import EditAdvert from "@/views/EditAdvert.vue";

async function isLoggedIn() {
  const cache = new InMemoryCache();
  const apolloClient = new ApolloClient({
    cache,
    //uri: "http://localhost:90/",
    uri: "https://api-12dpdsprogis.kvalifikacija.rvt.lv/",
  });
  provideApolloClient(apolloClient);

  const refreshToken = localStorage.getItem("refresh_token");
  const accessToken = localStorage.getItem("access_token");

  if (accessToken) {
    console.log(1);
    //const { result, loading, error } = useQuery(ME, { accessToken });

    const { onError } = useQuery(ME, { accessToken });

    onError(async (_) => {
      if (refreshToken) {
        const { mutate: refresh } = useMutation(REFRESH);
        try {
          const result = await refresh({ refreshToken });
          localStorage.setItem(
            "access_token",
            result?.data.refresh.accessToken
          );
          localStorage.setItem(
            "refresh_token",
            result?.data.refresh.refreshToken
          );
          localStorage.setItem("logedIn", "true");
          return true;
        } catch (error) {
          localStorage.removeItem("access_token");
          localStorage.removeItem("refresh_token");
          localStorage.removeItem("logedIn");
          console.log(error);
          return false;
        }
      }
      localStorage.removeItem("access_token");
      localStorage.removeItem("refresh_token");
      localStorage.setItem("logedIn", "false");

      return false;
    });
    return true;
    // if (error) {
    //   console.error(error);
    //   localStorage.removeItem("access_token");
    // } else {
    //   return true;
    // }
  }
}

// async function isLoggedIn() {
//   const cache = new InMemoryCache();
//   const apolloClient = new ApolloClient({
//     cache,
//     uri: "http://localhost:8000",
//   });
//   provideApolloClient(apolloClient);

//   const refreshToken = localStorage.getItem("refresh_token");
//   const accessToken = localStorage.getItem("access_token");

//   if (accessToken) {
//     const { result, loading, error } = useQuery(ME, { accessToken });

//     if (error) {
//       console.error(error);
//       localStorage.removeItem("access_token");
//     } else {
//       return true;
//     }
//   }

//   if (refreshToken) {
//     const { mutate: refresh } = useMutation(REFRESH);
//     try {
//       const result = await refresh({ refreshToken });
//       localStorage.setItem("access_token", result?.data.refresh.accessToken);
//       localStorage.setItem("refresh_token", result?.data.refresh.refreshToken);
//       localStorage.setItem("logedIn", "true");
//       return true;
//     } catch (error) {
//       localStorage.removeItem("access_token");
//       localStorage.removeItem("refresh_token");
//       localStorage.removeItem("logedIn");
//       console.log(error);
//       return false;
//     }
//   }
//   localStorage.removeItem("access_token");
//   localStorage.removeItem("refresh_token");
//   localStorage.setItem("logedIn", "false");

//   return false;
// }

function logout() {
  localStorage.removeItem("access_token");
  localStorage.removeItem("refresh_token");
  localStorage.removeItem("logedIn");
}

const routes = [
  {
    path: "/",
    redirect: "/home",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "/home",
        name: "home",
        component: Home,
        beforeEnter: async (to: any, from: any, next: any) => {
          await isLoggedIn();
          next();
        },
      },
      {
        path: "/bookmarks",
        name: "bookmarks",
        component: Bookmarks,
        beforeEnter: async (to: any, from: any, next: any) => {
          if (await isLoggedIn()) {
            next();
          } else {
            next("/login");
          }
        },
      },
      {
        path: "/adverts",
        name: "adverts",
        component: Adverts,
        beforeEnter: async (to: any, from: any, next: any) => {
          await isLoggedIn();
          next();
        },
      },
      {
        path: "/register",
        name: "register",
        component: Register,
        meta: { hideNavigation: true },
        beforeEnter: async (to: any, from: any, next: any) => {
          if (localStorage.getItem("logedIn") == "true" ? false : true) {
            next();
          } else {
            next("/me");
          }
        },
      },
      {
        path: "/login",
        name: "login",
        component: Login,
        meta: { hideNavigation: true },
        beforeEnter: async (to: any, from: any, next: any) => {
          if (localStorage.getItem("logedIn") == "true" ? false : true) {
            next();
          } else {
            next("/me");
          }
        },
      },
      {
        path: "/user/:id",
        name: "user",
        component: User,
      },
      {
        path: "/advert/:id",
        name: "advert",
        component: Advert,
        beforeEnter: async (to: any, from: any, next: any) => {
          await isLoggedIn();
          next();
        },
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
        path: "/edit",
        name: "edit",
        component: Edit,
        beforeEnter: async (to: any, from: any, next: any) => {
          if (await isLoggedIn()) {
            next();
          } else {
            next("/login");
          }
        },
      },
      {
        path: "/edit_advert/:id",
        name: "edit advert",
        component: EditAdvert,
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
        beforeEnter: async (to: any, from: any, next: any) => {
          if (await isLoggedIn()) {
            next();
          } else {
            next("/login");
          }
        },
      },

      {
        path: "/logout",
        name: "logout",
        component: Me,
        beforeEnter: async (to: any, from: any, next: any) => {
          logout();
          next("/login");
        },
      },
      {
        path: "/tac",
        name: "tac",
        component: TAC,
      },
      {
        path: "/stats",
        name: "stats",
        component: Stats,
      },
      {
        path: "/404",
        component: NotFound,
      },
      {
        path: "/contact",
        component: () => import("@/views/Contact.vue"),
      },
      {
        path: "/:catchAll(.*)",
        redirect: "/404",
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

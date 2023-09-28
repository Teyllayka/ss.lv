// Composables
import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/Home.vue";
import AdvertView from "@/views/AdvertView.vue";
import UserView from "@/views/UserView.vue";
import MeView from "@/views/Me.vue";

const routes = [
  {
    path: "/",
    component: () => import("@/layouts/default/Default.vue"),
    children: [
      {
        path: "",
        name: "home",
        component: HomeView,
      },
      {
        path: "/user/:id",
        name: "user",
        component: UserView,
      },
      {
        path: "/user/:userId/advert/:id",
        name: "advert",
        component: AdvertView,
      },
      {
        path: "/me",
        name: "me",
        component: MeView,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;

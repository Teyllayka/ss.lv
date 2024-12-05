import { writable } from "svelte/store";
import Cookies from "js-cookie";

const defaultUser: UserStore = {
  emailVerified: false,
  isCompany: false,
  isLoggedIn: false,
  role: "USER",
  id: 0,
};

function createUserStore() {
  let initialState = { ...defaultUser };

  if (typeof window !== "undefined") {
    const storedUser = Cookies.get("user");
    if (storedUser) {
      try {
        const parsedUser = JSON.parse(storedUser);
        initialState = { ...defaultUser, ...parsedUser };
      } catch (error) {
        console.error("Failed to parse user cookie:", error);
      }
    }
  }

  const { subscribe, set, update } = writable(initialState);

  return {
    subscribe,
    set: (value: UserStore) => {
      const newValue = { ...defaultUser, ...value };
      if (typeof window !== "undefined") {
        Cookies.set("user", JSON.stringify(newValue), {
          secure: true,
          sameSite: "strict",
        });
      }
      set(newValue);
    },
    update: (fn: any) => {
      update((currentValue) => {
        const newValue = { ...currentValue, ...fn(currentValue) };
        if (typeof window !== "undefined") {
          Cookies.set("user", JSON.stringify(newValue), {
            secure: true,
            sameSite: "strict",
          });
        }
        return newValue;
      });
    },
    logout: () => {
      if (typeof window !== "undefined") {
        Cookies.remove("user");
      }
      set({ ...defaultUser });
    },
  };
}

export const user = createUserStore();

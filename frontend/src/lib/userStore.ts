import { writable } from "svelte/store";

function createUserStore() {
  let initialState = {
    emailVerified: false,
    isCompany: false,
    isLogedIn: false,
    role: "USER",
    id: 0,
  };

  if (typeof window !== "undefined") {
    const storedUser = localStorage.getItem("user");
    if (storedUser) {
      initialState = JSON.parse(storedUser);
    }
  }

  const { subscribe, set, update } = writable(initialState);

  return {
    subscribe,
    set: (value: any) => {
      if (typeof window !== "undefined") {
        localStorage.setItem("user", JSON.stringify(value));
      }
      set(value);
    },
    update: (fn: any) => {
      update((currentValue) => {
        const newValue = fn(currentValue);
        if (typeof window !== "undefined") {
          localStorage.setItem("user", JSON.stringify(newValue));
        }
        return newValue;
      });
    },
  };
}

export const user = createUserStore();

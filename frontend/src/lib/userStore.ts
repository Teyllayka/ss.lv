import { writable } from "svelte/store";

const defaultUser: UserStore = {
  emailVerified: false,
  isCompany: false,
  isLoggedIn: false,
  role: "USER",
  id: 0,
  banned: false,
};

function createUserStore() {
  let initialState = { ...defaultUser };

  const { subscribe, set, update } = writable(initialState);

  return {
    subscribe,
    set: (value: UserStore) => {
      const newValue = { ...defaultUser, ...value };
      set(newValue);
    },
    update: (fn: any) => {
      update((currentValue) => {
        const newValue = { ...currentValue, ...fn(currentValue) };

        return newValue;
      });
    },
    logout: () => {
      set({ ...defaultUser });
    },
  };
}

export const user = createUserStore();

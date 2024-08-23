import { writable  } from "svelte/store";

export const user = writable<{emailVerified: boolean}>({emailVerified: false});
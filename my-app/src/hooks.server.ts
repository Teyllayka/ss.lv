import { setSession } from "$houdini";

/* @type { import('@sveltejs/kit').Handle } */
export const handle = async ({ event, resolve }: any) => {
  const user = event.cookies.get("accessToken");

  setSession(event, { user });

  return await resolve(event);
};

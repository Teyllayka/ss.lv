import { graphql, setSession } from "$houdini";
import { redirect, type RequestEvent } from "@sveltejs/kit";

/* @type { import('@sveltejs/kit').Handle } */
export const handle = async ({
  event,
  resolve,
}: {
  event: RequestEvent;
  resolve: any;
}) => {
  let accessToken = event.cookies.get("accessToken");
  const expiresAt = event.cookies.get("expiresAt");
  const refreshToken = event.cookies.get("refreshToken");
  console.log("Handling request", {
    accessToken,
    expiresAt,
    refreshToken,
  }, event.url.pathname);

  if ((expiresAt && parseInt(expiresAt) < Date.now()) || !accessToken) {
    if (!refreshToken) {
      event.cookies.delete("accessToken", { path: "/" });
      event.cookies.delete("refreshToken", { path: "/" });
      event.cookies.delete("expiresAt", { path: "/" });
      event.cookies.delete("userId", { path: "/" });
      redirect(302, "/login");
    }
    
    const refresh = graphql(`
      mutation refresh($refreshToken: String!) {
        refresh(refreshToken: $refreshToken) {
          refreshToken
          accessToken
        }
      }
    `);

    let res = await refresh.mutate({ refreshToken: refreshToken }, { event });

    if (!res.errors && res.data) {
      accessToken = res.data.refresh.accessToken;
      event.cookies.set("accessToken", res.data.refresh.accessToken, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        maxAge: 60 * 100,
      });
      event.cookies.set("refreshToken", res.data.refresh.refreshToken, {
        path: "/",
        httpOnly: true,
        sameSite: "strict",
        maxAge: 60 * 180,
      });
      event.cookies.set(
        "expiresAt",
        (Date.now() + 100 * 60 * 1000).toString(),
        {
          path: "/",
        },
      );
    } else {
      event.cookies.delete("accessToken", { path: "/" });
      event.cookies.delete("refreshToken", { path: "/" });
      event.cookies.delete("expiresAt", { path: "/" });
      redirect(302, "/login");
    }
  }

  setSession(event, { accessToken });

  return await resolve(event);

};

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
  const user = event.cookies.get("accessToken");
  const expiresAt = event.cookies.get("expiresAt");
  const refreshToken = event.cookies.get("refreshToken");

  if (!refreshToken) {
    redirect(302, "/login");
  }

  if (expiresAt && parseInt(expiresAt) < Date.now()) {
    const refresh = graphql(`
      mutation refresh($refreshToken: String!) {
        refresh(refreshToken: $refreshToken) {
          refreshToken
          accessToken
        }
      }
    `);

    let res = await refresh.mutate({ refreshToken: refreshToken }, { event });

    console.log(res);

    if (!res.errors && res.data) {
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
        }
      );
    } else {
      event.cookies.delete("accessToken", { path: "/" });
      event.cookies.delete("refreshToken", { path: "/" });
      event.cookies.delete("expiresAt", { path: "/" });
      redirect(302, "/login");
    }
  }

  setSession(event, { user });

  return await resolve(event);
};

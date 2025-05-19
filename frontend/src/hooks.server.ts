import { graphql, setSession } from "$houdini";
import { redirect, type Handle, type RequestEvent } from "@sveltejs/kit";

export const handle: Handle = async ({
  event,
  resolve,
}) => {
  let accessToken = event.cookies.get("accessToken");
  const expiresAt = event.cookies.get("expiresAt");
  const refreshToken = event.cookies.get("refreshToken");
  console.log("Handling request", {
    accessToken,
    expiresAt,
    refreshToken,
  }, event.url.pathname);


  if (event.url.pathname === "/login") {
    return await resolve(event);
  }

  const needsRefresh = (expiresAt && parseInt(expiresAt) < Date.now()) || !accessToken;
  
  if (needsRefresh) {
    console.log("Access token expired or not present");
    if (!refreshToken) {
      console.log("No refresh token available, redirecting to login");
      clearAuthCookies(event);
      throw redirect(302, "/login");
    }
    
    try {
      const refresh = graphql(`
        mutation refresh($refreshToken: String!) {
          refresh(refreshToken: $refreshToken) {
            refreshToken
            accessToken
          }
        }
      `);

      let res = await refresh.mutate({ refreshToken }, { event });
      console.log("Refresh response", res);

      if (!res.errors && res.data?.refresh) {
        console.log("Refresh success", res.data.refresh);
        accessToken = res.data.refresh.accessToken;
        
        event.cookies.set("accessToken", accessToken, {
          path: "/",
          httpOnly: true,
          sameSite: "strict",
          maxAge: 60 * 100, // 100 minutes
        });
        
        event.cookies.set("refreshToken", res.data.refresh.refreshToken, {
          path: "/",
          httpOnly: true,
          sameSite: "strict",
          maxAge: 60 * 180, // 3 hours
        });
        
        const newExpiresAt = Date.now() + 100 * 60 * 1000; // 100 minutes in ms
        event.cookies.set("expiresAt", newExpiresAt.toString(), {
          path: "/",
        });
        
        setSession(event, { accessToken });
      } else {
        console.log("Refresh error", res.errors);
        clearAuthCookies(event);
        throw redirect(302, "/login");
      }
    } catch (error) {
      console.error("Token refresh failed", error);
      clearAuthCookies(event);
      throw redirect(302, "/login");
    }
  } else {
    setSession(event, { accessToken });
  }

  return await resolve(event);

};


function clearAuthCookies(event: RequestEvent) {
  event.cookies.delete("accessToken", { path: "/" });
  event.cookies.delete("refreshToken", { path: "/" });
  event.cookies.delete("expiresAt", { path: "/" });
  event.cookies.delete("userId", { path: "/" });
}
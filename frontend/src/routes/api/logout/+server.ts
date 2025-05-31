import { redirect } from "@sveltejs/kit";
import { cache } from '$houdini'
import { setSession } from '$houdini';



export function POST(event: any) {
  event.cookies.delete("accessToken", { path: "/" });
  event.cookies.delete("refreshToken", { path: "/" });
  event.cookies.delete("expiresAt", { path: "/" });
  event.cookies.delete("user", { path: "/" });
  event.cookies.delete("userId", { path: "/" });
  cache.reset()
      setSession(event, {
    accessToken: null});
  
  return redirect(303, "/");
}

import { redirect } from "@sveltejs/kit";
import { cache } from '$houdini'



export function POST({ cookies }: any) {
  cookies.delete("accessToken", { path: "/" });
  cookies.delete("refreshToken", { path: "/" });
  cookies.delete("expiresAt", { path: "/" });
  cookies.delete("user", { path: "/" });
  cache.reset()
  return redirect(303, "/");
}

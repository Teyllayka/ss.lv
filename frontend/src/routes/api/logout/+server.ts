import {  redirect } from "@sveltejs/kit";

export function POST({ cookies }: any) {
  cookies.delete("accessToken", { path: "/" });
  cookies.delete("refreshToken", { path: "/" });
  cookies.delete("expiresAt", { path: "/" });
  cookies.delete("user", {path: "/"});
  return redirect(303, "/");
}

import { json } from "@sveltejs/kit";

export function POST({ cookies }: any) {
  cookies.delete("accessToken", { path: "/" });
  cookies.delete("refreshToken", { path: "/" });
  cookies.delete("expiresAt", { path: "/" });
  return json({
    status: 200,
    body: {
      message: "Logged out",
    },
  });
}

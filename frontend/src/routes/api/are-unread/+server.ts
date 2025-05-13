import { chatUrl } from "$lib/consts";
import type { RequestHandler } from "@sveltejs/kit";



export const GET: RequestHandler = async ({ cookies, fetch }) => {
  const accessToken = cookies.get("accessToken");
  console.log("chatUrl ", `${chatUrl}/are-unread`)
  const response = await fetch(`${chatUrl}/are-unread`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
  });
  const data = await response.json();
  if (response.status === 401) {
    return new Response(null, { status: 401 });
  }
  return new Response(JSON.stringify(data), { status: response.status });
};

import { chatUrl } from "$lib/consts";
import type { RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ cookies, request, fetch }) => {
  const { chatId, messageId } = await request.json();
  const accessToken = cookies.get("accessToken");
  const response = await fetch(`${chatUrl}/message-read`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify({ chatId, messageId }),
  });
  if (!response.ok) {
    return new Response(null, { status: 401 });
  }
  return new Response(null, { status: response.status });
};

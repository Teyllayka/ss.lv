import type { RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ cookies, request }) => {
  const { chatId, messageId } = await request.json();
  const accessToken = cookies.get("accessToken");
  const response = await fetch(`http://localhost:4000/message-read`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify({ chatId, messageId }),
  });
  console.log("response", response);
  const data = await response.json();
  console.log("data", data);
  if (response.status === 401) {
    return new Response(null, { status: 401 });
  }
  return new Response(JSON.stringify(data), { status: response.status });
};

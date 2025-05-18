import { chatUrl } from "$lib/consts";
import { redirect } from "@sveltejs/kit";



export async function load({ cookies, fetch }: any) {
  const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

  if (!logedIn) {
    return redirect(302, "/login");
  }

  console.log("chatUrl", chatUrl);

  let chats = await fetch(`${chatUrl}/get-chats`, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + cookies.get("accessToken"),
    },
  });

  
  let data = await chats.json();

  return {
    chats: data,
  };
}

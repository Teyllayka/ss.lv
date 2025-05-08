import { redirect } from "@sveltejs/kit";

export async function load({ cookies }: any) {
  const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

  if (!logedIn) {
    return redirect(302, "/login");
  }

  let chats = await fetch("http://localhost:4000/get-chats", {
    method: "GET",
    headers: {
      Authorization: "Bearer " + cookies.get("accessToken"),
    },
  });

  let data = await chats.json();
  console.log("chats", data, data[0].chat, data[0].advert, data[0].participant);

  return {
    chats: data,
  };
}

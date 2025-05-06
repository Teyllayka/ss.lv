import { fail, redirect, type Actions } from "@sveltejs/kit";

export async function load({ cookies, params }: any) {
  const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

  if (!logedIn) {
    return redirect(302, "/login");
  }

  let chats = await fetch("http://localhost:4000/get-message", {
    method: "POST",
    headers: {
      Authorization: "Bearer " + cookies.get("accessToken"),
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ chatId: params.id }),
  });

  let data = await chats.json();

  return data;
}

export const actions: Actions = {
  sendMessage: async ({ request, locals, params, cookies }) => {
    const formData = await request.formData();

    const { id } = params;
    const content = formData.get("content");

    if (!id || isNaN(Number(id))) {
      return fail(400, { error: "Missing or invalid chat id." });
    }

    if (!content || typeof content !== "string" || !content.trim()) {
      return fail(400, { error: "Message content cannot be empty." });
    }

    let response = await fetch("http://localhost:4000/send-message", {
      method: "POST",
      headers: {
        Authorization: "Bearer " + cookies.get("accessToken"),
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ chatId: id, content }),
    });

    let data = await response.json();
    console.log("chat response", data, JSON.stringify({ chatId: id, content }));
  },

  sendDeal: async ({ request, cookies }) => {
    const formData = await request.formData();
    const chat_id = formData.get("chat_id");
    const price = formData.get("price");
    const state = formData.get("state");

    if (!chat_id || isNaN(Number(chat_id))) {
      return fail(400, { error: "Missing or invalid chat id." });
    }
    if (
      !["complete", "stop"].includes(state?.toString() || "") &&
      (!price || isNaN(Number(price)) || Number(price) <= 0)
    ) {
      return fail(400, { error: "Invalid offer price." });
    }

    const response = await fetch("http://localhost:4000/request-deal", {
      method: "POST",
      headers: {
        Authorization: "Bearer " + cookies.get("accessToken"),
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        chatId: chat_id,
        price: Number(price),
        state: state ? state : "start",
      }),
    });

    // const data = await response.json();
    // console.log("Deal offer response", data);
  },
};

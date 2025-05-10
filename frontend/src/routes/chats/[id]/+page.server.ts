import { fail, redirect, type Actions } from "@sveltejs/kit";


export async function load({ cookies, params }: any) {
  const start = Date.now();

  const loggedIn = cookies.get("accessToken") || cookies.get("refreshToken");
  if (!loggedIn) {
    return redirect(302, "/login");
  }

  const res = await fetch("http://localhost:4000/get-message", {
    method: "POST",
    headers: {
      Authorization: "Bearer " + cookies.get("accessToken"),
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ chatId: params.id }),
  });

  const data = await res.json();

  const duration = Date.now() - start;
  console.log(`load() for chat ${params.id} took ${duration}ms`);

  return {
    ...data,
    loadTimeMs: duration
  };
}

export const actions: Actions = {
  sendMessage: async ({ request, locals, params, cookies }) => {
    const formData = await request.formData();

    console.log("formData", formData);

    const { id } = params;
    const content = formData.get("content");

    if (!id || isNaN(Number(id))) {
      return fail(400, { error: "Missing or invalid chat id." });
    }

    if (!content || typeof content !== "string" || !content.trim()) {
      return fail(400, { error: "Message content cannot be empty." });
    }

    let urls = [];
    const additionalPhotos = formData.getAll("photos");

    for (const file of additionalPhotos) {
      if (file instanceof File) {
        const formGachi = new FormData();
        formGachi.append("file", file);
        const response = await fetch("https://gachi.gay/api/upload", {
          method: "POST",
          body: formGachi,
        });
        const dataGachi = await response.json();
        urls.push(dataGachi.link);
      }
    }

    console.log("urls", urls, additionalPhotos);

    let response = await fetch("http://localhost:4000/send-message", {
      method: "POST",
      headers: {
        Authorization: "Bearer " + cookies.get("accessToken"),
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ chatId: id, content, urls }),
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

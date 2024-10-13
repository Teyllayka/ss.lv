// src/routes/api/submit/+server.ts
import { graphql } from "$houdini";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async (event) => {
  // Receive the entire event
  const { request, cookies } = event; // Destructure request and cookies from event

  const data = await request.json();

  console.log("request", data, cookies);

  const { advertId, isFavorited } = data;

  const favorite = graphql(`
    mutation favorite($id: Int!) {
      addFavorite(advertId: $id) {
        id
      }
    }
  `);

  const removeFavorite = graphql(`
    mutation removeFavorite($id: Int!) {
      removeFavorite(advertId: $id) {
        id
      }
    }
  `);

  let res;
  if (isFavorited) {
    res = await favorite.mutate({ id: advertId }, { event }); // Pass the event correctly
  } else {
    res = await removeFavorite.mutate({ id: advertId }, { event }); // Pass the event correctly
  }

  console.log("res", res);

  return new Response(JSON.stringify({}));
};

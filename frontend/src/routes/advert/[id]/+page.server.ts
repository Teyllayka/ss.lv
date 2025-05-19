import { graphql } from "$houdini";
import { chatUrl } from "$lib/consts";
import { editAdvertSchema, validateSchema } from "$lib/schemas";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";

import type { Actions, LoadEvent } from "@sveltejs/kit";

export async function load(event: LoadEvent) {
  const advertId = parseInt(event.params.id ?? "0");

  const advertQuery = graphql(`
    query Advert($id: Int!) {
      advert(id: $id) {
        id
        title
        description
        price
        createdAt
        lat
        lon
        oldPrice
        isFavorited
        photoUrl
        additionalPhotos
        soldTo
        available

        review {
          id
          rating
          message
          createdAt

          user {
            id
            name
            surname
            email
            phone
          }
        }

        specs {
          id
          key
          value
        }

        user {
          id
          rating
          name
          surname
          createdAt
          email
          phone
        }
      }
    }
  `);

  const similarAdvertsQuery = graphql(`
    query similarAdverts($id: Int!) {
      similarAdverts(id: $id) {
        id
        title
        price
        oldPrice
        lat
        lon
        createdAt
        isFavorited
        photoUrl
        additionalPhotos
        user {
          id
          name
          surname
          rating
        }
      }
    }
  `);

  if (!advertId) {
    return redirect(302, "/");
  }

  const advert = await advertQuery.fetch({
    variables: { id: advertId },
    event,
  });

  console.log("advert", advert);

  if(!advert.data) {
    return redirect(302, "/");
  }


  const similarAdverts = await similarAdvertsQuery.fetch({
    variables: { id: advertId },
    event,
  });



  return { similarAdverts, advert };
}

export const actions: Actions = {
  delete: async (event: RequestEvent) => {
    const { id } = event.params;

    if (!id) {
      return { success: false };
    }

    const deleteAdvert = graphql(`
      mutation deleteAdvert($advertId: Int!) {
        deleteAdvert(advertId: $advertId) {
          id
        }
      }
    `);

    const res = await deleteAdvert.mutate(
      { advertId: parseInt(id) },
      { event },
    );

    if (!res.errors && res.data) {
      redirect(302, "/");
    } else {
      return { success: false };
    }
  },

  edit: async (event: RequestEvent) => {
    const { id } = event.params;

    if (!id) {
      return { success: false };
    }

    const formData = await event.request.formData();

    const data: any = {};
    formData.forEach((value, key) => {
      data[key] = value;
    });

    let photos: string[] = [];
    try {
      photos = JSON.parse(data.photos as string);
    } catch {
      return fail(400, { error: "Invalid photos payload" });
    }


    const errs = await validateSchema(editAdvertSchema, { ...data, photos });

   if (errs.length > 0) {
        const serializable = { ...data };
        delete serializable.photos;
        return fail(400, {
          data: serializable,
          errors: errs,
        });
      }
   

    

    const baseFields = [
      "title",
      "description",
      "price",
      "location",
      "mainPhoto",
      "additionalPhotos",
      "location_json",
      "category",
      "photos",
    ];

    const baseData: any = {};
    // const categoryData: any = {};

    for (const key in data) {
      if (baseFields.includes(key)) {
        baseData[key] = data[key];
      }
    }


    const edit = graphql(`
      mutation editAdvert(
        $description: String!
        $lat: Float!
        $lon: Float!
        $photos: [String!]!
        $price: Float!
        $title: String!
        $id: Int!
      ) {
        editAdvert(
          description: $description
          lat: $lat
          lon: $lon
          photos: $photos
          price: $price
          title: $title
          id: $id
        ) {
          id
          title
          description
          price
          photoUrl
          additionalPhotos
          available
          lat
          lon
        }
      }
    `);

    //let location_json = JSON.parse(data.location_json);

    let res = await edit.mutate(
      {
        description: baseData.description,
        lat: 1,
        lon: 2,
        photos,
        price: parseFloat(baseData.price),
        title: baseData.title,
        id: parseInt(id),
      },
      { event },
    );

    if (!res.errors && res.data) {
      return { success: true, advert: res.data.editAdvert };
    } else {
      console.log("errors", res.errors);
      return fail(500, { error: "Failed to edit advert" });
    }
  },
  chat: async ({request, fetch, cookies }) => {
    const formData = await request.formData();
    const advertId = formData.get("advertId");

    if (!advertId) {
      return fail(400, { error: "Invalid advert or user ID" });
    }

    const response = await fetch(`${chatUrl}/create-chat`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + cookies.get("accessToken"),
      },
      body: JSON.stringify({ postId: advertId }),
    });

    let data = await response.json();

    console.log("chat response", data);

    throw redirect(303, `/chats/${data.id}`);
  },

  review: async (event: RequestEvent) => {
    const { id } = event.params;

    if (!id) {
      return { success: false };
    }

    const formData = await event.request.formData();
    const rating = formData.get("rating");
    const text = formData.get("text");

    if (!rating || !text) {
      return fail(400, { error: "Invalid rating or text" });
    }

    const writeReview = graphql(`
      mutation writeReview($advertId: Int!, $message: String!, $rating: Int!) {
        writeReview(advertId: $advertId, message: $message, rating: $rating) {
          id
        }
      }
    `);

    const res = await writeReview.mutate(
      {
        advertId: parseInt(id),
        message: text.toString(),
        rating: parseInt(rating.toString()),
      },
      { event },
    );

    if (!res.errors && res.data) {
      console.log("review", res.data);
    } else {
      console.log("errors", res.errors);
      return fail(500, { error: "Failed to create advert" });
    }
  },
};

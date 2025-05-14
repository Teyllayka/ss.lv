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
    return { similarAdverts: null };
  }

  const similarAdverts = await similarAdvertsQuery.fetch({
    variables: { id: advertId },
    event,
  });

  const advert = await advertQuery.fetch({
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

    const errs = await validateSchema(editAdvertSchema, data);

    if (errs.length > 0) {
      const serializableData = { ...data };
      delete serializableData.newMainPhoto;
      delete serializableData.newPhotos;
      delete serializableData.existingPhotos;
      delete serializableData.existingMainPhoto;

      return fail(400, {
        data: serializableData,
        errors: errs,
      });
    }

    const newPhotos = formData.getAll("newPhotos") || [];
    const existingPhotos =
      (formData.getAll("existingPhotos") as string[]) || [];

    if (!data.existingPhotos && data?.newMainPhoto?.size === 0) {
      return fail(400, { error: "Invalid main photo" });
    }

    console.log(data.existingMainPhoto, data.newMainPhoto);
    let urls: string[] = [];
    const hasNewMain =
      data.newMainPhoto instanceof File && data.newMainPhoto.size > 0;

    if (!hasNewMain && data.existingMainPhoto) {
      urls.push(data.existingMainPhoto);
    }

    if (hasNewMain) {
      const formGachi = new FormData();
      formGachi.append("file", data.newMainPhoto);
      const response = await fetch("https://gachi.gay/api/upload", {
        method: "POST",
        body: formGachi,
      });
      const dataGachi = await response.json();
      urls.push(dataGachi.link);
    }

    urls.push(...existingPhotos);

    for (const entry of newPhotos) {
      if (entry instanceof File && entry.size > 0) {
        const formGachi = new FormData();
        formGachi.append("file", entry);
        const response = await fetch("https://gachi.gay/api/upload", {
          method: "POST",
          body: formGachi,
        });
        const dataGachi = await response.json();
        urls.push(dataGachi.link);
      }
    }

    console.log(urls);

    const baseFields = [
      "title",
      "description",
      "price",
      "location",
      "mainPhoto",
      "additionalPhotos",
      "location_json",
      "category",
    ];

    const baseData: any = {};
    // const categoryData: any = {};

    for (const key in data) {
      if (baseFields.includes(key)) {
        baseData[key] = data[key];
      }
    }

    delete baseData.newMainPhoto;
    delete baseData.existingMainPhoto;
    delete baseData.existingPhotos;
    delete baseData.newPhotos;

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
        photos: urls,
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

import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";

import type { LoadEvent } from "@sveltejs/kit";

export async function load(event: LoadEvent) {
  const advertId = parseInt(event.params.id ?? "0");

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

  return { similarAdverts };
}

export const actions = {
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

    console.log(data);

    //err check

    let urls: string[] = [];
    const additionalPhotos = formData.getAll("additionalPhotos");
    const existingPhotos = formData.getAll("existingAdditionalPhotos");

    // for (const entry of additionalPhotos) {
    //   if (entry instanceof File && entry.size > 0) {
    //     // Upload new file
    //     const formGachi = new FormData();
    //     formGachi.append("file", entry);
    //     const response = await fetch("https://gachi.gay/api/upload", {
    //       method: "POST",
    //       body: formGachi,
    //     });
    //     const dataGachi = await response.json();
    //     urls.push(dataGachi.link);
    //   }
    // }

    urls = [
      ...existingPhotos.filter((photo) => typeof photo === "string"),
      ...urls,
    ];
    urls = urls.filter((url) => url !== undefined);

    console.log(urls);

    //delete photos

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
    const categoryData: any = {};

    for (const key in data) {
      if (baseFields.includes(key)) {
        baseData[key] = data[key];
      } else {
        categoryData[key] = data[key];
      }
    }

    delete baseData.mainPhoto;
    delete baseData.additionalPhotos;

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
        }
      }
    `);

    let location_json = JSON.parse(data.location_json);

    let res = await edit.mutate(
      {
        description: baseData.description,
        lat: location_json.lat,
        lon: location_json.lon,
        photos: urls,
        price: parseFloat(baseData.price),
        title: baseData.title,
        id: parseInt(id),
      },
      { event },
    );

    if (!res.errors && res.data) {
      throw redirect(302, `/`);
    } else {
      console.log("errors", res.errors);
      return fail(500, { error: "Failed to create advert" });
    }
  },
  chat: async (event: RequestEvent) => {
    const formData = await event.request.formData();
    const advertId = formData.get("advertId");

    if (!advertId) {
      return fail(400, { error: "Invalid advert or user ID" });
    }

    const response = await fetch("http://localhost:4000/create-chat", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + event.cookies.get("accessToken"),
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

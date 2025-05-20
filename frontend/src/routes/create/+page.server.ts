import {
  fullAdvertSchemas,
  validateSchema,
} from "$lib/schemas";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { graphql } from "$houdini";

export const actions = {
  default: async (event: RequestEvent) => {
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

    const category = data.category;
    const fullSchema = fullAdvertSchemas[category] || fullAdvertSchemas.default;

    const errs = await validateSchema(fullSchema, { ...data, photos });

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
    const categoryData: any = {};

    for (const key in data) {
      if (baseFields.includes(key)) {
        baseData[key] = data[key];
      } else {
        categoryData[key] = data[key];
      }
    }




    const create = graphql(`
      mutation createAdvert(
        $category: String!
        $data: JSON!
        $description: String!
        $lat: Float!
        $lon: Float!
        $photos: [String!]!
        $price: Float!
        $title: String!
      ) {
        createAdvert(
          category: $category
          data: $data
          description: $description
          lat: $lat
          lon: $lon
          photos: $photos
          price: $price
          title: $title
        ) {
          id
        }
      }
    `);

    let location_json = JSON.parse(data.location_json);

    let res = await create.mutate(
      {
        category,
        data: categoryData,
        description: baseData.description,
        lat: parseFloat(location_json.lat),
        lon: parseFloat(location_json.lon),
        photos,
        price: parseFloat(baseData.price),
        title: baseData.title,
      },
      { event },
    );

    console.log("res", res);

    if (!res.errors && res.data) {
      const advertId = res.data.createAdvert.id;
      throw redirect(302, `/advert/${advertId}`);
    } else {
      console.log("errors", res.errors);
      return fail(500, { error: "Failed to create advert" });
    }
  },
};

export function load({ cookies }: any) {
  const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

  if (!logedIn) {
    return redirect(302, "/login");
  }
}


import {
	advertCarSchema,
	advertElectronicsSchema,
	advertSchema,
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

		const category = formData.get("category") as string;
		const schema = getCategorySchema(category);
		if (!schema) {
			console.log("grr");
			return;
		}
		const fullSchema = advertSchema.concat(schema);

		const errs = await validateSchema(fullSchema, data);
		console.log(formData, errs);

		if (errs.length > 0) {
			const serializableData = { ...data };
			delete serializableData.mainPhoto;
			delete serializableData.additionalPhotos;

			return fail(400, {
				data: serializableData,
				errors: errs,
			});
		}

		let urls = [];
		const additionalPhotos = formData.getAll('additionalPhotos');

		if(data.mainPhoto instanceof File) {
			const formGachi = new FormData();
			formGachi.append("file", data.mainPhoto);
			const response = await fetch("https://gachi.gay/api/upload", {
				method: "POST",
				body: formGachi,
			});
			const dataGachi = await response.json();
			urls.push(dataGachi.link);
		}
		for (const file of additionalPhotos) {
			if (file instanceof File) {
				const formGachi = new FormData();
				formGachi.append('file', file);
			
				const response = await fetch('https://gachi.gay/api/upload', {
				method: 'POST',
				body: formGachi,
				});
				const dataGachi = await response.json();
				urls.push(dataGachi.link);
			}
		}
		

		const create = graphql(`
      mutation createAdvert(
        $category: String!
        $data: JSON!
        $description: String!
        $location: String!
        $photos: [String!]!
        $price: Float!
        $title: String!
      ) {
        createAdvert(
          category: $category
          data: $data
          description: $description
          location: $location
          photos: $photos
          price: $price
          title: $title
        ) {
          id
        }
      }
    `);

		let res = await create.mutate(
			{
				category,
				data,
				description: data.description,
				location: data.location,
				photos: urls,
				price: parseFloat(data.price),
				title: data.title,
			},
			{ event },
		);

		if (!res.errors && res.data) {
			const advertId = res.data.createAdvert.id;
			throw redirect(302, `/advert/${advertId}`);
		} else {
			console.log("errors", res.errors);
		}
	},
};

export function load({ cookies }: any) {
	const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

	if (!logedIn) {
		return redirect(302, "/login");
	}
}

function getCategorySchema(category: string) {
	switch (category) {
		case "vehicles":
			return advertCarSchema;
		case "electronics":
			return advertElectronicsSchema;
		default:
			return null;
	}
}

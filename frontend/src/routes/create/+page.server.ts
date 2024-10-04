import { advertCarSchema, advertElectronicsSchema, advertSchema, validateSchema } from "$lib/schemas";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";

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
			console.log("grr")
			return;
		}
		const fullSchema = advertSchema.concat(schema);


		const errs = await validateSchema(fullSchema, data);
		console.log(formData, errs);

		if (errs.length > 0) {
			const serializableData = { ...data };
			delete serializableData.mainPhoto;
		  
			return fail(400, {
			  data: serializableData,
			  errors: errs,
			});
		  }

		// if (errs.length > 0 || !email || !password) {
		//   return fail(400, {
		//     email,
		//     errors: errs,
		//   });
		// }

		// const login = graphql(`
		//   mutation login($email: String!, $password: String!) {
		//     login(email: $email, password: $password) {
		//       refreshToken
		//       accessToken
		//     }
		//   }
		// `);

		// let res = await login.mutate({ email, password }, { event });

		// if (!res.errors && res.data) {
		//   event.cookies.set("accessToken", res.data.login.accessToken, {
		//     path: "/",
		//     httpOnly: true,
		//     sameSite: "strict",
		//     maxAge: 60 * 100,
		//   });
		//   event.cookies.set("refreshToken", res.data.login.refreshToken, {
		//     path: "/",
		//     httpOnly: true,
		//     sameSite: "strict",
		//     maxAge: 60 * 180,
		//   });
		//   event.cookies.set(
		//     "expiresAt",
		//     (Date.now() + 100 * 60 * 1000).toString(),
		//     {
		//       path: "/",
		//     }
		//   );
		//   redirect(302, "/");
		// } else {
		//   return fail(400, {
		//     email,
		//     password,
		//     errors: [
		//       { field: "email", message: "Invalid email or password" },
		//       { field: "password", message: "Invalid email or password" },
		//     ],
		//   });
		// }
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
  
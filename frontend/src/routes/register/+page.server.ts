import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { registerSchema, validateSchema } from "$lib/schemas";

export const actions = {
	default: async (event: RequestEvent) => {
		const formData = await event.request.formData();

		const data: any = {};
		formData.forEach((value, key) => {
			data[key] = value;
		});

		const errs = await validateSchema(registerSchema, data);

		if (errs.length > 0) {
			return fail(400, {
				data,
				errors: errs,
			});
		}

		console.log("no errors");

		const register = graphql(`
      mutation register(
        $email: String!
        $name: String
        $password: String!
        $surname: String
        $companyName: String
      ) {
        register(
          email: $email
          name: $name
          password: $password
          surname: $surname
          companyName: $companyName
        ) {
          id
        }
      }
    `);

		let res = await register.mutate(
			{
				email: data.email,
				password: data.password,
				name: data.name,
				surname: data.surname,
				companyName: data.companyName,
			},
			{ event },
		);
		console.log(res);

		if (!res.errors && res.data) {
			redirect(302, "/login");
		} else {
			return fail(400, {
				data,
				errors: [
					{ field: "email", message: "Invalid email or password" },
					{ field: "password", message: "Invalid email or password" },
				],
			});
		}
	},
};

export function load({ cookies }: any) {
	const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

	if (logedIn) {
		return redirect(302, "/me");
	}
}

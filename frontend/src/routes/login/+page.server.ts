import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { loginSchema, validateSchema } from "$lib/schemas";

export const actions = {
	default: async (event: RequestEvent) => {
		const formData = await event.request.formData();

		const data: any = {};
		formData.forEach((value, key) => {
			data[key] = value;
		});

		const errs = await validateSchema(loginSchema, data);

		if (errs.length > 0) {
			return fail(400, {
				data,
				errors: errs,
			});
		}

		const login = graphql(`
      mutation login($email: String!, $password: String!) {
        login(email: $email, password: $password) {
          refreshToken
          accessToken
        }
      }
    `);

		let res = await login.mutate(
			{ email: data.email, password: data.password },
			{ event },
		);

		if (!res.errors && res.data) {
			event.cookies.set("accessToken", res.data.login.accessToken, {
				path: "/",
				httpOnly: true,
				sameSite: "strict",
				expires: new Date(Date.now() + 60 * 100 * 1000),
			});
			event.cookies.set("refreshToken", res.data.login.refreshToken, {
				path: "/",
				httpOnly: true,
				sameSite: "strict",
				expires: new Date(Date.now() + 60 * 180 * 1000),
			});
			event.cookies.set(
				"expiresAt",
				(Date.now() + 60 * 100 * 1000).toString(),
				{
					path: "/",
				},
			);
			redirect(302, "/");
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

import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { user } from "$lib/userStore";

export function load({ cookies }: any) {
	const logedIn = cookies.get("accessToken");

	if (!logedIn) {
		return redirect(302, "/login");
	}
}

export const actions = {
	verify: async (event: RequestEvent) => {
		console.log("verify");

		let userValue: any;

		user.subscribe((value) => {
			userValue = value;
		});

		console.log(userValue);

		if (userValue?.emailVerified) {
			return { success: false };
		}

		const resendEmail = graphql(`
      mutation resendEmail {
        resendEmail
      }
    `);

		let res = await resendEmail.mutate(null, { event });
		console.log(res);

		if (!res.errors && res.data) {
			return { sucess: true };
		} else {
			return fail(400, {
				errors: res.errors,
			});
		}
	},
};

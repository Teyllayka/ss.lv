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

	

	updateProfile: async (event: RequestEvent) => {
		const formData = await event.request.formData();

		const name = formData.get("name") as string;
		const surname = formData.get("surname") as string;
		const companyName = formData.get("companyName") as string;
		const phone = formData.get("phone") as string;
		const password = formData.get("password") as string;

		if(!password) {
			return fail(400, { errors: { form: "Password is required." } });
		}


		console.log(name, surname, companyName, phone, password);


		const profileData = graphql(`
			query profileData {
				me {
					name
					surname
					companyName
					phone
					email
					emailVerified
					telegramUsername
				}
			}
		`);

		const profileResponse = await profileData.fetch({ event });
		if (!profileResponse.data || !profileResponse.data.me) {
			return fail(400, { errors: { form: "Failed to fetch profile data." } });
		}
		const currentData = profileResponse.data.me;


		//check what fields are different

		let updateFields: any = {};

		if (name !== currentData.name) {
			updateFields.name = name;
		}

		if (surname !== currentData.surname) {
			updateFields.surname = surname;
		}

		if (companyName !== currentData.companyName) {
			updateFields.companyName = companyName;
		}

		if (phone !== currentData.phone) {
			updateFields.phone = phone;
		}

		console.log(updateFields);

		if (Object.keys(updateFields).length === 0) {
			return fail(400, { errors: { form: "No changes detected." } });
		}


		const editProfile = graphql(`
			mutation editProfile($name: String, $surname: String, $companyName: String, $phone: String, $password: String!) {
				edit(name: $name, surname: $surname, companyName: $companyName, phone: $phone, password: $password) {
					id
				}
			}
		`);
		

		// const input: any = {
		// 	name,
		// 	surname,
		// 	phone,
		// };

		// if (companyName) {
		// 	input.companyName = companyName;
		// }

		// if (password) {
		// 	input.password = password;
		// }

		// try {
		// 	const res = await updateProfileMutation.mutate(
		// 		{ input },
		// 		{ event }
		// 	);

		// 	if (res.errors || !res.data?.updateProfile.success) {
		// 		return fail(400, {
		// 			errors: {
		// 				form: res.data?.updateProfile.message || "Failed to update profile.",
		// 			},
		// 		});
		// 	}


		// 	return { success: true };
		// } catch (error) {
		// 	console.error("Error updating profile:", error);
		// 	return fail(500, { errors: { form: "Internal Server Error" } });
		// }
	},
};

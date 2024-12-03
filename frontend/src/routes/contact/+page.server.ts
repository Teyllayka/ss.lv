import { fail, type RequestEvent } from "@sveltejs/kit";
import { contactSchema, validateSchema } from "$lib/schemas";

export const actions = {
	default: async (event: RequestEvent) => {
		const formData = await event.request.formData();

		const data: any = {};
		formData.forEach((value, key) => {
			data[key] = value;
		});

		const errs = await validateSchema(contactSchema, data);

		if (errs.length > 0) {
			return fail(400, {
				data,
				errors: errs,
			});
		}
	},
};

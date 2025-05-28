import { graphql } from "$houdini";
import { fail, json, redirect, type RequestEvent } from "@sveltejs/kit";
import { forgotPasswordSchema, validateSchema } from "$lib/schemas";

export const actions = {
  default: async (event: RequestEvent) => {
    const formData = await event.request.formData();
    const data: any = {};
    formData.forEach((value, key) => {
      data[key] = value;
    });

    const errs = await validateSchema(forgotPasswordSchema, data);

    if (errs.length > 0) {
      return fail(400, {
        data,
        errors: errs,
      });
    }

    const forgotPassword = graphql(`
      mutation forgotPassword($email: String!) {
        forgotPassword(email: $email)
      }
    `);

    let res = await forgotPassword.mutate(
      { email: data.email },
      { event },
    );

    console.log("Forgot password result:", res);

    if (!res.errors && res.data) {
    return {
        success: true
    }
    } else {
      return fail(400, {
        data,
        errors: [
          { field: "email", message: "Invalid email" },
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

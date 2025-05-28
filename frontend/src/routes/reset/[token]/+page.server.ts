import { graphql } from "$houdini";
import { fail, redirect } from "@sveltejs/kit";
import type { RequestEvent } from "./$types";
import { resetPasswordSchema, validateSchema } from "$lib/schemas";

export async function load(event: RequestEvent) {
  const validateToken = graphql(`
    query validateResetToken($token: String!) {
      validateResetToken(token: $token)
    }
  `);

  let res = await validateToken.fetch({ event, variables: {
    token: event.params.token
  }});

  return res;
}

export const actions = {
  resetPassword: async (event: RequestEvent) => {
    const formData = await event.request.formData();
    const data: Record<string, string> = {};
    formData.forEach((v, k) => (data[k] = v as string));

    const errs = await validateSchema(resetPasswordSchema, data);
    if (errs.length) {
      return fail(400, { data, errors: errs });
    }

    const resetPassword = graphql(`
      mutation resetPassword($token: String!, $newPassword: String!) {
        resetPassword(token: $token, newPassword: $newPassword)
      }
    `);

    const res = await resetPassword.mutate(
      { token: event.params.token, newPassword: data.password },
      { event }
    );

    if (res.errors?.length) {
      return fail(400, { error: res.errors[0].message });
    }

    if (res.data?.resetPassword === "Password has been reset successfully") {
      throw redirect(303, '/login');
    }

    return fail(400, { error: 'Failed to reset password. Please try again.' });
  }
};

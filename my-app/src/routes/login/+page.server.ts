import { graphql } from "$houdini";
import { fail, redirect } from "@sveltejs/kit";
import { loginSchema, validateSchema } from "$lib/schemas";

export const actions = {
  default: async (event: any, cookies: any) => {
    const data = await event.request.formData();

    const password = data.get("password")?.toString();
    const email = data.get("email")?.toString();

    const errs = await validateSchema(loginSchema, { email, password });

    if (errs.length > 0) {
      return fail(400, {
        email,
        password,
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

    let res = await login.mutate({ email, password }, { event });
    console.log(res);

    if (!res.errors && res.data) {
      event.cookies.set("accessToken", res.data.login.accessToken, {
        path: "/",
      });
      event.cookies.set("refreshToken", res.data.login.refreshToken, {
        path: "/",
      });
      redirect(302, "/");
    } else {
      return fail(400, {
        email,
        password,
        errors: [
          { field: "email", message: "Invalid email or password" },
          { field: "password", message: "Invalid email or password" },
        ],
      });
    }
  },
};

export function load({ cookies }: any) {
  const logedIn = cookies.get("accessToken");

  if (logedIn) {
    return redirect(302, "/");
  }
}

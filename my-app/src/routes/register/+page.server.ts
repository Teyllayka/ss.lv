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

    console.log(errs);

    if (
      errs.length > 0
    ) {
      return fail(400, {
        data,
        errors: errs,
      });
    }

    console.log("no errors");

    const register = graphql(`
      mutation register(
        $email: String!
        $image: String!
        $name: String!
        $password: String!
        $phone: String!
        $surname: String!
      ) {
        register(
          email: $email
          image: $image
          name: $name
          password: $password
          phone: $phone
          surname: $surname
        ) {
          id
        }
      }
    `);

    let res = await register.mutate(
      { email: data.email, password: data.password, name: data.name, surname: data.surname, image: data.image, phone: data.phone },
      { event }
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

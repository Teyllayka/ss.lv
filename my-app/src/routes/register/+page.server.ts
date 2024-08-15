import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { registerSchema, validateSchema } from "$lib/schemas";

export const actions = {
  default: async (event: RequestEvent) => {
    const data = await event.request.formData();

    const password = data.get("password")?.toString();
    const email = data.get("email")?.toString();
    const name = data.get("name")?.toString();
    const surname = data.get("surname")?.toString();
    const phone = data.get("phone")?.toString();
    const image = data.get("image")?.toString();

    const errs = await validateSchema(registerSchema, {
      email,
      password,
      name,
      surname,
      phone,
      image,
    });

    console.log(errs);

    if (
      errs.length > 0 ||
      !email ||
      !password ||
      !name ||
      !surname ||
      !phone ||
      !image
    ) {
      return fail(400, {
        email,
        name,
        surname,
        phone,
        image,
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
      { email, password, name, surname, image, phone },
      { event }
    );
    console.log(res);

    if (!res.errors && res.data) {
      redirect(302, "/login");
    } else {
      return fail(400, {
        email,
        name,
        surname,
        phone,
        image,
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
    return redirect(302, "/me");
  }
}

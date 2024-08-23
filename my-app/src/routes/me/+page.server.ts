import { redirect, type RequestEvent } from "@sveltejs/kit";

export function load({ cookies }: any) {
  const logedIn = cookies.get("accessToken");

  if (!logedIn) {
    return redirect(302, "/login");
  }
}

export const actions = {
  // verify: async (event: RequestEvent) => {
  //   const register = graphql(`
  //     mutation register(
  //       $email: String!
  //       $image: String!
  //       $name: String!
  //       $password: String!
  //       $phone: String!
  //       $surname: String!
  //     ) {
  //       register(
  //         email: $email
  //         image: $image
  //         name: $name
  //         password: $password
  //         phone: $phone
  //         surname: $surname
  //       ) {
  //         id
  //       }
  //     }
  //   `);

  //   let res = await register.mutate(
  //     { email: data.email, password: data.password, name: data.name, surname: data.surname, image: data.image, phone: data.phone },
  //     { event }
  //   );
  //   console.log(res);

  //   if (!res.errors && res.data) {
  //     redirect(302, "/login");
  //   } else {
  //     return fail(400, {
  //       data,
  //       errors: [
  //         { field: "email", message: "Invalid email or password" },
  //         { field: "password", message: "Invalid email or password" },
  //       ],
  //     });
  //   }
  // },
};
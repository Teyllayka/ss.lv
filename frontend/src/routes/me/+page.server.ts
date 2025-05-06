import { graphql } from "$houdini";
import { fail, redirect, type RequestEvent } from "@sveltejs/kit";
import { user } from "$lib/userStore";
import { editProfileSchema, validateSchema } from "$lib/schemas";

export function load({ cookies }: any) {
  const logedIn = cookies.get("accessToken") || cookies.get("refreshToken");

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
        errors: { field: "password", message: "wrong password" },
      });
    }
  },

  updateProfile: async (event: RequestEvent) => {
    const formData = await event.request.formData();

    const data: any = {};
    formData.forEach((value, key) => {
      data[key] = value;
    });

    console.log(data);

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

    let updateFields: any = {};

    if (data.name && data.name !== currentData.name) {
      updateFields.name = data.name;
    }

    if (data.surname && data.surname !== currentData.surname) {
      updateFields.surname = data.surname;
    }

    if (data.companyName && data.companyName !== currentData.companyName) {
      updateFields.companyName = data.companyName;
    }

    if (data.phone && data.phone !== currentData.phone) {
      updateFields.phone = data.phone;
    }

    console.log(updateFields);

    if (Object.keys(updateFields).length === 0) {
      return fail(400, { errors: { form: "No changes detected." } });
    }

    const errors = await validateSchema(editProfileSchema, {
      ...updateFields,
      password: data.password,
    });

    if (errors.length > 0) {
      return fail(400, { errors, updateFields });
    }

    const editProfile = graphql(`
      mutation editProfile(
        $name: String
        $surname: String
        $companyName: String
        $phone: String
        $password: String!
      ) {
        edit(
          name: $name
          surname: $surname
          companyName: $companyName
          phone: $phone
          password: $password
        ) {
          id
          name
          surname
          companyName
          email
          emailVerified
          rating
          telegramUsername
          phone

          reviewedAdverts {
            title
            lat
            lon
            price
            createdAt
            review {
              message
              rating
              createdAt
            }
          }

          advertsWithReviews {
            title
            price
            lat
            lon
            available
            review {
              rating
              message
              createdAt
              user {
                name
              }
            }
          }

          adverts {
            id
            title
            price
            description
            lat
            lon
            createdAt
            photoUrl
            additionalPhotos
            oldPrice
            available

            review {
              rating
              id
              message
            }
          }
        }
      }
    `);

    const res = await editProfile.mutate(
      { ...updateFields, password: data.password },
      { event },
    );

    console.log(res);

    if (!res.data || !res.data.edit) {
      return fail(400, {
        updateFields,
        errors: [{ field: "password", message: "wrong password" }],
      });
    }

    return { success: true, data: res.data.edit };
  },
};

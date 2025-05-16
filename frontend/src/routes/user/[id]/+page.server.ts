import { graphql } from "$houdini";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async (event) => {
  const userId = event.cookies.get("userId");
  const id = parseInt(event.params.id ?? "0");

  console.log("userId", userId, id);

  if (userId === event.params.id) {
    return redirect(302, "/me");
  }


const userQuery = graphql(`
query User($id: Int!) {
  user(id: $id) {
    id
    name
    surname
    phone
    email
    rating
    avatarUrl
    adverts {
      id
      title
      price
      description
      lat
      lon
      createdAt
      available
      photoUrl
      additionalPhotos
      oldPrice
      isFavorited

      review {
        rating
        id
        message
      }
    }

    reviewedAdverts {
      title
      lat
      lon
      price
      createdAt
      photoUrl
      review {
        message
        rating
        createdAt
        user {
          name
          avatarUrl
          id
        }
      }
    }

    advertsWithReviews {
      title
      price
      lat
      lon
      available
      photoUrl
      review {
        rating
        message
        createdAt
        user {
          name
          avatarUrl
          id
        }
      }
    }
  }
}
  `);

  const userData = await userQuery.fetch({ variables: { id: id },event, policy: "NoCache" });
  console.log("u", userData);

  if (!userData.data) {
    throw redirect(302, "/");
  }


  return {
    user: userData
  }



}

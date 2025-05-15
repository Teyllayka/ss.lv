import { graphql } from "$houdini";
import { user } from "$lib/userStore";
import { redirect, type LoadEvent } from "@sveltejs/kit";

export async function load(event: LoadEvent) {
  let userValue: any;

  user.subscribe((value) => {
    userValue = value;
  });

  if (userValue?.id.toString() === event.params.id) {
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
    telegramUsername
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

  const userData = await userQuery.fetch({ event, policy: "NoCache" });


  if (!userData.data) {
    throw redirect(302, "/");
  }


  return {
    user: userData
  }



}

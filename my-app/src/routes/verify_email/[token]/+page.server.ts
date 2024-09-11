import { graphql } from "$houdini";
import type { RequestEvent } from "./$types";
import { user } from "$lib/userStore";
import { redirect } from "@sveltejs/kit";

export async function load(event: RequestEvent) {
  let userValue: any;
  
  user.subscribe((value) => {
    userValue = value;
  });
  
  
  console.log(userValue);
  
  if(userValue?.emailVerified) {
    redirect(302, "/");
  }


  const verify = graphql(`
    mutation verify_email($token: String!) {
      verifyEmail(token: $token)
    }
  `);

  let res = await verify.mutate({ token: event.params.token }, { event });

  if(res.data?.verifyEmail == "Email verified") {
    user.update((value: any) => {
      return { ...value, emailVerified: true };
    });
    console.log(userValue)
  }

  return res;
}

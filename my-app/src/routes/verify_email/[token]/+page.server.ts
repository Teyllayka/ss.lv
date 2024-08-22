import { graphql } from "$houdini";
import type { RequestEvent } from "./$types";

export async function load(event: RequestEvent) {
  const verify = graphql(`
    mutation verify_email($token: String!) {
      verifyEmail(token: $token)
    }
  `);

  let res = await verify.mutate({ token: event.params.token }, { event });

  return res;
}

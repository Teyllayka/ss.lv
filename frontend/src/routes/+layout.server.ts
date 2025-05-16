import { graphql } from "$houdini";
import type { LayoutServerLoad } from "./$types";



export const load: LayoutServerLoad = async (event) => {
    const HeaderMe = graphql(`
        query HeaderMe {
  me {
    id
    name
    surname
    companyName
    email
    emailVerified
    role
    avatarUrl
    banned
  }
}

        
        `)
  const data = await HeaderMe.fetch({ event, policy: "NoCache" });


  return {
    HeaderMe: data
  }


}
import { HoudiniClient } from "$houdini";
// import { PUBLIC_BACKEND_URI } from '$env/static/public';

export default new HoudiniClient({
  url: "http://localhost:80",

  // uncomment this to configure the network call (for things like authentication)
  // for more information, please visit here: https://www.houdinigraphql.com/guides/authentication
  // fetchParams({ session }) {
  //     return {
  //         headers: {
  //             Authentication: `Bearer ${session.token}`,
  //         }
  //     }
  // }
});

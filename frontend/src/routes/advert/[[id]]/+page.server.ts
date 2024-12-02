import { graphql } from "$houdini";
import { redirect, type RequestEvent } from "@sveltejs/kit";

export const actions = {
  delete: async (event: RequestEvent) => {
    const { id } = event.params;

    if (!id) {
      return { success: false };
    }

    console.log("delete", id);

    const deleteAdvert = graphql(`
      mutation deleteAdvert($advertId: Int!) {
        deleteAdvert(advertId: $advertId) {
          id
        }
      }
    `);

    const res = await deleteAdvert.mutate(
      { advertId: parseInt(id) },
      { event },
    );

    console.log(res);

    if (!res.errors && res.data) {
      redirect(302, "/");
    } else {
      return { success: false };
    }
  },
};

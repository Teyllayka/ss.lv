import { HoudiniClient } from "$houdini";

interface Session {
  user?: string; // or whatever type the user field should be
}

export default new HoudiniClient({
  url: "http://localhost:80",

  fetchParams({ session }: { session?: Session | null }) {
    return {
      headers: {
        Authorization: `Bearer ${session?.user}`,
      },
    };
  },
});

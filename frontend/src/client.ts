import { HoudiniClient } from "$houdini";
import { env } from "$env/dynamic/public";

interface Session {
  accessToken?: string;
}

export default new HoudiniClient({
  url: env.PUBLIC_API_URL || "http://127.0.0.1:90",

  fetchParams({ session }: { session?: Session | null }) {
    return {
      headers: {
        Authorization: `Bearer ${session?.accessToken}`,
      },
    };
  },
});

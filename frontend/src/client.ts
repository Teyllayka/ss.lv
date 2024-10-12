import { HoudiniClient } from "$houdini";

interface Session {
	accessToken?: string;
}

export default new HoudiniClient({
	url: "http://0.0.0.0:9000",

	fetchParams({ session }: { session?: Session | null }) {
		return {
			headers: {
				Authorization: `Bearer ${session?.accessToken}`,
			},
		};
	},
});

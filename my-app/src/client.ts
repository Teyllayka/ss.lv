import { HoudiniClient } from "$houdini";

interface Session {
	accessToken?: string;
}

export default new HoudiniClient({
	url: "http://localhost:80",

	fetchParams({ session }: { session?: Session | null }) {
		return {
			headers: {
				Authorization: `Bearer ${session?.accessToken}`,
			},
		};
	},
});

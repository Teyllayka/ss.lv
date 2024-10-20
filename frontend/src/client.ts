import { HoudiniClient } from "$houdini";
const apiUrl = import.meta.env.VITE_API_URL;

interface Session {
	accessToken?: string;
}

export default new HoudiniClient({
	url: apiUrl || "http://localhost:80",

	fetchParams({ session }: { session?: Session | null }) {
		return {
			headers: {
				Authorization: `Bearer ${session?.accessToken}`,
			},
		};
	},
});

import { redirect } from "@sveltejs/kit";

export function load({ cookies }: any) {
	const logedIn = cookies.get("accessToken");

	if (!logedIn) {
		return redirect(302, "/login");
	}
}

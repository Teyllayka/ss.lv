import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ cookies }) => { 
    const accessToken = cookies.get("accessToken");
    const response = await fetch(`http://localhost:4000/are-unread`, {
        method: "GET",
        headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${accessToken}`,
        },
    });
    const data = await response.json();
    if (response.status === 401) {
        return new Response(null, { status: 401 });
    }
    return new Response(JSON.stringify(data), { status: response.status });
}

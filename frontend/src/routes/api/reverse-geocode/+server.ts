import type { RequestHandler } from "@sveltejs/kit";

const cache = new Map<string, string>();

export const GET: RequestHandler = async ({ url }) => {
  const lat = url.searchParams.get("lat");
  const lon = url.searchParams.get("lon");

  if (!lat || !lon) {
    return new Response(
      JSON.stringify({ error: "lat and lon query parameters are required." }),
      { status: 400, headers: { "Content-Type": "application/json" } },
    );
  }

  const key = `${lat},${lon}`;

  if (cache.has(key)) {
    return new Response(JSON.stringify({ location: cache.get(key) }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  }

  try {
    const response = await fetch(
      `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lon}&accept-language=en`,
    );
    const data = await response.json();

    const location =
      data.address?.road && data.address?.city
        ? `${data.address.road}, ${data.address.city}`
        : "Unknown location";

    cache.set(key, location);

    return new Response(JSON.stringify({ location }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    console.error("Error fetching geocode data:", error);
    return new Response(
      JSON.stringify({ error: "Error fetching geocode data" }),
      { status: 500, headers: { "Content-Type": "application/json" } },
    );
  }
};

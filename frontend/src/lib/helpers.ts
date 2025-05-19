export function clickOutside(node: HTMLElement) {
  const handleClick = (event: MouseEvent) => {
    if (!node.contains(event.target as Node)) {
      node.dispatchEvent(new CustomEvent("click_outside"));
    }
  };

  document.addEventListener("click", handleClick, true);

  return {
    destroy() {
      document.removeEventListener("click", handleClick, true);
    },
  };
}

export function capitalizeFirstLetter(message: string) {
  return message.charAt(0).toUpperCase() + message.slice(1);
}

export function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  const year = date.getFullYear();
  const month = date.toLocaleString("en-US", { month: "short" });
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");

  return `${year} ${month} ${day} ${hours}:${minutes}`;
}

export function renderStars(rating: number) {
  const stars = Array.from({ length: 5 }, (_, i) => ({
    isFilled: i < Math.floor(rating),
  }));
  return stars;
}

export function calculateDistance(aa: any, bb: any) {
  const [lat1, lon1] = aa;
  const [lat2, lon2] = bb;

  const R = 6371000;
  const dLat = ((lat2 - lat1) * Math.PI) / 180;
  const dLon = ((lon2 - lon1) * Math.PI) / 180;
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos((lat1 * Math.PI) / 180) *
      Math.cos((lat2 * Math.PI) / 180) *
      Math.sin(dLon / 2) *
      Math.sin(dLon / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  return Math.round((R * c) / 1000);
}



export async function uploadFile(file: File): Promise<string> {
  const form = new FormData();
  form.append("file", file);
  const res = await fetch("https://gachi.gay/api/upload", {
    method: "POST",
    body: form,
  });
  const json = await res.json();
  return json.link;
}
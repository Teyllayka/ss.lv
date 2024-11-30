export const _SearchVariables = ({ url }: { url: URL }) => {
  const searchQuery = url.searchParams.get("q") || "";
  const region = url.searchParams.get("region") || "";
  const category = url.searchParams.get("category") || "";
  return {
    title: searchQuery,
    region: region,
    category: category,
    offset: 0,
  };
};

export const _SearchVariables = ({ url }: { url: URL }) => {
  const searchQuery = url.searchParams.get("q") || "";
  const category = url.searchParams.get("category") || "";
  return {
    title: searchQuery,
    category: category,
    offset: 0,
  };
};

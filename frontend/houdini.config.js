/** @type {import('houdini').ConfigFile} */
const config = {
  schemaPath: "./schema.graphql",
  watchSchema: {
    url: "http://127.0.0.1:80",
  },
  plugins: {
    "houdini-svelte": {},
  },
  scalars: {
    NaiveDateTime: {
      type: "Date",
    },
    JSON: {
      type: "Record<string, unknown>",
    },
  },
};

export default config;

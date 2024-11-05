/** @type {import('houdini').ConfigFile} */
const config = {
	schemaPath: "./schema.graphql",
	watchSchema: {
		url: "http://localhost:90",
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

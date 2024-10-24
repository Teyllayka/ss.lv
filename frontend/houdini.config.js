/** @type {import('houdini').ConfigFile} */
const config = {
	schemaPath: "./schema.graphql",
	watchSchema: {
		url: typeof import.meta.env !== 'undefined' && import.meta.env.VITE_API_URL 
            ? import.meta.env.VITE_API_URL 
            : "http://localhost:80",
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

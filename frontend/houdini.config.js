/// <references types="houdini-svelte">
// import { PUBLIC_BACKEND_URI } from '$env/static/public';

/** @type {import('houdini').ConfigFile} */
const config = {
	schemaPath: "./schema.graphql",
	watchSchema: {
		url: "http://0.0.0.0:9000",
	},
	plugins: {
		"houdini-svelte": {},
	},
	scalars: {
		NaiveDateTime: {
			type: "Date",
		},
	},
};

export default config;

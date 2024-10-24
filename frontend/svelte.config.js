import adapter from "@sveltejs/adapter-auto";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		alias: {
			$houdini: "./$houdini",
		},
		csp: {
			mode: "auto",
			directives: {
				"script-src": ["self"],
				"style-src": ["self"],
			},
		},
	},
};

export default config;

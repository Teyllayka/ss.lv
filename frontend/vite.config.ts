import { paraglide } from "@inlang/paraglide-sveltekit/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import houdini from "houdini/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [
		paraglide({ project: "./project.inlang", outdir: "./src/lib/paraglide" }),
		houdini(),
		sveltekit(),
	],
});

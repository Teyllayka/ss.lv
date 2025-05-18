import { paraglide } from "@inlang/paraglide-sveltekit/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import houdini from "houdini/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    houdini(),
    paraglide({ project: "./project.inlang", outdir: "./src/lib/paraglide",  }),
    sveltekit(),
  ],
  build: {
    sourcemap: false,
  }
  //   server: {
  //     proxy: {
  //       "/backend": {
  //         target: process.env.VITE_API_URL || "http://localhost:80",
  //         changeOrigin: true,
  //         rewrite: (path) => path.replace(/^\/backend/, ""),
  //       },
  //     },
  //   },
});

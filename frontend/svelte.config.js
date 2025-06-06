import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
    alias: {
      $houdini: "./$houdini",
    },
    // csp: {
    //   mode: "auto",
    //   directives: {
    //     "style-src": [
    //       "'self'",
    //       "https://fonts.googleapis.com",
    //       "https://fonts.gstatic.com",
    //       "unsafe-inline",
    //       "https://unpkg.com",
    //     ],
    //     "script-src": ["'self'", "'unsafe-eval'"],
    //     "frame-ancestors": ["'self'"],
    //     "manifest-src": ["'self'"],
    //     "font-src": ["'self'", "https://fonts.gstatic.com"],
    //     "connect-src": [
    //       "'self'",
    //       "http://localhost:90",
    //       "http://localhost:80",
    //       "http://127.0.0.1:80",
    //       "http://127.0.0.1:90",
    //       "http://localhost:4000",
    //       "ws://localhost:4000",
    //       "nominatim.openstreetmap.org",
    //       "https://api.emailjs.com",
    //       "https://unpkg.com",
    //     ],
    //     "frame-src": ["'self'"],
    //     "object-src": ["'self'"],
    //     "form-action": ["'self'"],
    //     //   "media-src": ["'self'"],
    //     //   "img-src": ["'self'", "data:"],
    //   },
    // },
  },
};

export default config;

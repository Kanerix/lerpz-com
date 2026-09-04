import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        // The company site is fully prerendered — every route ends up as a
        // plain HTML file, so it can be hosted by any static file server.
        adapter: adapter({
            pages: "build",
            assets: "build",
            fallback: undefined,
            precompress: false,
            strict: true,
        }),
        alias: {
            $lib: "./src/lib",
        },
        paths: {
            base: process.env.BASE_PATH ?? "",
        },
    },
};

export default config;

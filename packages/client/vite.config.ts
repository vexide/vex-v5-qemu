import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

import path from "node:path";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [svelte()],
    resolve: {
        alias: {
            "~/": `${path.resolve("src")}/`,
        },
    },
});

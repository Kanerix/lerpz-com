import { defineConfig } from "orval";

export default defineConfig({
    "portal-api": {
        output: {
            mode: "tags-split",
            target: "./src/lib/api",
            schemas: "./src/lib/api/models",
            client: "svelte-query",
            override: {
                header: () => "// @ts-nocheck\n",
                mutator: {
                    path: "./src/lib/http/orval-mutator.ts",
                    name: "customFetch",
                },
                query: {
                    useQuery: true,
                    useMutation: true,
                    signal: true,
                },
            },
        },
        input: {
            target: "http://localhost:3001/api/openapi.json",
        },
    },
});

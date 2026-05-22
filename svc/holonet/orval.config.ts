import { defineConfig } from "orval";

export default defineConfig({
    "portal-api": {
        output: {
            mode: "tags-split",
            target: "./src/services/api",
            schemas: "./src/services/api/models",
            client: "react-query",
            override: {
                header: () => "// @ts-nocheck\n",
                mutator: {
                    path: "./src/lib/orval-mutator.ts",
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

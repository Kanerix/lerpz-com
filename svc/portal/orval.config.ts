import { defineConfig } from "orval";

export default defineConfig({
  petstore: {
    output: {
      mode: "tags-split",
      target: "./src/services/api",
      schemas: "./src/services/api/models",
      mock: true,
    },
    input: {
      target: "http://localhost:3001/api/openapi.json",
    },
  },
});

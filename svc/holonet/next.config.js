import path from "node:path";
import { fileURLToPath } from "node:url";
import { createJiti } from "jiti";

const jiti = createJiti(import.meta.url);
await jiti.import("./src/lib/env");

const __dirname = fileURLToPath(new URL(".", import.meta.url));

/** @type {import('next').NextConfig} */
const nextConfig = {
    output: "standalone",
    cacheComponents: true,
    pageExtensions: ["tsx", "ts", "jsx", "js"],
    transpilePackages: ["@t3-oss/env-nextjs", "@t3-oss/env-core"],
    outputFileTracingRoot: path.join(__dirname, "../../"),
    devIndicators: {
        position: "bottom-right",
    },
};

export default nextConfig;

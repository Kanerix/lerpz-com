import path from "node:path";
import { fileURLToPath } from "node:url";
import createMDX from "@next/mdx";
import { createJiti } from "jiti";

const jiti = createJiti(import.meta.url);
await jiti.import("./src/lib/env");

const __dirname = fileURLToPath(new URL(".", import.meta.url));

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  cacheComponents: true,
  pageExtensions: ["tsx", "ts", "jsx", "js", "mdx", "md"],
  transpilePackages: ["@t3-oss/env-nextjs", "@t3-oss/env-core"],
  outputFileTracingRoot: path.join(__dirname, "../../"),
  devIndicators: {
    position: "bottom-right",
  },
};

const withMDX = createMDX({
  extension: /\.(md|mdx)$/,
});

export default withMDX(nextConfig);

/** @type {import("next").NextConfig} */
var BASE_PATH = require("./app_config").BASE_PATH;
const withPWA = require("next-pwa")({
  dest: "public",
  skipWaiting: true, // Turn this to false once you're ready to deploy a banner to develop update prompt.
  mode: process.env.NODE_ENV === "production" ? "production" : "development" // This will create worker-box production build.
});

const isProduction = process.env.NODE_ENV === "production";

function getVersion() {
  const date = new Date();

  return (
    "v" +
    date.getUTCFullYear().toString() +
    (date.getUTCMonth() + 1).toString().padStart(2, "0") +
    date.getUTCDay().toString().padStart(2, "0") +
    date.getUTCHours().toString().padStart(2, "0") +
    date.getUTCMinutes().toString().padStart(2, "0")
  );
}

const withTM = require("next-transpile-modules")([
  "substrate-react",
  "shared",
  "tokens",
  "defi-interfaces"
]);

const nextConfig = {
  images: {
    unoptimized: true
  },
  basePath: isProduction ? BASE_PATH : "",
  reactStrictMode: true,
  env: {
    SUBSTRATE_PROVIDER_URL_KUSAMA_2019:
    process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019,
    SUBSTRATE_PROVIDER_URL_KUSAMA: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA,
    SUBSQUID_URL: process.env.SUBSQUID_URL,
    WEBSITE_VERSION: getVersion()
  },
  webpack(config) {
    config.module.rules.push({
      test: /\.svg$/,
      use: ["@svgr/webpack"]
    });

    return config;
  }
};

module.exports = withPWA(withTM(nextConfig));

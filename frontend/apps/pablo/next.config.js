/** @type {import("next").NextConfig} */
const withPWA = require("next-pwa")({
  dest: "public",
  skipWaiting: true, // Turn this to false once you're ready to deploy a banner to develop update prompt.
  mode: process.env.NODE_ENV === "production" ? "production" : "development", // This will create worker-box production build.
});

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
  "@polkadot/react-identicon",
  "substrate-react",
  "shared",
  "tokens",
  "defi-interfaces",
  "endpoints",
  "wallet",
  "bi-lib",
]);

const nextConfig = {
  images: {
    unoptimized: true,
  },
  reactStrictMode: true,
  env: {
    SUBSTRATE_PROVIDER_URL_KUSAMA_2019:
      process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019,
    SUBSTRATE_PROVIDER_URL_KUSAMA: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA,
    SUBSQUID_URL: process.env.SUBSQUID_URL,
    WEBSITE_VERSION: getVersion(),
    SUBSTRATE_PROVIDER_URL_STATEMINE:
      process.env.SUBSTRATE_PROVIDER_URL_STATEMINE,
    COINGECKO_KEY: process.env.COINGECKO_KEY,
  },
  webpack(config) {
    config.module.rules.push({
      test: /\.svg$/,
      use: ["@svgr/webpack"],
    });

    return config;
  },
};

module.exports = withPWA(withTM(nextConfig));

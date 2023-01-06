/** @type {import("next").NextConfig} */
const withPWA = require("next-pwa")({
  dest: "public",
  skipWaiting: true, // Turn this to false once you're ready to deploy a banner to develop update prompt.
  mode: process.env.NODE_ENV === "production" ? "production" : "development", // This will create worker-box production build.
});
const { withSentryConfig } = require("@sentry/nextjs");
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
  sentry: {
    // Use `hidden-source-map` rather than `source-map` as the Webpack `devtool`
    // for client-side builds. (This will be the default starting in
    // `@sentry/nextjs` version 8.0.0.) See
    // https://webpack.js.org/configuration/devtool/ and
    // https://docs.sentry.io/platforms/javascript/guides/nextjs/manual-setup/#use-hidden-source-map
    // for more information.
    hideSourceMaps: true,
    disableServerWebpackPlugin: true,
    disableClientWebpackPlugin: true,
  },
  images: {
    unoptimized: true,
  },
  reactStrictMode: true,
  env: {
    SUBSTRATE_PROVIDER_URL_KUSAMA_2019:
      process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019,
    SUBSTRATE_PROVIDER_URL_KUSAMA: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA,
    SUBSQUID_URL: process.env.SUBSQUID_URL,
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

module.exports = withSentryConfig(withPWA(withTM(nextConfig)));

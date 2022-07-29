/** @type {import('next').NextConfig} */
const withPWA = require('next-pwa')

const withTM = require("next-transpile-modules")([
  "substrate-react",
  "shared",
  "tokens"
]);

const nextConfig = {
  reactStrictMode: true,
  env: {
    SUBSTRATE_PROVIDER_URL_KUSAMA_2019: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019,
    SUBSTRATE_PROVIDER_URL_KUSAMA: process.env.SUBSTRATE_PROVIDER_URL_KUSAMA,
    SUBSQUID_URL: process.env.SUBSQUID_URL
  },
  pwa: {
    dest: 'public',
    skipWaiting: true, // Turn this to false once you're ready to deploy a banner to develop update prompt.
    mode: process.env.NODE_ENV === 'production' ? 'production' : 'development', // This will create worker-box production build.
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

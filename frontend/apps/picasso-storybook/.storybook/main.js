const path = require("path");

module.exports = {
  stories: [
    "../stories/**/*.stories.mdx",
    "../stories/**/*.stories.@(js|jsx|ts|tsx)",
  ],
  core: {
    builder: "webpack5",
  },
  webpackFinal: async (config) => {
    // Transpile @integrations-lib module to ES5
    config.module.rules[0].exclude = [
      /node_modules\/(?!(@integrations-lib\/core)\/)/,
    ];
    config.module.rules[0].resolve = {
      fullySpecified: false,
    };

    config.resolve.alias = {
      ...config.resolve.alias,
      "@/apollo": path.resolve(__dirname, "..", "..", "picasso", "apollo"),
      "@/pages": path.resolve(__dirname, "..", "..", "picasso", "pages"),
      "@/components": path.resolve(
        __dirname,
        "..",
        "..",
        "picasso",
        "components"
      ),
      "@/styles": path.resolve(__dirname, "..", "..", "picasso", "styles"),
      "@/utils": path.resolve(__dirname, "..", "..", "picasso", "utils"),
      "@/contexts": path.resolve(__dirname, "..", "..", "picasso", "contexts"),
      "@/hooks": path.resolve(__dirname, "..", "..", "picasso", "hooks"),
      "@/defi": path.resolve(__dirname, "..", "..", "picasso", "defi"),
      "@/stores": path.resolve(__dirname, "..", "..", "picasso", "stores"),
    };

    return config;
  },
  staticDirs: ["../../picasso/public"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-controls",
    {
      name: "storybook-addon-next",
      options: {
        nextConfigPath: path.resolve(
          __dirname,
          "..",
          "..",
          "picasso",
          "next.config.js"
        ),
      },
    },
  ],
  framework: "@storybook/react",
};

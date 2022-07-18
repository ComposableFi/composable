const path = require("path");

module.exports = {
  stories: [
    "../stories/**/*.stories.mdx",
    "../stories/**/*.stories.@(js|jsx|ts|tsx)"
  ],
  core: {
    builder: "webpack5"
  },
  webpackFinal: async (config) => {
    // Transpile @integrations-lib module to ES5
    config.module.rules[0].exclude = [/node_modules\/(?!(@integrations-lib\/core)\/)/];
    config.module.rules[0].resolve = {
      fullySpecified: false
    };

    config.resolve.alias = {
      ...config.resolve.alias,
      "@/pages": path.resolve(__dirname, "..", "..", "app", "pages"),
      "@/components": path.resolve(__dirname, "..", "..", "app", "components"),
      "@/styles": path.resolve(__dirname, "..", "..", "app", "styles"),
      "@/utils": path.resolve(__dirname, "..", "..", "app", "utils"),
      "@/contexts": path.resolve(__dirname, "..", "..", "app", "contexts"),
      "@/hooks": path.resolve(__dirname, "..", "..", "app", "hooks"),
      "@/defi": path.resolve(__dirname, "..", "..", "app", "defi"),
      "@/stores": path.resolve(__dirname, "..", "..", "app", "stores")
    };

    return config;
  },
  staticDirs: ["../../app/public"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-knobs",
    {
      name: "storybook-addon-next",
      options: {
        nextConfigPath: path.resolve(__dirname, "..", "..", "app", "next.config.js")
      }
    }
  ],
  framework: "@storybook/react"
};

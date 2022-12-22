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
      "@/apollo": path.resolve(__dirname, "..", "..", "pablo", "apollo"),
      "@/pages": path.resolve(__dirname, "..", "..", "pablo", "pages"),
      "@/components": path.resolve(
        __dirname,
        "..",
        "..",
        "pablo",
        "components"
      ),
      "@/styles": path.resolve(__dirname, "..", "..", "pablo", "styles"),
      "@/utils": path.resolve(__dirname, "..", "..", "pablo", "utils"),
      "@/contexts": path.resolve(__dirname, "..", "..", "pablo", "contexts"),
      "@/hooks": path.resolve(__dirname, "..", "..", "pablo", "hooks"),
      "@/defi": path.resolve(__dirname, "..", "..", "pablo", "defi"),
      "@/stores": path.resolve(__dirname, "..", "..", "pablo", "stores"),
      "@/store": path.resolve(__dirname, "..", "..", "pablo", "store"),
      "@/updaters": path.resolve(__dirname, "..", "..", "pablo", "updaters"),
      "@/constants": path.resolve(__dirname, "..", "..", "pablo", "constants"),
    };

    return config;
  },
  staticDirs: ["../../pablo/public"],
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
          "pablo",
          "next.config.js"
        ),
      },
    },
  ],
  framework: "@storybook/react",
};

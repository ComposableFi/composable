module.exports = {
  stories: [
    "../stories/**/*.stories.mdx",
    "../stories/**/*.stories.@(js|jsx|ts|tsx)",
  ],
  staticDirs: ["../../app/public"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-knobs",
    "storybook-addon-next-router",
  ],
  framework: "@storybook/react",
  features: {
    storyStoreV7: true,
  },
  core: {
    builder: "webpack5",
  },
};

module.exports = {
  stories: [
    "../stories/**/*.stories.@(js|jsx|ts|tsx)",
  ],
  staticDirs: ["../../pablo/public"],
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

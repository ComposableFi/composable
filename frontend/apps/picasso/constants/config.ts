const config = {
  governanceUrl: "https://picasso.polkassembly.io/",
  twitterUrl: "https://twitter.com/picasso_network",
  mediumUrl: "https://medium.com/picasso-network",
  discordUrl: "https://discord.gg/composable",
  pabloUrl: "https://app.pablo.finance",
  analytics: {
    mixpanelToken: process.env.NEXT_PUBLIC_MIXPANEL_TOKEN || "",
    gaToken: process.env.NEXT_PUBLIC_GA_TOKEN || "",
  },
} as const;

export default config;

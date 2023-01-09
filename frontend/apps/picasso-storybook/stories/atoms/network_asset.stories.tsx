import { NetworkAsset, NetworkAssetProps } from "picasso/components";

import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import config from "picasso/constants/config";

const NetworkAssetsStories = (props: NetworkAssetProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
  };

  return (
    <Box sx={boxStyle}>
      <NetworkAsset {...props} />
    </Box>
  );
};
export default {
  title: "atoms/NetworkAsset",
  component: NetworkAssetsStories,
  argTypes: {
    networkId: {
      options: config.evm.networkIds,
      control: {
        type: "select",
      },
    },
  },
};

const defaultArgs = {
  networkId: config.evm.networkIds[0],
  iconSize: 24,
};

const Template: Story<typeof NetworkAssetsStories> = (args) => (
  <NetworkAssetsStories {...defaultArgs} {...args} />
);

export const NetworkAssets = Template.bind({});
NetworkAssets.args = defaultArgs;

import { 
  NetworkAssetProps,
  NetworkAsset 
} from "picasso/components";
import { NETWORK_IDS } from "picasso/defi/Networks";

import { 
  Box, 
  SxProps, 
} from "@mui/material";
import { Story } from "@storybook/react";

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
      options: NETWORK_IDS,
      control: {
        type: "select",
      },
    },
  }
};

const defaultArgs = {
  networkId: NETWORK_IDS[0],
  iconSize: 24,
};

const Template: Story<typeof NetworkAssetsStories> = (args) => (
  <NetworkAssetsStories {...defaultArgs} {...args} />
);

export const NetworkAssets = Template.bind({});
NetworkAssets.args = defaultArgs

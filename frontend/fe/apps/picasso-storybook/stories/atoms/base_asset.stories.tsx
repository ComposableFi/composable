import { 
  BaseAssetProps,
  BaseAsset 
} from "picasso/components";
import { 
  Box, 
  SxProps, 
} from "@mui/material";
import { Story } from "@storybook/react";

const BaseAssetsStories = (props: BaseAssetProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
  };

  return (
    <Box sx={boxStyle}>
      <BaseAsset {...props} />
    </Box>
  );
};
export default {
  title: "atoms/BaseAsset",
  component: BaseAsset,
};

const defaultArgs = {
  icon: '/tokens/eth-mainnet.svg',
  label: 'ETH',
  iconSize: 24,
};

const Template: Story<typeof BaseAssetsStories> = (args) => (
  <BaseAssetsStories {...defaultArgs} {...args} />
);

export const DefaultBaseAsset = Template.bind({});
DefaultBaseAsset.args = defaultArgs;

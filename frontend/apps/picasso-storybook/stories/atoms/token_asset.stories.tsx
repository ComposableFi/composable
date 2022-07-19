import { TokenAssetProps, TokenAsset } from "picasso/components";
import { TOKEN_IDS } from "tokens";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const TokenAssetsStories = (props: TokenAssetProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto"
  };

  return (
    <Box sx={boxStyle}>
      <TokenAsset {...props} />
    </Box>
  );
};
export default {
  title: "atoms/TokenAsset",
  component: TokenAsset,

  argTypes: {
    tokenId: {
      options: TOKEN_IDS,
      control: {
        type: "select"
      }
    }
  }
};

const defaultArgs = {
  tokenId: TOKEN_IDS[0],
  iconSize: 24
};

const Template: Story<typeof TokenAssetsStories> = args => (
  <TokenAssetsStories {...defaultArgs} {...args} />
);

export const TokenAssets = Template.bind({});
TokenAssets.args = defaultArgs;

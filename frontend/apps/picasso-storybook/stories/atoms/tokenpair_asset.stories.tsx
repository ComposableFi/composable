import { TokenPairAssetProps, TokenPairAsset } from "picasso/components";
import { TOKEN_IDS } from "tokens";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const TokenPairAssetsStories = (props: TokenPairAssetProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto"
  };

  return (
    <Box sx={boxStyle}>
      <TokenPairAsset {...props} />
    </Box>
  );
};
export default {
  title: "atoms/TokenPairAsset",
  component: TokenPairAsset,

  argTypes: {
    tokenId: {
      options: TOKEN_IDS,
      control: {
        type: "select"
      }
    },
    toTokenId: {
      options: TOKEN_IDS,
      control: {
        type: "select"
      }
    }
  }
};

const defaultArgs = {
  tokenId: TOKEN_IDS[0],
  toTokenId: TOKEN_IDS[1],
  iconSize: 24
};

const Template: Story<typeof TokenPairAssetsStories> = args => (
  <TokenPairAssetsStories {...defaultArgs} {...args} />
);

export const TokenPairAssets = Template.bind({});
TokenPairAssets.args = defaultArgs;

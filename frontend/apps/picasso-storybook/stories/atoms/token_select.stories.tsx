import { TokenSelect, TokenSelectProps } from "picasso/components";
import { TokenId, TOKEN_IDS } from "tokens";

import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import { useState } from "react";

const TokenSelectsStories = (props: TokenSelectProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 2
  };

  const [value, setValue] = useState<TokenId>(TOKEN_IDS[0]);

  return (
    <Box sx={boxStyle}>
      <TokenSelect value={value} setValue={setValue} {...props} />
      <TokenSelect value={TOKEN_IDS[0]} {...props} disabled />
    </Box>
  );
};
export default {
  title: "atoms/TokenSelect",
  component: TokenSelect
};

const Template: Story<typeof TokenSelectsStories> = args => (
  <TokenSelectsStories {...args} />
);

const mainLabelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: "Tooltip master here"
  }
};

export const TokenSelects = Template.bind({});
TokenSelects.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps
  },
  options: TOKEN_IDS.map(tokenId => ({ tokenId: tokenId }))
};

export const CenteredTokenSelects = Template.bind({});
CenteredTokenSelects.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps
  },
  centeredLabel: true,
  options: TOKEN_IDS.map(tokenId => ({ tokenId: tokenId }))
};

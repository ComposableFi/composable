import {
  TokenDropdownCombinedInput,
  TokenDropdownCombinedInputProps,
  TokenSelectProps,
} from "picasso/components";
import { TOKEN_IDS, TOKENS } from "tokens";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import { AssetId } from "picasso/defi/polkadot/types";

const TokenDropdownCombinedInputsStories = (
  props: TokenDropdownCombinedInputProps
) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 2,
  };

  return (
    <Box sx={boxStyle}>
      <TokenDropdownCombinedInput value="Input text" {...props} />
      <TokenDropdownCombinedInput placeholder="Placeholder text" {...props} />
      <TokenDropdownCombinedInput value="Disabled text" {...props} disabled />
      <TokenDropdownCombinedInput value="Error text" {...props} error />
    </Box>
  );
};
export default {
  title: "molecules/TokenDropdownCombinedInput",
  component: TokenDropdownCombinedInput,
};

const Template: Story<typeof TokenDropdownCombinedInputsStories> = (args) => (
  <TokenDropdownCombinedInputsStories {...args} />
);

const mainLabelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: "Tooltip master here",
  },
};

const balanceLabelProps = {
  label: "Balance:",
  LabelTypographyProps: {},
  balanceText: "435 KSM",
  BalanceTypographyProps: {},
};

const selectProps: TokenSelectProps = {
  value: TOKEN_IDS[0],
  options: Object.values(TOKENS).map((token) => ({
    tokenId: token.id as AssetId,
    icon: token.icon,
    symbol: token.symbol,
    disabled: false,
  })),
};

export const TokenDropdownCombinedInputs = Template.bind({});
TokenDropdownCombinedInputs.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: selectProps,
};

export const TokenDropdownCombinedInputsWithButton = Template.bind({});
TokenDropdownCombinedInputsWithButton.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: selectProps,
  buttonLabel: "Max",
};

export const LabeledTokenDropdownCombinedInputsWithBalance = Template.bind({});
LabeledTokenDropdownCombinedInputsWithBalance.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
    balanceLabelProps: balanceLabelProps,
  },
  CombinedSelectProps: selectProps,
};

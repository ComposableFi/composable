import { BigNumberInput, BigNumberInputProps } from "pablo/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import BigNumber from "bignumber.js";

const BigNumberInputsStories = (props: BigNumberInputProps) => {
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
      <BigNumberInput value={new BigNumber(355.33)} {...props}/>
      <BigNumberInput placeholder="0.00" {...props}/>
      <BigNumberInput value={new BigNumber(355.33)} {...props} disabled />
      <BigNumberInput value={new BigNumber(355.33)} {...props} error />
      <BigNumberInput value={new BigNumber(355.33)} {...props} alert />
    </Box>
  );
};
export default {
  title: "atoms/BigNumberInput",
  component: BigNumberInput,
};

const Template: Story<typeof BigNumberInputsStories> = (args) => (
  <BigNumberInputsStories {...args} />
);

const labelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: 'Tooltip master here',
  }
};

export const TextOnly = Template.bind({});
TextOnly.args = {
  LabelProps: labelProps,
}

export const TextInsideButton = Template.bind({});
TextInsideButton.args = {
  buttonLabel: 'Max',
  LabelProps: labelProps,
}

export const StartAdornmentAsset = Template.bind({});
StartAdornmentAsset.args = {
  startAdornmentAsset: {
    icon: "/dummy/token.svg",
    label: "Token",
  },
  buttonLabel: 'Max',
  LabelProps: labelProps,
}

export const TextAndReference = Template.bind({});
TextAndReference.args = {
  referenceText: 'Reference Text',
  LabelProps: labelProps,
}

export const LabeledInputsWithBalance = Template.bind({});
LabeledInputsWithBalance.args = {
  LabelProps: {
    ...labelProps,
    BalanceProps: {
      title: "Balance:",
      TitleTypographyProps: {},
      balance: "435 KSM",
      BalanceTypographyProps: {},
    },
  },
}

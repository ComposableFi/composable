import {
  DropdownCombinedInput,
  DropdownCombinedInputProps,
  SelectProps,
} from "picasso/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const DropdownCombinedInputsStories = (props: DropdownCombinedInputProps) => {
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
      <DropdownCombinedInput value="Input text" {...props} />
      <DropdownCombinedInput placeholder="Placeholder text" {...props} />
      <DropdownCombinedInput value="Disabled text" {...props} disabled />
      <DropdownCombinedInput value="Error text" {...props} error />
    </Box>
  );
};
export default {
  title: "molecules/DropdownCombinedInput",
  component: DropdownCombinedInput,
};

const Template: Story<typeof DropdownCombinedInputsStories> = (args) => (
  <DropdownCombinedInputsStories {...args} />
);

const selectProps: SelectProps = {
  value: "select1",
  options: [
    {
      value: "select1",
      label: "Select 1",
    },
    {
      value: "select2",
      label: "Select 2",
    },
    {
      value: "select3",
      label: "Select 3",
    },
    {
      value: "select4",
      label: "Select 4",
    },
    {
      value: "select5",
      label: "Select 5",
    },
  ],
};

const iconSelectProps: SelectProps = {
  value: "select1",
  options: [
    {
      value: "select1",
      label: "Select 1",
      icon: "/tokens/eth-mainnet.svg",
    },
    {
      value: "select2",
      label: "Select 2",
      icon: "/tokens/eth-mainnet.svg",
    },
    {
      value: "select3",
      label: "Select 3",
      icon: "/tokens/eth-mainnet.svg",
    },
    {
      value: "select4",
      label: "Select 4",
      icon: "/tokens/eth-mainnet.svg",
      disabled: true,
    },
    {
      value: "select5",
      label: "Select 5",
      icon: "/tokens/eth-mainnet.svg",
    },
  ],
};

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

export const DropdownCombinedInputs = Template.bind({});
DropdownCombinedInputs.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: selectProps,
};

export const DropdownCombinedInputsWithButton = Template.bind({});
DropdownCombinedInputsWithButton.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: selectProps,
  buttonLabel: "Max",
};

export const IconDropdownCombinedInputs = Template.bind({});
IconDropdownCombinedInputs.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: iconSelectProps,
};

export const IconDropdownCombinedInputsWithButton = Template.bind({});
IconDropdownCombinedInputsWithButton.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  CombinedSelectProps: iconSelectProps,
  buttonLabel: "Max",
};

export const DropdownCombinedInputsWithBalance = Template.bind({});
DropdownCombinedInputsWithBalance.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
    balanceLabelProps: balanceLabelProps,
  },
  CombinedSelectProps: selectProps,
};

import { Select, SelectProps } from "picasso/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";
import { useState } from "react";

const SelectsStories = (props: SelectProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 2,
  };

  const [value, setValue] = useState<string>("select1");

  return (
    <Box sx={boxStyle}>
      <Select value={value} setValue={setValue} {...props} />
      <Select value="select1" {...props} disabled />
    </Box>
  );
};
export default {
  title: "atoms/Select",
  component: Select,
};

const Template: Story<typeof SelectsStories> = (args) => (
  <SelectsStories {...args} />
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

export const TextOnlySelects = Template.bind({});
TextOnlySelects.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  searchable: true,
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

export const IconSelects = Template.bind({});
IconSelects.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  searchable: true,
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

export const CenteredSelects = Template.bind({});
CenteredSelects.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
  },
  searchable: true,
  centeredLabel: true,
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

export const LabeledSelectsWithBalance = Template.bind({});
LabeledSelectsWithBalance.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
    balanceLabelProps: balanceLabelProps,
  },
  searchable: true,
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

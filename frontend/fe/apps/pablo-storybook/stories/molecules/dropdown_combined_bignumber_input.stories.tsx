import { 
  DropdownCombinedBigNumberInput, 
  DropdownCombinedInputBigNumberProps, 
  SelectProps 
} from "pablo/components";
import BigNumber from "bignumber.js";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const DropdownCombinedBigNumberInputsStories = (props: DropdownCombinedInputBigNumberProps) => {
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
      <DropdownCombinedBigNumberInput value={new BigNumber(500.432)} {...props}/>
      <DropdownCombinedBigNumberInput placeholder="0.00" {...props}/>
      <DropdownCombinedBigNumberInput value={new BigNumber(500.432)} {...props} disabled />
      <DropdownCombinedBigNumberInput value={new BigNumber(500.432)} {...props} error />
    </Box>
  );
};
export default {
  title: "molecules/DropdownCombinedBigNumberInput",
  component: DropdownCombinedBigNumberInput,
};

const Template: Story<typeof DropdownCombinedBigNumberInputsStories> = (args) => (
  <DropdownCombinedBigNumberInputsStories {...args} />
);

const selectProps: SelectProps = {
  value: "select1",
  options: [
    {
      value: "select1",
      label: "Select 1"
    },
    {
      value: "select2",
      label: "Select 2"
    },
    {
      value: "select3",
      label: "Select 3"
    },
    {
      value: "select4",
      label: "Select 4"
    },
    {
      value: "select5",
      label: "Select 5"
    },
  ],
};

const iconSelectProps: SelectProps = {
  value: "select1",
  options: [
    {
      value: "select1",
      label: "Select 1",
      icon: '/dummy/token.svg',
    },
    {
      value: "select2",
      label: "Select 2",
      icon: '/dummy/token.svg',
    },
    {
      value: "select3",
      label: "Select 3",
      icon: '/dummy/token.svg',
    },
    {
      value: "select4",
      label: "Select 4",
      icon: '/dummy/token.svg',
      disabled: true,
    },
    {
      value: "select5",
      label: "Select 5",
      icon: '/dummy/token.svg',
    },
  ],
};

const labelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: 'Tooltip master here',
  }
};

export const DropdownCombinedInputs = Template.bind({});
DropdownCombinedInputs.args = {
  LabelProps: labelProps,
  CombinedSelectProps: selectProps,
};

export const DropdownCombinedInputsWithButton = Template.bind({});
DropdownCombinedInputsWithButton.args = {
  LabelProps: labelProps,
  CombinedSelectProps: selectProps,
  buttonLabel: 'Max',
};

export const IconDropdownCombinedInputs = Template.bind({});
IconDropdownCombinedInputs.args = {
  LabelProps: labelProps,
  CombinedSelectProps: iconSelectProps,
};

export const IconDropdownCombinedInputsWithButton = Template.bind({});
IconDropdownCombinedInputsWithButton.args = {
  LabelProps: labelProps,
  CombinedSelectProps: iconSelectProps,
  buttonLabel: 'Max',
};

export const DropdownCombinedInputsWithBalance = Template.bind({});
DropdownCombinedInputsWithBalance.args = {
  LabelProps: {
    ...labelProps,
    BalanceProps: {
      title: "Balance:",
      TitleTypographyProps: {},
      balance: "435 KSM",
      BalanceTypographyProps: {},
    },
  },
  CombinedSelectProps: selectProps,
};

import { Select, SelectProps } from "@/components";
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
      <Select value={value} setValue={setValue} {...props}/>
      <Select value="select1" {...props} disabled/>
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

const labelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: 'Tooltip master here',
  }
};

export const TextOnlySelects = Template.bind({});
TextOnlySelects.args = {
  LabelProps: labelProps,
  searchable: true,
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
}

export const IconSelects = Template.bind({});
IconSelects.args = {
  LabelProps: labelProps,
  searchable: true,
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

export const CenteredSelects = Template.bind({});
CenteredSelects.args = {
  LabelProps: labelProps,
  searchable: true,
  centeredLabel: true,
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

export const LabeledSelectsWithBalance = Template.bind({});
LabeledSelectsWithBalance.args = {
  LabelProps: {
    ...labelProps,
    BalanceProps: {
      title: "Balance:",
      TitleTypographyProps: {},
      balance: "435 KSM",
      BalanceTypographyProps: {},
    },
  },
  searchable: true,
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

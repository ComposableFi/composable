import { Input, InputProps } from "@/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const InputsStories = (props: InputProps) => {
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
      <Input value="Input text" {...props}/>
      <Input placeholder="Placeholder text" {...props}/>
      <Input value="Disabled text" {...props} disabled />
      <Input value="Error text" {...props} error />
      <Input value="Alert text" {...props} alert />
    </Box>
  );
};
export default {
  title: "atoms/Input",
  component: Input,
};

const Template: Story<typeof InputsStories> = (args) => (
  <InputsStories {...args} />
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

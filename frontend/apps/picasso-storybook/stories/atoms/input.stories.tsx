import { Input, InputProps } from "picasso/components";
import { TOKEN_IDS } from "tokens";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const InputsStories = (props: InputProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
    padding: 2
  };

  return (
    <Box sx={boxStyle}>
      <Input value="Input text" {...props} />
      <Input placeholder="Placeholder text" {...props} />
      <Input value="Disabled text" {...props} disabled />
      <Input value="Error text" {...props} error />
      <Input value="Alert text" {...props} alert />
    </Box>
  );
};
export default {
  title: "atoms/Input",
  component: Input
};

const Template: Story<typeof InputsStories> = args => (
  <InputsStories {...args} />
);

const mainLabelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: "Tooltip master here"
  }
};

const balanceLabelProps = {
  label: "Balance:",
  LabelTypographyProps: {},
  balanceText: "435 KSM",
  BalanceTypographyProps: {}
};

export const TextOnly = Template.bind({});
TextOnly.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps
  }
};

export const TextInsideButton = Template.bind({});
TextInsideButton.args = {
  buttonLabel: "Max",
  LabelProps: {
    mainLabelProps: mainLabelProps
  }
};

export const TokenInsideButton = Template.bind({});
TokenInsideButton.args = {
  tokenId: TOKEN_IDS[0],
  buttonLabel: "Max",
  LabelProps: {
    mainLabelProps: mainLabelProps
  }
};

export const TextAndReference = Template.bind({});
TextAndReference.args = {
  referenceText: "Reference Text",
  LabelProps: {
    mainLabelProps: mainLabelProps
  }
};

export const LabeledInputsWithBalance = Template.bind({});
LabeledInputsWithBalance.args = {
  LabelProps: {
    mainLabelProps: mainLabelProps,
    balanceLabelProps: balanceLabelProps
  }
};

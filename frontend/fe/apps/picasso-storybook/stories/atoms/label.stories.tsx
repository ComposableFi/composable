import { Label, LabelProps } from "picasso/components";
import { Box, SxProps } from "@mui/material";
import { Story } from "@storybook/react";

const LabelStories = (props: LabelProps) => {
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
      <Label {...props} />
    </Box>
  );
};
export default {
  title: "atoms/Label",
  component: Label,
};

const Template: Story<typeof LabelStories> = (args) => (
  <LabelStories {...args} />
);

const mainLabelProps = {
  label: "Label master here",
  TypographyProps: {},
  TooltipProps: {
    title: 'Tooltip master here',
  }
};

const balanceLabelProps = {
  label: "Balance:",
  LabelTypographyProps: {},
  balanceText: "435 KSM",
  BalanceTypographyProps: {},
};

export const TooltipLabels = Template.bind({});
TooltipLabels.args = {
  mainLabelProps: mainLabelProps,
  balanceLabelProps: {},
}

export const TooltipLabelsWithBalance = Template.bind({});
TooltipLabelsWithBalance.args = {
  mainLabelProps: mainLabelProps,
  balanceLabelProps: balanceLabelProps,
}

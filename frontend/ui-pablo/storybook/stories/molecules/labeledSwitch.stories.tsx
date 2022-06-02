import { LabeledSwitch, LabeledSwitchProps } from "@/components/Molecules";
import { Story } from "@storybook/react";
import { InfoOutlined } from "@mui/icons-material";

export default {
  title: "molecules/TextAndSwitches",
  component: LabeledSwitch,
  textFirst: {
    options: [true, false],
  },
};

const defaultArgs = {
  label: "Text element",
};

const Template: Story<LabeledSwitchProps> = (args) => (
  <LabeledSwitch {...args} {...defaultArgs} />
);

export const LabeledSwitches = Template.bind({});
LabeledSwitches.args = {
  textFirst: true,
};

export const LabeledSwitchesWithTooltip = Template.bind({});
LabeledSwitchesWithTooltip.args = {
  TooltipProps: {
    title: "Tooltip master here",
    children: <InfoOutlined />,
    open: true,
  },
  textFirst: true,
};

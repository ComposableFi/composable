import { Box } from "@mui/material";
import { ComponentMeta, ComponentStory } from "@storybook/react";

import { FeeDisplay } from "picasso/components/Molecules";

export default {
  title: "molecules/FeeDisplay",
  component: FeeDisplay,
  textFirst: {
    options: [true, false],
  },
} as ComponentMeta<typeof FeeDisplay>;

const Template: ComponentStory<typeof FeeDisplay> = (args) => (
  <FeeDisplay {...args} />
);

export const FeeDisplayWithoutTooltip = Template.bind({});
FeeDisplayWithoutTooltip.args = {
  label: "Fee",
  feeText: "345 ETH",
  textFirst: true,
};

export const FeeDisplayWithTooltip = Template.bind({});
FeeDisplayWithTooltip.args = {
  label: "Fee",
  feeText: "345 ETH",
  TooltipProps: {
    title: "Fee display tooltip",
  },
  textFirst: true,
};

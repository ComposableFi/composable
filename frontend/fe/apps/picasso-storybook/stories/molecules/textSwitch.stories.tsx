import { Box } from "@mui/material";
import { Story } from "@storybook/react";

import { TextSwitch, TextSwitchProps } from "picasso/components/Molecules";

const TextSwitchStories = (props: TextSwitchProps) => {
  return (
    <Box>
      <TextSwitch {...props} />
    </Box>
  );
};

export default {
  title: "molecules/TextAndSwitches",
  component: TextSwitchStories,
  textFirst: {
    options: [true, false],
  },
  checked: {
    options: [true, false],
  },
};

const defaultArgs = {
  label: "Text element",
  checked: true,
  onChange: () => {},
};

const Template: Story<typeof TextSwitchStories> = (args) => (
  <TextSwitchStories {...defaultArgs} {...args} />
);

export const TextSwitchesNoTooltip = Template.bind({});
TextSwitchesNoTooltip.args = {
  textFirst: true,
};

export const TextSwitchesWithTooltip = Template.bind({});
TextSwitchesWithTooltip.args = {
  textFirst: true,
  TooltipProps: {
    title: "Tooltip text",
  },
};

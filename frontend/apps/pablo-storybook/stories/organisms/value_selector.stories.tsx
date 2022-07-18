import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { ValueSelectorProps, ValueSelector } from "pablo/components";

const ValueSelectorStories = (props: ValueSelectorProps) => {
  return (
    <Box width={500}>
      <ValueSelector {...props} />
    </Box>
  );
};
export default {
  title: "organisms/ValueSelector",
  component: ValueSelector,
};

const Template: ComponentStory<typeof ValueSelector> = (args) => (
  <ValueSelectorStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  values: [25, 50, 75, 100],
  unit: "%",
  onChangeHandler: () => {},
};

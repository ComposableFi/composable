import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { YourBondTable } from "@ui-pablo/app/components";

const YourBondTableStories = () => {
  return (
    <Box>
      <YourBondTable />
    </Box>
  );
};
export default {
  title: "organisms/YourBondTable",
  component: YourBondTable,
};

const Template: ComponentStory<typeof YourBondTable> = (args) => (
  <YourBondTableStories />
);

export const Default = Template.bind({});
Default.args = {};

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AllBondTable } from "pablo/components";

const AllBondTableStories = () => {
  return (
    <Box>
      <AllBondTable />
    </Box>
  );
};
export default {
  title: "organisms/AllBondTable",
  component: AllBondTable,
};

const Template: ComponentStory<typeof AllBondTable> = (args) => (
  <AllBondTableStories />
);

export const Default = Template.bind({});
Default.args = {};

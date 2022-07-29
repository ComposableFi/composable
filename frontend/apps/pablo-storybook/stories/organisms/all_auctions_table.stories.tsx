import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AllAuctionsTable } from "pablo/components/Organisms/AllAuctionsTable";

const AllAuctionsTableStories = ({}) => {
  return (
    <Box>
      <AllAuctionsTable />
    </Box>
  );
};
export default {
  title: "organisms/AllAuctionsTable",
  component: AllAuctionsTable,
};

const Template: ComponentStory<typeof AllAuctionsTable> = (args) => (
  <AllAuctionsTableStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

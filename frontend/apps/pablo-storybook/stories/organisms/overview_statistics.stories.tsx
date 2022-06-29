import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { Statistics } from "pablo/components/Organisms/overview/Statistics";

const StatisticsStories = ({}) => {
  return (
    <Box>
      <Statistics />
    </Box>
  );
};
export default {
  title: "organisms/overview/Statistics",
  component: Statistics,
};

const Template: ComponentStory<typeof Statistics> = (args) => (
  <StatisticsStories />
);

export const Default = Template.bind({});
Default.args = {};

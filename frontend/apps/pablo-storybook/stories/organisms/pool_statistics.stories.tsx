import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolStatistics } from "pablo/components/Organisms";

const PoolStatisticsStories = ({}) => {
  return (
    <Box>
      <PoolStatistics poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolStatistics",
  component: PoolStatistics,
};

const Template: ComponentStory<typeof PoolStatistics> = (args) => (
  <PoolStatisticsStories />
);

export const Default = Template.bind({});
Default.args = {};

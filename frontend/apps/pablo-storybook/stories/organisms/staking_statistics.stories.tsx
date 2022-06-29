import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { StakingStatistics } from "pablo/components/Organisms/staking/Statistics";

const StakingStatisticsStories = ({}) => {
  return (
    <Box>
      <StakingStatistics />
    </Box>
  );
};
export default {
  title: "organisms/staking/StakingStatistics",
  component: StakingStatistics,
};

const Template: ComponentStory<typeof StakingStatistics> = (args) => (
  <StakingStatisticsStories />
);

export const Default = Template.bind({});
Default.args = {};

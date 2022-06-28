import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolRewardsPanel } from "pablo/components/Organisms";

const PoolRewardsPanelStories = ({}) => {
  return (
    <Box>
      <PoolRewardsPanel poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolRewardsPanel",
  component: PoolRewardsPanel,
};

const Template: ComponentStory<typeof PoolRewardsPanel> = (args) => (
  <PoolRewardsPanelStories />
);

export const Default = Template.bind({});
Default.args = {};

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolRewardsPanel } from "@ui-pablo/nextjs/components/Organisms";

const PoolRewardsPanelStories = ({}) => {
  return (
    <Box>
      <PoolRewardsPanel />
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

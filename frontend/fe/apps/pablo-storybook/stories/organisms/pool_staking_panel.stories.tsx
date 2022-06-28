import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolStakingPanel } from "pablo/components/Organisms";

const PoolStakingPanelStories = ({}) => {
  return (
    <Box>
      <PoolStakingPanel poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolStakingPanel",
  component: PoolStakingPanel,
};

const Template: ComponentStory<typeof PoolStakingPanel> = (args) => (
  <PoolStakingPanelStories />
);

export const Default = Template.bind({});
Default.args = {};

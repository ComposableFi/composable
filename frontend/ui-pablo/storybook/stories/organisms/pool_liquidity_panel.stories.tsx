import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolLiquidityPanel } from "@ui-pablo/nextjs/components/Organisms";

const PoolLiquidityPanelStories = ({}) => {
  return (
    <Box>
      <PoolLiquidityPanel />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolLiquidityPanel",
  component: PoolLiquidityPanel,
};

const Template: ComponentStory<typeof PoolLiquidityPanel> = (args) => (
  <PoolLiquidityPanelStories />
);

export const Default = Template.bind({});
Default.args = {};

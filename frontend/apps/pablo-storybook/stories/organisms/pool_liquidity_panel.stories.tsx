import { ComponentStory } from "@storybook/react";
import { PoolLiquidityPanel } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolLiquidityPanel",
  component: PoolLiquidityPanel,
};

const Template: ComponentStory<typeof PoolLiquidityPanel> = (args) => (
  <PoolLiquidityPanel {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

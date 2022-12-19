import { ComponentStory } from "@storybook/react";
import { PoolStakingPanel } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolStakingPanel",
  component: PoolStakingPanel,
};

const Template: ComponentStory<typeof PoolStakingPanel> = (args) => (
  <PoolStakingPanel {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

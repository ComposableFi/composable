import { ComponentStory } from "@storybook/react";
import { PoolRewardsPanel } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolRewardsPanel",
  component: PoolRewardsPanel,
};

const Template: ComponentStory<typeof PoolRewardsPanel> = (args) => (
  <PoolRewardsPanel {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0 ;",
};

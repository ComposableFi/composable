import { ComponentStory } from "@storybook/react";
import { PoolStakeForm } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolStakeForm",
  component: PoolStakeForm,
};

const Template: ComponentStory<typeof PoolStakeForm> = (args) => (
  <PoolStakeForm {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

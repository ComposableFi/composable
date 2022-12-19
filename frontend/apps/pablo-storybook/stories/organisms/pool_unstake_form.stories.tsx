import { ComponentStory } from "@storybook/react";
import { PoolUnstakeForm } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolUnstakeForm",
  component: PoolUnstakeForm,
};

const Template: ComponentStory<typeof PoolUnstakeForm> = (args) => (
  <PoolUnstakeForm {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

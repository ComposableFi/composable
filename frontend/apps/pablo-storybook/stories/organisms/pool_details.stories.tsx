import { ComponentStory } from "@storybook/react";
import { PoolDetails } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/Main",
  component: PoolDetails,
};

const Template: ComponentStory<typeof PoolDetails> = (args) => (
  <PoolDetails {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

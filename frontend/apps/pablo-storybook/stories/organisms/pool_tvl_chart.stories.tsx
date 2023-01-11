import { ComponentStory } from "@storybook/react";
import { PoolTVLChart } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolTVLChart",
  component: PoolTVLChart,
};

const Template: ComponentStory<typeof PoolTVLChart> = (args) => (
  <PoolTVLChart {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

import { ComponentStory } from "@storybook/react";
import { PoolStatistics } from "pablo/components/Organisms";

export default {
  title: "organisms/PoolDetails/PoolStatistics",
  component: PoolStatistics,
};

const Template: ComponentStory<typeof PoolStatistics> = (args) => (
  <PoolStatistics {...args} />
);

export const Default = Template.bind({});
Default.args = {
  poolId: "0",
};

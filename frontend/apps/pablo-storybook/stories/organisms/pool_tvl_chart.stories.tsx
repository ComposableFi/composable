import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolTVLChart } from "pablo/components/Organisms";

const PoolTVLChartStories = ({}) => {
  return (
    <Box>
      <PoolTVLChart poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolTVLChart",
  component: PoolTVLChart,
};

const Template: ComponentStory<typeof PoolTVLChart> = (args) => (
  <PoolTVLChartStories />
);

export const Default = Template.bind({});
Default.args = {};

import { ComponentStory, Story } from "@storybook/react";
import { AreaChart, AreaChartProps } from "pablo/components";

export default {
  title: "atoms/AreaChart",
  component: AreaChart,
};

const defaultArgs: AreaChartProps = {
  data: [
    [1644550600000, 20],
    [1644560620928, 50],
    [1644570600000, 30],
    [1644580600000, 100],
    [1644590600000, 80],
  ],
  height: 200,
  shorthandLabel: "Change",
  labelFormat: (n: number) => n.toFixed(),
  color: "#FF8500",
};

const Template: ComponentStory<typeof AreaChart> = (args) => (
  <AreaChart {...args} />
);

export const DefaultAreaChart = Template.bind({});

DefaultAreaChart.args = defaultArgs;

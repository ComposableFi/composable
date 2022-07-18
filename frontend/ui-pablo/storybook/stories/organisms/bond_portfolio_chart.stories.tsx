import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { BondPortfolioChart } from "@ui-pablo/app/components/Organisms/bonds/BondPortfolioChart";

const BondPortfolioChartStories = ({}) => {
  return (
    <Box>
      <BondPortfolioChart />
    </Box>
  );
};
export default {
  title: "organisms/Bonds/PortfolioChart",
  component: BondPortfolioChart,
};

const Template: ComponentStory<typeof BondPortfolioChart> = (args) => (
  <BondPortfolioChartStories />
);

export const Default = Template.bind({});
Default.args = {};

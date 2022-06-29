import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AllLiquidityTableProps, AllLiquidityTable } from "pablo/components/Organisms/AllLiquidityTable";

const AllLiquidityTableStories = (props: AllLiquidityTableProps) => {
  return (
    <Box>
      <AllLiquidityTable {...props} />
    </Box>
  );
};
export default {
  title: "organisms/AllLiquidityTable",
  component: AllLiquidityTable,
};

const Template: ComponentStory<typeof AllLiquidityTable> = (args) => (
  <AllLiquidityTableStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  flow: "user",
};

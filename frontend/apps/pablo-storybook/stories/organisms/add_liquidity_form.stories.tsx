import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AddLiquidityForm } from "pablo/components/Organisms/liquidity/AddForm";

const AddLiquidityFormStories = ({}) => {
  return (
    <Box>
      <AddLiquidityForm />
    </Box>
  );
};
export default {
  title: "organisms/AddLiquidityForm",
  component: AddLiquidityForm,
};

const Template: ComponentStory<typeof AddLiquidityForm> = (args) => (
  <AddLiquidityFormStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

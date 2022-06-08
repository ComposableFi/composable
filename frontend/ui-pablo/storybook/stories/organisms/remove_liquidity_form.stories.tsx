import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { RemoveLiquidityForm } from "@ui-pablo/nextjs/components/Organisms/liquidity/RemoveForm";

const RemoveLiquidityFormStories = () => {
  return (
    <Box>
      <RemoveLiquidityForm />
    </Box>
  );
};
export default {
  title: "organisms/RemoveLiquidityForm",
  component: RemoveLiquidityForm,
};

const Template: ComponentStory<typeof RemoveLiquidityForm> = (args) => (
  <RemoveLiquidityFormStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

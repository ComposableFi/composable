import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { LiquidityProvidersBox } from "pablo/components/Organisms/overview/LiquidityProvidersBox";

const LiquidityProvidersBoxStories = ({}) => {
  return (
    <Box>
      <LiquidityProvidersBox />
    </Box>
  );
};
export default {
  title: "organisms/overview/LiquidityProvidersBox",
  component: LiquidityProvidersBox,
};

const Template: ComponentStory<typeof LiquidityProvidersBox> = (args) => (
  <LiquidityProvidersBoxStories />
);

export const Default = Template.bind({});
Default.args = {};

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { Staking } from "pablo/components/Organisms/staking";

const StakingStories = ({}) => {
  return (
    <Box>
      <Staking />
    </Box>
  );
};
export default {
  title: "organisms/staking/Main",
  component: Staking,
};

const Template: ComponentStory<typeof Staking> = (args) => (
  <StakingStories />
);

export const Default = Template.bind({});
Default.args = {};

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { ClaimableRewards } from "pablo/components/Organisms/staking/ClaimableRewards";

const ClaimableRewardsStories = ({}) => {
  return (
    <Box>
      <ClaimableRewards />
    </Box>
  );
};
export default {
  title: "organisms/staking/ClaimableRewards",
  component: ClaimableRewards,
};

const Template: ComponentStory<typeof ClaimableRewards> = (args) => (
  <ClaimableRewardsStories />
);

export const Default = Template.bind({});
Default.args = {};

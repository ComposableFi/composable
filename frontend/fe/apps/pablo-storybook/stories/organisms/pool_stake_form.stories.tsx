import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolStakeForm } from "pablo/components/Organisms";

const PoolStakeFormStories = ({}) => {
  return (
    <Box>
      <PoolStakeForm poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolStakeForm",
  component: PoolStakeForm,
};

const Template: ComponentStory<typeof PoolStakeForm> = (args) => (
  <PoolStakeFormStories />
);

export const Default = Template.bind({});
Default.args = {};

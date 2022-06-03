import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolStakeForm } from "@ui-pablo/nextjs/components/Organisms";

const PoolStakeFormStories = ({}) => {
  return (
    <Box>
      <PoolStakeForm />
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

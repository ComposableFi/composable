import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolUnstakeForm } from "pablo/components/Organisms";

const PoolUnstakeFormStories = ({}) => {
  return (
    <Box>
      <PoolUnstakeForm poolId={-1} />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/PoolUnstakeForm",
  component: PoolUnstakeForm,
};

const Template: ComponentStory<typeof PoolUnstakeForm> = (args) => (
  <PoolUnstakeFormStories />
);

export const Default = Template.bind({});
Default.args = {};

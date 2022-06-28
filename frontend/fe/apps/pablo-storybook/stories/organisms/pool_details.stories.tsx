import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { PoolDetails } from "pablo/components/Organisms";

const PoolDetailsStories = ({}) => {
  return (
    <Box>
      <PoolDetails />
    </Box>
  );
};
export default {
  title: "organisms/PoolDetails/Main",
  component: PoolDetails,
};

const Template: ComponentStory<typeof PoolDetails> = (args) => (
  <PoolDetailsStories />
);

export const Default = Template.bind({});
Default.args = {};

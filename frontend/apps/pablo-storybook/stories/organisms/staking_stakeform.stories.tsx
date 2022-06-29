import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { StakeForm } from "pablo/components/Organisms/staking/StakeForm";

const StakeFormStories = ({}) => {
  return (
    <Box>
      <StakeForm />
    </Box>
  );
};
export default {
  title: "organisms/staking/StakeForm",
  component: StakeForm,
};

const Template: ComponentStory<typeof StakeForm> = (args) => (
  <StakeFormStories />
);

export const Default = Template.bind({});
Default.args = {};

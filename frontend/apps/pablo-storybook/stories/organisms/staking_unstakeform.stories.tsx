import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { UnstakeForm } from "pablo/components/Organisms/staking/UnstakeForm";

const UnstakeFormStories = ({}) => {
  return (
    <Box>
      <UnstakeForm />
    </Box>
  );
};
export default {
  title: "organisms/staking/UnstakeForm",
  component: UnstakeForm,
};

const Template: ComponentStory<typeof UnstakeForm> = (args) => (
  <UnstakeFormStories />
);

export const Default = Template.bind({});
Default.args = {};

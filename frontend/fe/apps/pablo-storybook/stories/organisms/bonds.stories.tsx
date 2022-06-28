import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { Bonds } from "pablo/components/Organisms/bonds";

const BondsStories = ({}) => {
  return (
    <Box>
      <Bonds />
    </Box>
  );
};
export default {
  title: "organisms/Bonds/Main",
  component: Bonds,
};

const Template: ComponentStory<typeof Bonds> = (args) => (
  <BondsStories />
);

export const Default = Template.bind({});
Default.args = {};


import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { NavBar } from "pablo/components/Organisms/NavBar";

const NavBarStories = () => {
  return (
    <Box width={300}>
      <NavBar />
    </Box>
  );
};
export default {
  title: "organisms/NavBar",
  component: NavBar,
};

const Template: ComponentStory<typeof NavBar> = (args) => (
  <NavBarStories />
);

export const Default = Template.bind({});
Default.args = {};

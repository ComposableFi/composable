import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { XPablosBox } from "@ui-pablo/app/components/Organisms";

const XPablosBoxStories = ({}) => {
  return (
    <Box>
      <XPablosBox />
    </Box>
  );
};
export default {
  title: "organisms/overview/XPablosBox",
  component: XPablosBox,
};

const Template: ComponentStory<typeof XPablosBox> = (args) => (
  <XPablosBoxStories />
);

export const Default = Template.bind({});
Default.args = {};

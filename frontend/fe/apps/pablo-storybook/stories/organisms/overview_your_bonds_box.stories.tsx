import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { YourBondsBox } from "pablo/components/Organisms/overview/YourBondsBox";

const YourBondsBoxStories = ({}) => {
  return (
    <Box>
      <YourBondsBox />
    </Box>
  );
};
export default {
  title: "organisms/overview/YourBondsBox",
  component: YourBondsBox,
};

const Template: ComponentStory<typeof YourBondsBox> = (args) => (
  <YourBondsBoxStories />
);

export const Default = Template.bind({});
Default.args = {};

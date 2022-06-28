import { Box, BoxProps } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { CreatePool } from "pablo/components/Organisms/pool/CreatePool";

const CreatePoolStories = (props: BoxProps) => {
  return (
    <Box>
      <CreatePool {...props} />
    </Box>
  );
};
export default {
  title: "organisms/CreatePool",
  component: CreatePool,
};

const Template: ComponentStory<typeof CreatePool> = (args) => (
  <CreatePoolStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

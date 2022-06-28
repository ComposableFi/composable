import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { UnverifiedPoolWarningModal } from "pablo/components/Organisms/pool/CreatePool/UnverifiedPoolWarningModal";

const UnverifiedPoolWarningModalStories = () => {
  return (
    <Box>
      <UnverifiedPoolWarningModal open={true} />
    </Box>
  );
};
export default {
  title: "organisms/UnverifiedPoolWarningModal",
  component: UnverifiedPoolWarningModal,
};

const Template: ComponentStory<typeof UnverifiedPoolWarningModal> = (args) => (
  <UnverifiedPoolWarningModalStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

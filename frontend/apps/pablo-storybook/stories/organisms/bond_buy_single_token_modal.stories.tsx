import { BuySingleTokenModal } from "pablo/components/Organisms";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";

const BuySingleTokenModalStories = () => {
  return (
    <Box>
      <BuySingleTokenModal tokenId="ksm" open={true} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/BuySingleTokenModal",
  component: BuySingleTokenModal,
};

const Template: ComponentStory<typeof BuySingleTokenModal> = (args) => (
  <BuySingleTokenModalStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

import { BuySingleTokenModal } from "@/components/Organisms";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { useAppSelector } from "@/hooks/store";

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

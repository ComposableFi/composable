import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { BuyForm } from "@ui-pablo/app/components/Organisms/auction/BuyForm";
import useStore from "@ui-pablo/app/store/useStore";

const BuyFormStories = () => {
  const {auctions} = useStore();
  return (
    <Box>
      <BuyForm auction={auctions.activeLBP} />
    </Box>
  );
};
export default {
  title: "organisms/Auction/BuyForm",
  component: BuyForm,
};

const Template: ComponentStory<typeof BuyForm> = (args) => (
  <BuyFormStories />
);

export const Default = Template.bind({});
Default.args = {};

import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { BuyForm } from "pablo/components/Organisms/auction/BuyForm";

const BuyFormStories = () => {
  const { activePool } = useAuctionsSlice();
  return (
    <Box>
      {activePool && <BuyForm auction={activePool} />}
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

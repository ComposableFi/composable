import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionDetails } from "pablo/components/Organisms/auction/AuctionDetails";
import useStore from "pablo/store/useStore";

const AuctionDetailsStories = () => {
  const { activePool, activePoolStats } = useAuctionsSlice();
  return (
    <Box>
      <AuctionDetails stats={activePoolStats} auction={activePool} />
    </Box>
  );
};
export default {
  title: "organisms/Auction/AuctionDetails",
  component: AuctionDetails,
};

const Template: ComponentStory<typeof AuctionDetails> = (args) => (
  <AuctionDetailsStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

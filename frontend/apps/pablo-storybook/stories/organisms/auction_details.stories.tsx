import { useAsset } from "@/defi/hooks";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionDetails } from "pablo/components/Organisms/auction/AuctionDetails";
import useStore from "pablo/store/useStore";

const AuctionDetailsStories = () => {
  const { activePool, activePoolStats } = useAuctionsSlice();
  const baseAsset = useAsset(activePool?.getPair().getBaseAsset().toString() as string ?? "-");
  const quoteAsset = useAsset(activePool?.getPair().getQuoteAsset().toString() as string ?? "-");
  const hasLoaded = baseAsset && quoteAsset && activePool

  return (
    <Box>
      {hasLoaded && <AuctionDetails 
        quoteAsset={quoteAsset}
        baseAsset={baseAsset}
        stats={activePoolStats}
        auction={activePool} />}
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

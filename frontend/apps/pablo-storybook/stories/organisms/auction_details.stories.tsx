import { useAsset } from "@/defi/hooks";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionDetails } from "pablo/components/Organisms/auction/AuctionDetails";
import useStore from "pablo/store/useStore";

const AuctionDetailsStories = () => {
  const { activePool, activePoolStats } = useAuctionsSlice();
  const pair = activePool ? Object.keys(activePool.getAssets().assets) : null;
  const baseAsset = useAsset(pair?.[0] ?? "-");
  const quoteAsset = useAsset(pair?.[1] ?? "-");
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

import { useAsset } from "@/defi/hooks";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionInformation } from "pablo/components/Organisms/auction/AuctionInformation";

const AuctionInformationStories = () => {
  const { activePool, activePoolStats }: any = useAuctionsSlice();
  const pair = activePool ? Object.keys(activePool.getAssets().assets) : null;
  const baseAsset = useAsset(pair?.[0] ?? "-");
  const quoteAsset = useAsset(pair?.[1] ?? "-");
  const hasLoaded = baseAsset && quoteAsset && activePool

  return (
    <Box>
      {hasLoaded && <AuctionInformation 
      quoteAsset={quoteAsset}
      baseAsset={baseAsset}
      stats={activePoolStats} auction={activePool} />}
    </Box>
  );
};
export default {
  title: "organisms/Auction/AuctionInformation",
  component: AuctionInformation,
};

const Template: ComponentStory<typeof AuctionInformation> = (args) => (
  <AuctionInformationStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

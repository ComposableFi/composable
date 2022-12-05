import { useAsset } from "@/defi/hooks";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionHistoriesTable } from "pablo/components/Organisms/auction/AuctionHistoriesTable";


const AuctionHistoriesTableStories = () => {
  const { activePool, activePoolTradeHistory } = useAuctionsSlice();
  const pair = activePool ? Object.keys(activePool.getAssets().assets) : null;
  const baseAsset = useAsset(pair?.[0] ?? "-");
  const quoteAsset = useAsset(pair?.[1] ?? "-");
  const hasLoaded = baseAsset && quoteAsset && activePool

  return (
    <Box>
      {hasLoaded && <AuctionHistoriesTable 
        quoteAsset={quoteAsset}
        baseAsset={baseAsset}
        history={activePoolTradeHistory}
        auction={activePool} />}
    </Box>
  );
};
export default {
  title: "organisms/Auction/AuctionHistoriesTable",
  component: AuctionHistoriesTable,
};

const Template: ComponentStory<typeof AuctionHistoriesTable> = (args) => (
  <AuctionHistoriesTableStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

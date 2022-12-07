import { Box, BoxProps } from "@mui/material";
import { TabItem, Tabs, TabPanel } from "@/components";
import { AuctionDetails } from "@/components/Organisms/auction/AuctionDetails";
import { AuctionHistoriesTable } from "@/components/Organisms/auction/AuctionHistoriesTable";
import { useState } from "react";
import { useAsset } from "@/defi/hooks/assets/useAsset";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";

const AuctionDetailTabs: React.FC<BoxProps> = ({ ...props }) => {
  const { activePool, activePoolStats, activePoolTradeHistory } =
    useAuctionsSlice();

  const baseAsset = useAsset(
    activePool ? activePool.getPair().getBaseAsset().toString() : "-"
  );
  const quoteAsset = useAsset(
    activePool ? activePool.getPair().getQuoteAsset().toString() : "-"
  );

  const hasLoaded = quoteAsset && baseAsset && activePool;

  const tabItems: TabItem[] = [
    {
      label: "Auction Details",
    },
    {
      label: "Auction History",
    },
  ];

  const [tab, setTab] = useState(0);
  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <Box mt={8} {...props}>
      <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
      <TabPanel value={tab} index={0}>
        {hasLoaded && <AuctionDetails
          stats={activePoolStats}
          auction={activePool}
          baseAsset={baseAsset}
          quoteAsset={quoteAsset}
        />}
      </TabPanel>
      <TabPanel value={tab} index={1}>
        {hasLoaded && <AuctionHistoriesTable
          history={activePoolTradeHistory}
          auction={activePool}
          baseAsset={baseAsset}
          quoteAsset={quoteAsset}
        />}
      </TabPanel>
    </Box>
  );
};

export default AuctionDetailTabs;

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import {
  AuctionPriceChart,
  AuctionPriceChartProps,
} from "pablo/components/Organisms/auction/AuctionPriceChart";

import moment from "moment-timezone";
import { Asset } from "shared";
import { TOKENS } from "tokens";

const dummyAuctionPrices = [
  [1644454200000, 7],
  [1644550600000, 6.5],
  [1644657000000, 4.1],
  [1644743400000, 3.5],
  [1644829800000, 2],
] as [number, number][];

const AuctionPriceChartStories = (props: AuctionPriceChartProps) => {
  return (
    <Box width={500} height={500}>
      <AuctionPriceChart {...props} />
    </Box>
  );
};
export default {
  title: "organisms/Auction/AuctionPriceChart",
  component: AuctionPriceChart,
};

const Template: ComponentStory<typeof AuctionPriceChart> = (args) => (
  <AuctionPriceChartStories {...args} />
);

export const Default = Template.bind({});
Default.args = {
  baseAsset: new Asset(
    TOKENS.pblo.symbol,
    TOKENS.pblo.symbol,
    TOKENS.pblo.icon,
    TOKENS.pblo.id
  ),
  chartSeries: {
    currentPriceSeries: dummyAuctionPrices,
    predictedPriceSeries: [],
  },
  height: "100%",
  dateFormat: (timestamp: number | string) => {
    return moment(timestamp).utc().format("MMM D, h:mm:ss A");
  },
};

import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionInformation } from "pablo/components/Organisms/auction/AuctionInformation";

const AuctionInformationStories = () => {
  const { activePool, activePoolStats }: any = useAuctionsSlice();
  return (
    <Box>
      <AuctionInformation stats={activePoolStats} auction={activePool} />
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

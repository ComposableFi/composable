import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionInformation } from "pablo/components/Organisms/auction/AuctionInformation";
import useStore from "pablo/store/useStore";

const AuctionInformationStories = () => {
  const {auctions}: any = useStore();
  return (
    <Box>
      <AuctionInformation stats={auctions.activeLBPStats} auction={auctions.activeLBP} />
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

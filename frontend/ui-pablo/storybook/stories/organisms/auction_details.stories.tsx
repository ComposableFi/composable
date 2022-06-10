import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionDetails } from "@ui-pablo/app/components/Organisms/auction/AuctionDetails";
import useStore from "@ui-pablo/app/store/useStore";

const AuctionDetailsStories = () => {
  const {auctions} = useStore();
  return (
    <Box>
      <AuctionDetails stats={auctions.activeLBPStats} auction={auctions.activeLBP} />
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

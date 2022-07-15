import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionInformation } from "@ui-pablo/app/components/Organisms/auction/AuctionInformation";
import useStore from "@ui-pablo/app/store/useStore";

const AuctionInformationStories = () => {
  const {auctions} = useStore();
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

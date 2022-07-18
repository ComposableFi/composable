import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionHistoriesTable } from "@ui-pablo/app/components/Organisms/auction/AuctionHistoriesTable";
import useStore from "@ui-pablo/app/store/useStore";

const AuctionHistoriesTableStories = () => {
  const {auctions} = useStore();
  return (
    <Box>
      <AuctionHistoriesTable auction={auctions.activeLBP} />
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

import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import { AuctionHistoriesTable } from "../../../../apps/pablo/components/Organisms/auction/AuctionHistoriesTable";
import useStore from "../../../../apps/pablo/store/useStore";

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

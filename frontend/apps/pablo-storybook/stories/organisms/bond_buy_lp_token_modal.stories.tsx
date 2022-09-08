import { BuyLPTokenModal } from "pablo/components/Organisms";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import BigNumber from "bignumber.js";

const BuyLPTokenModalStories = () => {
  const bond = {
    tokenId1: "ksm",
    tokenId2: "pica",
    roi: 26,
    vesting_term: 7,
    tvl: new BigNumber(1500000),
    volume: new BigNumber(132500000),
    discount_price: new BigNumber(2.3),
    market_price: new BigNumber(2.9),
    balance: new BigNumber(435),
    rewardable_amount: new BigNumber(0),
    buyable_amount: new BigNumber(500),
    pending_amount: new BigNumber(0),
    claimable_amount: new BigNumber(0),
    remaining_term: 7,
    vested_term: 0,
  } as const;
  return (
    <Box>
      <BuyLPTokenModal bond={bond} open={true} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/BuyLPTokenModal",
  component: BuyLPTokenModal,
};

const Template: ComponentStory<typeof BuyLPTokenModal> = (args) => (
  <BuyLPTokenModalStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};

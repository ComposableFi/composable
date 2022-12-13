import { Box, BoxProps } from "@mui/material";
import { TabItem, TabPanel, Tabs } from "@/components";
import { StakingStatistics } from "./Statistics";
import { XPablosBox } from "../XPablosBox";
import { BoxWrapper } from "../BoxWrapper";
import { StakeForm } from "./StakeForm";
import { ClaimableRewards } from "./ClaimableRewards";
import { UnstakeForm } from "./UnstakeForm";
import { PBLO_ASSET_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";

const tabItems: TabItem[] = [
  {
    label: "Stake and mint",
  },
  {
    label: "Burn and unstake",
  },
];

const tableHeaders = [
  {
    header: "fNFT ID",
  },
  {
    header: "Locked PBLO",
  },
  {
    header: "Expiry",
  },
  {
    header: "Multiplier",
  },
  {
    header: "Your xPABLO",
  },
  {
    header: "", // kept empty for action column that has no header
  },
];

export const Staking: React.FC<BoxProps> = ({ ...boxProps }) => {
  const tab = 0;
  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {};

  const mockedRewardPool = {
    owner: "",
    rewards: {},
    claimedShares: new BigNumber(1),
    startBlock: new BigNumber(1),
    endBlock: new BigNumber(1),
    lock: {
      durationPresets: {},
      unlockPenalty: new BigNumber(0),
    },
    shareAssetId: "",
    financialNftAssetId: "",
    minimumStakingAmount: new BigNumber(1),
  };

  return (
    <Box {...boxProps}>
      <StakingStatistics
        stakingRewardPool={mockedRewardPool}
        rewardPoolId={PBLO_ASSET_ID}
      />
      <XPablosBox
        financialNftCollectionId={
          mockedRewardPool ? mockedRewardPool.financialNftAssetId : "-"
        }
        mt={8}
        title="Portfolio"
        header={tableHeaders}
      />

      <BoxWrapper mt={8}>
        <Tabs
          items={tabItems}
          value={tab}
          onChange={handleTabChange}
          disabled
        />
        <TabPanel index={0} value={tab}>
          <StakeForm stakingRewardPool={mockedRewardPool} />
        </TabPanel>
        <TabPanel index={1} value={tab}>
          <UnstakeForm stakingRewardPool={mockedRewardPool} />
        </TabPanel>
      </BoxWrapper>

      <ClaimableRewards
        financialNftCollectionId={mockedRewardPool?.financialNftAssetId.toString()}
        mt={8}
      />
    </Box>
  );
};

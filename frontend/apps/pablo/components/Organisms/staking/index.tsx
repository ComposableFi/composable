import { Box, useTheme, BoxProps } from "@mui/material";
import { TabItem, TabPanel, Tabs } from "@/components";
import { useState } from "react";
import { StakingStatistics } from "./Statistics";
import { XPablosBox } from "../XPablosBox";
import { BoxWrapper } from "../BoxWrapper";
import { StakeForm } from "./StakeForm";
import { ClaimableRewards } from "./ClaimableRewards";
import { UnstakeForm } from "./UnstakeForm";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { PBLO_ASSET_ID } from "@/defi/utils";
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
  const [tab, setTab] = useState(0);

  const stakingRewardPool = useStakingRewardPool(PBLO_ASSET_ID);
  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <Box {...boxProps}>
      <StakingStatistics stakingRewardPool={stakingRewardPool} rewardPoolId={PBLO_ASSET_ID} />
        <XPablosBox
          financialNftCollectionId={
            stakingRewardPool ? stakingRewardPool.financialNftAssetId : "-"
          }
          mt={8}
          title="Portfolio"
          header={tableHeaders}
        />

      <BoxWrapper mt={8}>
        <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
        <TabPanel index={0} value={tab}>
          <StakeForm stakingRewardPool={stakingRewardPool} />
        </TabPanel>
        <TabPanel index={1} value={tab}>
          <UnstakeForm stakingRewardPool={stakingRewardPool} />
        </TabPanel>
      </BoxWrapper>

        <ClaimableRewards
          financialNftCollectionId={stakingRewardPool?.financialNftAssetId.toString()}
          mt={8}
        />


      {/* {message.text && (
        <Box mt={8}>
          <Alert
            severity={message.severity}
            alertTitle={message.title}
            alertText={message.text}
            AlertTextProps={{ color: "text.secondary" }}
            onClose={() => dispatch(setMessage({}))}
            underlined
            action={
              message.link ? (
                <Link href={message.link} target="_blank" rel="noopener">
                  <OpenInNewRoundedIcon />
                </Link>
              ) : (
                undefined
              )
            }
          />
        </Box>
      )} */}
    </Box>
  );
};

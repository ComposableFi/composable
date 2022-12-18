import { Box, BoxProps, Grid } from "@mui/material";
import { TabItem, TabPanel, Tabs } from "@/components";
import { FC, SyntheticEvent, useState } from "react";
import { PoolTVLChart } from "./PoolTVLChart";
import { PoolStatistics } from "./PoolStatistics";
import { PoolLiquidityPanel } from "./PoolLiquidityPanel";
import { PoolStakingPanel } from "./PoolStakingPanel";
import { PoolRewardsPanel } from "./PoolRewardsPanel";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

const tabItems: TabItem[] = [
  {
    label: "Liquidity",
  },
  {
    label: "Staking",
    disabled: true,
  },
  {
    label: "Rewards",
    disabled: true,
  },
];

export type PoolDetailsProps = { poolId: string } & BoxProps;

export const PoolDetails: FC<PoolDetailsProps> = ({ poolId, ...boxProps }) => {
  const [tab, setTab] = useState<number>(0);
  const handleTabChange = (_: SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <PoolTVLChart poolId={Number(poolId)} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <PoolStatistics poolId={poolId} />
        </Grid>
      </Grid>

      <Box mt={6}>
        <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
        <TabPanel index={0} value={tab}>
          <PoolLiquidityPanel poolId={poolId} />
        </TabPanel>
        <TabPanel index={1} value={tab}>
          <PoolStakingPanel poolId={poolId} />
        </TabPanel>
        <TabPanel index={2} value={tab}>
          <PoolRewardsPanel poolId={poolId} />
        </TabPanel>
      </Box>
    </Box>
  );
};

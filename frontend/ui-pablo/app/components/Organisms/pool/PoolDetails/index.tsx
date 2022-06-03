import {
  Box,
  BoxProps,
  Grid,
} from "@mui/material";
import {
  TabItem,
  TabPanel,
  Tabs,
} from "@/components";
import { useState } from "react";
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
  },
  {
    label: "Rewards",
  },
];

export const PoolDetails: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const [tab, setTab] = useState<number>(0);
  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <PoolTVLChart />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <PoolStatistics />
        </Grid>
      </Grid>

      <Box mt={6}>
        <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
        <TabPanel index={0} value={tab}>
          <PoolLiquidityPanel />
        </TabPanel>
        <TabPanel index={1} value={tab}>
          <PoolStakingPanel />
        </TabPanel>
        <TabPanel index={2} value={tab}>
          <PoolRewardsPanel />
        </TabPanel>
      </Box>

    </Box>
  );
};

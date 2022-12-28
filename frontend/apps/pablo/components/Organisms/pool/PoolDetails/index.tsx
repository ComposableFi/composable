import {
  alpha,
  Box,
  BoxProps,
  Grid,
  Typography,
  useTheme,
} from "@mui/material";
import { TabItem, TabPanel, Tabs } from "@/components";
import { FC, SyntheticEvent, useState } from "react";
import { PoolStatistics } from "./PoolStatistics";
import { PoolLiquidityPanel } from "./PoolLiquidityPanel";
import { PoolConfig } from "@/store/pools/types";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

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

export type PoolDetailsProps = { pool: PoolConfig } & BoxProps;

export const PoolDetails: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  const [tab, setTab] = useState<number>(0);
  const handleTabChange = (_: SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };
  const theme = useTheme();

  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item xs={12} sm={6} md={8}>
          <HighlightBox
            variant="outlined"
            sx={{
              border: `1px solid ${alpha(theme.palette.common.white, 0.1)}`,
              background: alpha(theme.palette.common.white, 0.05),
            }}
          >
            <Typography variant="h5" textAlign="left" mb={4}>
              TVL
            </Typography>
            <Box
              sx={{
                height: theme.spacing(18),
                minHeight: theme.spacing(18),
                display: "flex",
                alignItems: "center",
                flexDirection: "column",
              }}
              gap={2}
            >
              <Typography variant="body1" textAlign="left">
                Chart will be available once enough data is gathered...
              </Typography>
            </Box>
          </HighlightBox>
        </Grid>
        <Grid item xs={12} sm={6} md={4}>
          <PoolStatistics pool={pool} />
        </Grid>
      </Grid>

      <Box mt={6}>
        <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
        <TabPanel index={0} value={tab}>
          <PoolLiquidityPanel pool={pool} />
        </TabPanel>
        <TabPanel index={1} value={tab}>
          {/*<PoolStakingPanel pool={pool} />*/}
        </TabPanel>
        <TabPanel index={2} value={tab}>
          {/*<PoolRewardsPanel pool={pool} />*/}
        </TabPanel>
      </Box>
    </Box>
  );
};

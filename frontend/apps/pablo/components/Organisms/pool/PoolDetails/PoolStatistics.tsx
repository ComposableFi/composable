import {
  alpha,
  Box,
  BoxProps,
  Grid,
  Typography,
  useTheme,
} from "@mui/material";
import { PoolDetailsProps } from "./index";
import { FC, useEffect, useState } from "react";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import { PoolConfig } from "@/store/pools/types";
import { fetchPabloDaily, PabloDaily } from "@/defi/subsquid/pabloPool";
import BigNumber from "bignumber.js";
import { fromChainUnits } from "@/defi/utils";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  label: string;
  value?: string;
} & BoxProps;

const Item: FC<ItemProps> = ({ label, value, children, ...boxProps }) => {
  const theme = useTheme();
  return (
    <Box
      py={3.5}
      borderRadius={1}
      textAlign="center"
      border={`1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`}
      sx={{
        background: theme.palette.gradient.secondary,
      }}
      {...boxProps}
    >
      <Typography variant="body1" color="text.secondary">
        {label}
      </Typography>
      {value && (
        <Typography variant="h6" mt={0.5}>
          {value}
        </Typography>
      )}
      {children && children}
    </Box>
  );
};

const usePabloDaily = (pool: PoolConfig) => {
  const [pabloDaily, setPabloDaily] = useState<PabloDaily>({
    fees: "0",
    transactions: "0",
    volume: "0",
  });
  useEffect(() => {
    fetchPabloDaily(pool.poolId.toNumber()).then((pabloDaily) => {
      setPabloDaily({
        fees: fromChainUnits(pabloDaily.fees, 12).toFormat(4),
        transactions: new BigNumber(pabloDaily.transactions).toFormat(0),
        volume: fromChainUnits(pabloDaily.volume, 12).toFormat(4),
      });
    });
  }, [pool]);

  return pabloDaily;
};
export const PoolStatistics: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  const { poolTVL } = usePoolRatio(pool);
  const pabloDaily = usePabloDaily(pool);
  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <Item label="Pool value" value={`$${poolTVL.toFormat(2)}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Volume (24H)" value={pabloDaily.volume} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Fees (24H)" value={pabloDaily.fees} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Transactions (24H)" value={pabloDaily.transactions} />
        </Grid>
      </Grid>
    </Box>
  );
};

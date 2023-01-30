import {
  alpha,
  Box,
  BoxProps,
  Grid,
  Skeleton,
  Typography,
  useTheme,
} from "@mui/material";
import { PoolDetailsProps } from "./index";
import { FC, ReactElement, useEffect, useState } from "react";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import useStore from "@/store/useStore";
import { fetchPabloDailyForPool } from "@/defi/subsquid/pools/queries";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";
import { flow, pipe } from "fp-ts/function";
import * as TE from "fp-ts/TaskEither";
import * as E from "fp-ts/Either";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";
import BigNumber from "bignumber.js";
import { humanBalance } from "shared";

const usePoolDailyStats = (poolId: string) => {
  const hasFetchedTokens = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const hasFetchedPools = useStore((store) => store.pools.isLoaded);
  const [stats, setStats] = useState<{
    fees: string;
    transactions: string;
    volume: string;
  }>({
    fees: "0",
    transactions: "0",
    volume: "0",
  });

  const [isLoading, setIsLoading] = useState(false);
  const picaPrice = usePicaPriceDiscovery();
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);

  useEffect(() => {
    if (hasFetchedPools && hasFetchedTokens) {
      const task = pipe(
        TE.fromIO(() => setIsLoading(true)),
        TE.chain(fetchPabloDailyForPool(poolId)),
        TE.chainFirst(() => TE.fromIO(() => setIsLoading(false)))
      );

      task().then(
        flow(
          E.matchW(
            (e) => null,
            (a) => {
              const volume = a.pabloDaily.volume
                .reduce(
                  parseLockedValue(getTokenById, picaPrice),
                  new BigNumber(0)
                )
                .toFormat(2);

              const fees = a.pabloDaily.fees
                .reduce(
                  parseLockedValue(getTokenById, picaPrice),
                  new BigNumber(0)
                )
                .toFormat(2);

              const transactions = humanBalance(a.pabloDaily.transactions);

              setStats({
                fees,
                transactions,
                volume,
              });
            }
          )
        )
      );
    }
  }, [getTokenById, hasFetchedPools, hasFetchedTokens, picaPrice, poolId]);

  return {
    isLoading,
    stats,
  } as const;
};
const twoColumnPageSize = {
  xs: 12,
  sm: 6,
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

const LoadingBox: FC<{
  isLoading: boolean;
  children: ReactElement<any, any>;
}> = ({ isLoading, children }) => {
  return isLoading ? (
    <Skeleton variant="rounded" width="100%" height="127px" />
  ) : (
    children
  );
};

export const PoolStatistics: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  const { poolTVL } = usePoolRatio(pool.poolId.toString());
  const { stats, isLoading } = usePoolDailyStats(pool.poolId.toString());
  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <Item label="Pool value" value={`$${poolTVL.toFormat(2)}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <LoadingBox isLoading={isLoading}>
            <Item label="Volume (24H)" value={`$${stats.volume}`} />
          </LoadingBox>
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <LoadingBox isLoading={isLoading}>
            <Item label="Fees (24H)" value={`$${stats.fees}`} />
          </LoadingBox>
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <LoadingBox isLoading={isLoading}>
            <Item label="Transactions (24H)" value={`${stats.transactions}`} />
          </LoadingBox>
        </Grid>
      </Grid>
    </Box>
  );
};

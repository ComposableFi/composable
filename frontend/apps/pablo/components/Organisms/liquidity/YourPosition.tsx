import { Label, PairAsset } from "@/components/Atoms";
import { Asset } from "shared";
import {
  alpha,
  Box,
  BoxProps,
  CircularProgress,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { FC, useEffect, useState } from "react";
import { PoolConfig } from "@/store/pools/types";
import useStore from "@/store/useStore";
import { getStats, GetStatsReturn } from "@/defi/utils";

type YourPositionProps = {
  pool: PoolConfig;
  noTitle?: boolean;
  noDivider?: boolean;
  assets: Asset[];
  amounts: [BigNumber, BigNumber];
  expectedLP: BigNumber;
} & BoxProps;

export const YourPosition: FC<YourPositionProps> = ({
  noTitle,
  noDivider,
  assets,
  expectedLP,
  amounts,
  pool,
  ...rest
}) => {
  const theme = useTheme();
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const [stats, setStats] = useState<GetStatsReturn>(null);
  useEffect(() => {
    if (isPoolsLoaded && pool) {
      getStats(pool).then((result) => {
        setStats(result);
      });
    }
  }, [isPoolsLoaded, pool]);

  const [assetLeft, assetRight] = assets;
  const [amountLeft, amountRight] = amounts;
  if (!stats) return <CircularProgress />;
  const totalLiquidityA =
    stats[assetLeft.getPicassoAssetId().toString()].total.liquidity;
  const totalLiquidityB =
    stats[assetRight.getPicassoAssetId().toString()].total.liquidity;
  const ratioA = totalLiquidityA.isZero()
    ? 100
    : amountLeft.div(totalLiquidityA).multipliedBy(100).toNumber();
  const ratioB = totalLiquidityB.isZero()
    ? 100
    : amountRight.div(totalLiquidityB).multipliedBy(100).toNumber();

  if (!stats) return null;

  return (
    <Box
      borderTop={
        noDivider
          ? undefined
          : `1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.main
            )}`
      }
      {...rest}
    >
      {!noTitle && (
        <Typography variant="h6" mt={4}>
          Your position
        </Typography>
      )}
      <Label
        mt={noTitle ? 3 : 4}
        BalanceProps={{
          balance: expectedLP.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      >
        <PairAsset assets={assets} separator="/" />
      </Label>

      <Label
        mt={3}
        label="Share of pool"
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: `${((ratioA + ratioB) / 2).toFixed()}%`,
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />

      <Label
        mt={3}
        label={`Pooled ${assets[0].getSymbol()}`}
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: amountLeft.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />

      <Label
        mt={3}
        label={`Pooled ${assets[1].getSymbol()}`}
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: amountRight.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />
    </Box>
  );
};

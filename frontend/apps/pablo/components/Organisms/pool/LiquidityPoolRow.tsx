import { Box, TableCell, TableRow, Typography } from "@mui/material";
import { useAssetIdOraclePrice, useAssets } from "@/defi/hooks";
import { PairAsset } from "@/components/Atoms";
import { useLiquidityPoolStats } from "@/defi/hooks/useLiquidityPoolStats";
import {
  calculatePoolTotalValueLocked,
  DEFAULT_UI_FORMAT_DECIMALS,
} from "@/defi/utils";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { calculateRewardPerDayByAssetId } from "@/defi/utils/stakingRewards/math";
import { useStakingRewardsPoolApy } from "@/defi/hooks/stakingRewards/useStakingRewardsPoolApy";
import { useMemo } from "react";
import { DualAssetConstantProduct } from "shared";
import { useLiquidity } from "@/defi/hooks/useLiquidity";
import millify from "millify";
import BigNumber from "bignumber.js";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick,
}: {
  liquidityPool: DualAssetConstantProduct;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
  const lpAssetId = liquidityPool
    .getLiquidityProviderToken()
    .getPicassoAssetId() as string;
  const rewardPool = useStakingRewardPool(lpAssetId);
  const rewardAssets = useAssets(
    rewardPool ? Object.keys(rewardPool.rewards) : []
  );

  const { baseAsset, quoteAsset } = useMemo(() => {
    const assets = liquidityPool.getLiquidityProviderToken().getUnderlyingAssets();
    let baseAsset, quoteAsset;

    if (assets.length > 0) {
      baseAsset = assets[0];
      quoteAsset = assets[1];
    }

    return {
      baseAsset,
      quoteAsset
    }
  }, [liquidityPool]);

  const poolStats = useLiquidityPoolStats(liquidityPool);
  const liquidity = useLiquidity(liquidityPool);

  const quoteAssetPriceUSD = useAssetIdOraclePrice(baseAsset?.getPicassoAssetId());
  const baseAssetPriceUSD = useAssetIdOraclePrice(quoteAsset?.getPicassoAssetId());

  const apy = useStakingRewardsPoolApy(lpAssetId);
  const rewardAPYs = useMemo(() => {
    return Object.keys(apy).reduce((v, i) => {
      return v.plus(apy[i]);
    }, new BigNumber(0));
  }, [apy]);

  return (
    <TableRow
      onClick={(e) => {
        handleRowClick(e, liquidityPool.getPoolId() as string);
      }}
      key={liquidityPool.getPoolId() as string}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        {baseAsset && quoteAsset && (
          <PairAsset
            assets={liquidityPool
              .getLiquidityProviderToken()
              .getUnderlyingAssetJSON()}
            separator="/"
          />
        )}
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {millify(
            calculatePoolTotalValueLocked(
              liquidity.baseAmount,
              liquidity.quoteAmount,
              baseAssetPriceUSD,
              quoteAssetPriceUSD
            ).toNumber()
          )}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {rewardAPYs.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        {rewardAssets
          ? rewardAssets.map((item) => {
              return (
                <Box key={item.getName()} display="flex">
                  <PairAsset
                    assets={[
                      {
                        icon: item.getIconUrl(),
                        label: item.getSymbol(),
                      },
                    ]}
                    label={calculateRewardPerDayByAssetId(
                      item.getPicassoAssetId() as string,
                      rewardPool
                    ).toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
                  />
                </Box>
              );
            })
          : null}
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {poolStats
            ? quoteAssetPriceUSD
                .times(poolStats.totalVolume)
                .toFormat(DEFAULT_UI_FORMAT_DECIMALS)
            : 0}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export default LiquidityPoolRow;

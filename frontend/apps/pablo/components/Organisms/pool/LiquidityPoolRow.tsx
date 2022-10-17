import { TableCell, TableRow, Box, Typography } from "@mui/material";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { useAsset, useAssets } from "@/defi/hooks";
import { useLiquidityByPool } from "@/store/hooks/useLiquidityByPool";
import millify from "millify";
import { PairAsset } from "@/components/Atoms";
import { useLiquidityPoolStats } from "@/store/hooks/useLiquidityPoolStats";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import {
  calculatePoolTotalValueLocked,
  DEFAULT_NETWORK_ID,
  DEFAULT_UI_FORMAT_DECIMALS,
} from "@/defi/utils";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import { calculateRewardPerDayByAssetId } from "@/defi/utils/stakingRewards/math";
import { useStakingRewardsPoolApy } from "@/defi/hooks/stakingRewards/useStakingRewardsPoolApy";
import { useMemo } from "react";
import BigNumber from "bignumber.js";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick,
}: {
  liquidityPool: StableSwapPool | ConstantProductPool;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
  const rewardPool = useStakingRewardPool(liquidityPool.lpToken);
  const rewardAssets = useAssets(
    rewardPool ? Object.keys(rewardPool.rewards) : []
  );

  const baseAsset = useAsset(liquidityPool.pair.base.toString());
  const quoteAsset = useAsset(liquidityPool.pair.quote.toString());

  const poolStats = useLiquidityPoolStats(liquidityPool);
  const liquidity = useLiquidityByPool(liquidityPool);

  const quoteAssetPriceUSD = useUSDPriceByAssetId(
    liquidityPool.pair.quote.toString()
  );
  const baseAssetPriceUSD = useUSDPriceByAssetId(
    liquidityPool.pair.base.toString()
  );

  const apy = useStakingRewardsPoolApy(liquidityPool?.lpToken ?? "-");

  const rewardAPYs = useMemo(() => {
    return Object.keys(apy).reduce((v, i) => {
      return v.plus(apy[i])
    }, new BigNumber(0))
  }, [apy]);

  return (
    <TableRow
      onClick={(e) => {
        handleRowClick(e, liquidityPool.poolId.toString());
      }}
      key={liquidityPool.poolId.toString()}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        {baseAsset && quoteAsset && (
          <PairAsset
            assets={[
              {
                icon: baseAsset.icon,
                label: baseAsset.symbol,
              },
              {
                icon: quoteAsset.icon,
                label: quoteAsset.symbol,
              },
            ]}
            separator="/"
          />
        )}
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {millify(
            calculatePoolTotalValueLocked(
              liquidity.tokenAmounts.baseAmount,
              liquidity.tokenAmounts.quoteAmount,
              baseAssetPriceUSD,
              quoteAssetPriceUSD
            ).toNumber()
          )}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{
          rewardAPYs.toFixed(DEFAULT_UI_FORMAT_DECIMALS)
        }%</Typography>
      </TableCell>
      <TableCell align="left">
        {rewardAssets
          ? rewardAssets.map((item) => {
              return (
                <Box key={item.name} display="flex">
                  <PairAsset
                    assets={[
                      {
                        icon: item.icon,
                        label: item.symbol,
                      },
                    ]}
                    label={calculateRewardPerDayByAssetId(
                      item.network[DEFAULT_NETWORK_ID],
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

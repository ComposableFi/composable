import { TableCell, TableRow, Box, Typography } from "@mui/material";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { useAsset } from "@/defi/hooks";
import { useLiquidityByPool } from "@/store/hooks/useLiquidityByPool";
import millify from "millify";
import { PairAsset } from "@/components/Atoms";
import { useLiquidityPoolStats } from "@/store/hooks/useLiquidityPoolStats";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import {
  calculatePoolTotalValueLocked,
  DEFAULT_UI_FORMAT_DECIMALS,
} from "@/defi/utils";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick
}: {
  liquidityPool: StableSwapPool | ConstantProductPool;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
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

  return (
    <TableRow
      onClick={(e) => {
        handleRowClick(e, liquidityPool.poolId.toString())
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
        <Typography variant="body2">{0}%</Typography>
      </TableCell>
      <TableCell align="left">
        {poolStats
          ? poolStats.dailyRewards.map((item) => {
              return (
                <Box key={item.assetId} display="flex">
                  <PairAsset
                    assets={[
                      {
                        icon: item.icon,
                        label: item.symbol,
                      },
                    ]}
                    label={item.rewardAmount}
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

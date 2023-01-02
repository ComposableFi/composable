import { TableCell, TableRow } from "@mui/material";
import { PoolConfig } from "@/store/createPool/types";
import useStore from "@/store/useStore";
import { PairAsset } from "@/components/Atoms";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import { usePoolTotalVolume } from "@/defi/hooks/pools/usePoolTotalVolume";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick,
}: {
  liquidityPool: PoolConfig;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
  const isLoaded = useStore((store) => store.substrateTokens.hasFetchedTokens);
  const assets = liquidityPool.config.assets;
  const { poolTVL } = usePoolRatio(liquidityPool);
  const poolVolume = usePoolTotalVolume(liquidityPool);

  if (isLoaded) {
    return (
      <TableRow
        onClick={(e) => {
          handleRowClick(e, liquidityPool.poolId.toString());
        }}
        key={liquidityPool.poolId.toString()}
        sx={{ cursor: "pointer" }}
      >
        <TableCell align="left">
          <PairAsset assets={assets} separator="/" />
        </TableCell>
        <TableCell align="left">${poolTVL.toFormat(0)}</TableCell>
        <TableCell align="left">${poolVolume.toFormat(0)}</TableCell>
      </TableRow>
    );
  }

  return null;
};

export default LiquidityPoolRow;

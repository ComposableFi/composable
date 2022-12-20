import { TableCell, TableRow } from "@mui/material";
import { PoolConfig } from "@/store/createPool/types";
import useStore from "@/store/useStore";
import { PairAsset } from "@/components/Atoms";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick,
}: {
  liquidityPool: PoolConfig;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
  const tokens = useStore((store) => store.substrateTokens.tokens);
  const isLoaded = useStore((store) => store.substrateTokens.hasFetchedTokens);
  const lpAssetId = liquidityPool.config.lpToken;
  const assets = liquidityPool.config.assets;

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
        <TableCell align="left"></TableCell>
        <TableCell align="left"></TableCell>
        <TableCell align="left"></TableCell>
        <TableCell align="left"></TableCell>
      </TableRow>
    );
  }

  return null;
};

export default LiquidityPoolRow;

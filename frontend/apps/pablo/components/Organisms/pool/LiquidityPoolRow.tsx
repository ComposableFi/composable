import { TableCell, TableRow, Typography } from "@mui/material";
import { PoolConfig } from "@/store/createPool/types";
import { option, readonlyArray } from "fp-ts";
import { pipe } from "fp-ts/function";
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
  console.log(liquidityPool.config.feeConfig);
  const assets = pipe(
    Object.keys(liquidityPool.config.assetsWeights),
    readonlyArray.map((asset) =>
      pipe(
        Object.values(tokens).find(
          (token) => token.getIdOnChain("picasso") === asset
        ),
        option.fromNullable
      )
    ),
    readonlyArray.compact,
    readonlyArray.toArray
  );
  console.log({
    assets,
    lpAssetId,
  });

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
        <TableCell align="left">
          <Typography variant="body2">N/A</Typography>
        </TableCell>
        <TableCell align="left">
          <Typography variant="body2">N/A</Typography>
        </TableCell>
        <TableCell align="left">N/A</TableCell>
        <TableCell align="left">
          <Typography variant="body2">N/A</Typography>
        </TableCell>
      </TableRow>
    );
  }

  return null;
};

export default LiquidityPoolRow;

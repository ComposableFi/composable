import { TableCell, TableRow } from "@mui/material";
import { PoolConfig } from "@/store/createPool/types";
import useStore from "@/store/useStore";
import { PairAsset } from "@/components/Atoms";
import { useEffect, useState } from "react";
import {
  getPoolTVL,
  getPoolVolume,
  getStats,
  GetStatsReturn,
} from "@/defi/utils";

const LiquidityPoolRow = ({
  liquidityPool,
  handleRowClick,
}: {
  liquidityPool: PoolConfig;
  handleRowClick: (e: any, poolId: string) => void;
}) => {
  const isLoaded = useStore((store) => store.substrateTokens.hasFetchedTokens);
  const assets = liquidityPool.config.assets;
  const [stats, setStats] = useState<GetStatsReturn>(null);
  useEffect(() => {
    getStats(liquidityPool).then((result) => {
      setStats(result);
    });
  }, [liquidityPool]);

  if (isLoaded) {
    const poolTVL = getPoolTVL(stats).toFormat(0);
    const poolVolume = getPoolVolume(stats).toFormat(0);
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
        <TableCell align="left">${poolTVL}</TableCell>
        <TableCell align="left">${poolVolume}</TableCell>
      </TableRow>
    );
  }

  return null;
};

export default LiquidityPoolRow;

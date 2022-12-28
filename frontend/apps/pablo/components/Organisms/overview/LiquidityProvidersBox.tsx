import { BoxProps } from "@mui/material";
import React, { FC } from "react";
import { BoxWrapper } from "../BoxWrapper";
import { NoPositionsPlaceholder } from "./NoPositionsPlaceholder";
import { OVERVIEW_ERRORS } from "./errors";
import useStore from "@/store/useStore";
import { YourLiquidityTable } from "@/components/Organisms/pool/YourLiquidityTable";
import BigNumber from "bignumber.js";

export const LiquidityProvidersBox: FC<BoxProps> = ({ ...boxProps }) => {
  const pools = useStore((store) => store.pools.config);
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const hasParticipated = pools.some((pool) => {
    const balance = userOwnedLiquidity[pool.config.lpToken]?.balance ?? {
      free: new BigNumber(0),
      locked: new BigNumber(0),
    };

    return !balance.free.isZero();
  });

  console.log(hasParticipated);

  return (
    <BoxWrapper title="Liquidity provider positions" {...boxProps}>
      {pools.length === 0 || !hasParticipated ? (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_LP} />
      ) : (
        <YourLiquidityTable pools={pools} />
      )}
    </BoxWrapper>
  );
};

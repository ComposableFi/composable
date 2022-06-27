import {
  Box,
  BoxProps,
  TypographyProps,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { Label } from "@/components";
import { AssetId } from "@/defi/polkadot/types";
import { Assets } from "@/defi/polkadot/Assets";
import { useMemo } from "react";
import { LiquidityPoolType } from "@/store/pools/pools.types";


export type SwapSummaryProps = {
  quoteAssetId: AssetId | "none",
  baseAssetId: AssetId | "none",
  poolType: LiquidityPoolType | "none",
  
  minimumReceived: BigNumber,
  priceImpact: number,
  PriceImpactProps?: TypographyProps,
  baseAssetAmount: BigNumber,
  quoteAssetAmount: BigNumber,
  fee: BigNumber,
  price?: BigNumber,
} & BoxProps;

export const SwapSummary: React.FC<SwapSummaryProps> = ({
  quoteAssetId,
  baseAssetId,
  poolType,
  minimumReceived,
  baseAssetAmount,
  quoteAssetAmount,
  priceImpact,
  PriceImpactProps,
  fee,
  price,
  ...boxProps
}) => {

  const validTokens = quoteAssetId !== "none" && baseAssetId !== "none";
  const feeCharged = useMemo(() => {
    if (validTokens) {
      return new BigNumber(quoteAssetAmount).times(fee)
    } else {
      return new BigNumber(0);
    }
  }, [quoteAssetAmount, validTokens, fee]);

  if (!validTokens) {
    return <></>;
  }

  return (
    <Box {...boxProps}>
      {price && (
        <Label
          label="Price"
          BalanceProps={{
            balance: `1 ${Assets[baseAssetId].symbol} = ${price.toFixed()} ${Assets[quoteAssetId].symbol}`
          }}
          mb={2}
        />
      )}
      <Label
        label="Minimum recieved"
        TooltipProps={{
          title: "Minimum recieved"
        }}
        BalanceProps={{
          balance: `${minimumReceived.toFixed()} ${Assets[baseAssetId].symbol}`
        }}
        mb={2}
      />
      {/* <Label
        label="Price impact"
        TooltipProps={{
          title: "Price impact"
        }}
        BalanceProps={{
          balance: `${priceImpact.toFixed(4)} %`,
          BalanceTypographyProps: {
            color: "primary.main",
            ...PriceImpactProps,
          },
        }}
        mb={2}
      /> */}
      <Label
        label="Liquidity provider fee"
        TooltipProps={{
          title: "Liquidity provider fee"
        }}
        BalanceProps={{
          balance: `${feeCharged.toFixed(4)} ${
            poolType !== "none" && poolType !== "StableSwap" ? Assets[quoteAssetId].symbol : Assets[baseAssetId].symbol
          }`
        }}
        mb={0}
      />
    </Box>
  );
}
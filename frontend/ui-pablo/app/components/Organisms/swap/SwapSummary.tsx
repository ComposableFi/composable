import {
  Box,
  BoxProps,
  TypographyProps,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { Label } from "@/components";
import { MockedAsset } from "@/store/assets/assets.types";


export type SwapSummaryProps = {
  quoteAsset: MockedAsset | undefined,
  baseAsset: MockedAsset | undefined,
  
  minimumReceived: BigNumber,
  // priceImpact: number,
  // PriceImpactProps?: TypographyProps,
  baseAssetAmount: BigNumber,
  quoteAssetAmount: BigNumber,
  feeCharged: BigNumber,
  spotPrice: BigNumber,
} & BoxProps;

export const SwapSummary: React.FC<SwapSummaryProps> = ({
  quoteAsset,
  baseAsset,
  minimumReceived,
  baseAssetAmount,
  quoteAssetAmount,
  // priceImpact,
  // PriceImpactProps,
  feeCharged,
  spotPrice,
  ...boxProps
}) => {

  const validTokens = !!baseAsset && !!quoteAsset;

  if (!validTokens) {
    return <></>;
  }

  return (
    <Box {...boxProps}>
      <Label
        label="Price"
        BalanceProps={{
          balance: `1 ${baseAsset?.symbol} = ${spotPrice.toFixed()} ${quoteAsset?.symbol}`
        }}
        mb={2}
      />

      <Label
        label="Minimum recieved"
        TooltipProps={{
          title: "Minimum recieved"
        }}
        BalanceProps={{
          balance: `${minimumReceived.toFixed()} ${baseAsset?.symbol}`
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
          balance: `${feeCharged.toFixed(4)} ${quoteAsset?.symbol}`
        }}
        mb={0}
      />
    </Box>
  );
}
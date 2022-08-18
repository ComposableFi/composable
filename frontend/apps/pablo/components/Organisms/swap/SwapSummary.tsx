import { Box, BoxProps } from "@mui/material";
import BigNumber from "bignumber.js";
import { Label } from "@/components";
import { MockedAsset } from "@/store/assets/assets.types";
import { useEffect, useState } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, fromChainUnits, toChainUnits } from "@/defi/utils";

export type SwapSummaryProps = {
  quoteAsset: MockedAsset | undefined;
  baseAsset: MockedAsset | undefined;

  minimumReceived: BigNumber;
  // priceImpact: number,
  // PriceImpactProps?: TypographyProps,
  baseAssetAmount: BigNumber;
  quoteAssetAmount: BigNumber;
  feeCharged: BigNumber;
  spotPrice: BigNumber;
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
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const validTokens = !!baseAsset && !!quoteAsset;

  const [estimatedGas, setGasEstimated] = useState(new BigNumber(0));
  useEffect(() => {
    if (parachainApi && selectedAccount && baseAsset && quoteAsset) {
      let pair = {
        base: baseAsset.network[DEFAULT_NETWORK_ID],
        quote: quoteAsset.network[DEFAULT_NETWORK_ID],
      };

      parachainApi.tx.dexRouter
        .exchange(
          pair,
          parachainApi.createType(
            "u128",
            toChainUnits(quoteAssetAmount).toString()
          ),
          parachainApi.createType(
            "u128",
            toChainUnits(minimumReceived).toString()
          )
        )
        .paymentInfo(selectedAccount.address)
        .then((gasInfo) => {
          setGasEstimated(fromChainUnits(gasInfo.partialFee.toString()));
        });
    }
  }, [
    baseAsset,
    quoteAsset,
    quoteAssetAmount,
    minimumReceived,
    selectedAccount,
    parachainApi,
  ]);

  if (!validTokens) {
    return <></>;
  }

  return (
    <Box {...boxProps}>
      <Label
        label="Price"
        BalanceProps={{
          balance: `1 ${baseAsset?.symbol} = ${spotPrice.toFixed()} ${
            quoteAsset?.symbol
          }`,
        }}
        mb={2}
      />

      <Label
        label="Minimum recieved"
        TooltipProps={{
          title: "Minimum recieved",
        }}
        BalanceProps={{
          balance: `${minimumReceived.toFixed()} ${baseAsset?.symbol}`,
        }}
        mb={2}
      />
      <Label
        label="Estimated Gas"
        TooltipProps={{
          title: "Estimated Gas",
        }}
        BalanceProps={{
          balance: `${estimatedGas.toFixed(4)} PICA`,
          // BalanceTypographyProps: {
          //   color: "primary.main",
          //   ...PriceImpactProps,
          // },
        }}
        mb={2}
      />
      <Label
        label="Liquidity provider fee"
        TooltipProps={{
          title: "Liquidity provider fee",
        }}
        BalanceProps={{
          balance: `${feeCharged.toFixed(4)} ${quoteAsset?.symbol}`,
        }}
        mb={0}
      />
    </Box>
  );
};

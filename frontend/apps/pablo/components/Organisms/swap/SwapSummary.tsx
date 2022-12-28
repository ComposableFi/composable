import { Box, BoxProps, TypographyProps } from "@mui/material";
import BigNumber from "bignumber.js";
import { Label } from "@/components";
import { Asset } from "shared";
import { FC, useEffect, useState } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, fromChainUnits, toChainUnits } from "@/defi/utils";
import { PoolConfig } from "@/store/pools/types";

export type SwapSummaryProps = {
  quoteAsset: Asset | undefined;
  baseAsset: Asset | undefined;
  minimumReceived: {
    asset: Asset | undefined;
    value: BigNumber;
  };
  priceImpact: BigNumber;
  PriceImpactProps?: TypographyProps;
  baseAssetAmount: BigNumber;
  quoteAssetAmount: BigNumber;
  feeCharged: BigNumber;
  spotPrice: BigNumber;
  pool: PoolConfig;
} & BoxProps;

export const SwapSummary: FC<SwapSummaryProps> = ({
  pool,
  quoteAsset,
  baseAsset,
  minimumReceived,
  baseAssetAmount,
  quoteAssetAmount,
  priceImpact,
  PriceImpactProps,
  feeCharged,
  spotPrice,
  ...boxProps
}) => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const validTokens = !!baseAsset && !!quoteAsset;

  const [estimatedGas, setGasEstimated] = useState(new BigNumber(0));
  useEffect(() => {
    const baseAssetId = baseAsset?.getPicassoAssetId()?.toString();
    const quoteAssetId = quoteAsset?.getPicassoAssetId()?.toString();

    if (
      parachainApi &&
      selectedAccount &&
      baseAssetId &&
      quoteAssetId &&
      quoteAsset &&
      baseAsset &&
      pool
    ) {
      parachainApi.tx.pablo
        .swap(
          pool.poolId.toString(),
          {
            assetId: quoteAssetId,
            amount: toChainUnits(
              quoteAssetAmount.toString(),
              quoteAsset.getDecimals(DEFAULT_NETWORK_ID)
            ).toString(),
          },
          {
            assetId: baseAssetId,
            amount: toChainUnits(
              baseAssetAmount.toString(),
              baseAsset.getDecimals(DEFAULT_NETWORK_ID)
            ).toString(),
          },
          true
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
    pool,
    baseAssetAmount,
  ]);

  if (!validTokens) {
    return <></>;
  }

  return (
    <Box {...boxProps}>
      <Label
        label="Price"
        BalanceProps={{
          balance: `1 ${baseAsset?.getSymbol()} = ${spotPrice.toFormat(
            quoteAsset?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )} ${quoteAsset?.getSymbol()}`,
        }}
        mb={2}
      />

      <Label
        label="Minimum received"
        TooltipProps={{
          title: "Minimum received",
        }}
        BalanceProps={{
          balance: `${minimumReceived.value.toFormat(
            minimumReceived.asset?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )} ${minimumReceived.asset?.getSymbol()}`,
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
        }}
        mb={2}
      />
      <Label
        label="Liquidity provider fee"
        TooltipProps={{
          title: "Liquidity provider fee",
        }}
        BalanceProps={{
          balance: `${feeCharged.toFixed(4)} ${quoteAsset?.getSymbol()}`,
        }}
        mb={0}
      />
    </Box>
  );
};

import React from "react";
import { BaseAsset, Label, PairAsset } from "@/components/Atoms";
import { Box, BoxProps } from "@mui/material";

import BigNumber from "bignumber.js";
import millify from "millify";
import { Asset } from "shared";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export type PreviewDetailsProps = {
  token1: Asset | undefined;
  token2: Asset | undefined;
  lpToRemove: BigNumber;
  expectedReceiveAmountToken1: BigNumber;
  expectedReceiveAmountToken2: BigNumber;
  price1: BigNumber;
  price2: BigNumber;
} & BoxProps;

export const PreviewDetails: React.FC<PreviewDetailsProps> = ({
  token1,
  token2,
  lpToRemove,
  expectedReceiveAmountToken1,
  expectedReceiveAmountToken2,
  price1,
  price2,
  ...rest
}) => {
  return (
    <Box {...rest}>
      <Label
        BalanceProps={{
          balance: `${lpToRemove.eq(0) ? "-" : millify(lpToRemove.toNumber())}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        {token1 && token2 && <PairAsset assets={[]} separator="/" />}
      </Label>

      <Label
        mt={3}
        BalanceProps={{
          balance: expectedReceiveAmountToken1.toFormat(
            token1?.getDecimals(DEFAULT_NETWORK_ID)
          ),
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset
          icon={token1?.getIconUrl()}
          label={`Expected ` + token1?.getSymbol()}
        />
      </Label>

      <Label
        mt={3}
        BalanceProps={{
          balance: expectedReceiveAmountToken2.toFormat(
            token2?.getDecimals(DEFAULT_NETWORK_ID)
          ),
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset
          icon={token2?.getIconUrl()}
          label={`Expected ` + token2?.getSymbol()}
        />
      </Label>

      <Label
        mt={4}
        label={`Price`}
        BalanceProps={{
          balance: `1 ${token2?.getSymbol()} = ${price2.toFormat(
            token1?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )} ${token1?.getSymbol()}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />

      <Label
        mt={2}
        label=""
        BalanceProps={{
          balance: `1 ${token1?.getSymbol()} = ${price1.toFormat(
            token2?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )} ${token2?.getSymbol()}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />
    </Box>
  );
};

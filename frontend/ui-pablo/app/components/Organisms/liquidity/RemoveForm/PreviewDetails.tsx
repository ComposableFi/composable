import React from "react";
import { Label, BaseAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import {
  Box,
  BoxProps,
} from "@mui/material";

import BigNumber from "bignumber.js";

export type PreviewDetailsProps = {
  tokenId1: TokenId,
  tokenId2: TokenId,
  amount1: BigNumber,
  amount2: BigNumber,
  price1: BigNumber,
  price2: BigNumber,
} & BoxProps;

export const PreviewDetails: React.FC<PreviewDetailsProps> = ({
  tokenId1,
  tokenId2,
  amount1,
  amount2,
  price1,
  price2,
  ...rest
}) => {
  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);

  return (
    <Box {...rest}>
      <Label
        BalanceProps={{
          balance: `${amount1.eq(0) ? '-' : amount1}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={token1?.icon} label={token1?.symbol} />
      </Label>

      <Label
        mt={3}
        BalanceProps={{
          balance: `${amount2.eq(0) ? '-' : amount2}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={token2?.icon} label={token2?.symbol} />
      </Label>

      <Label
        mt={4}
        label={`Price`}
        BalanceProps={{
          balance: `1 ${token2?.symbol} = ${price2} ${token1?.symbol}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />

      <Label
        mt={2}
        label=""
        BalanceProps={{
          balance: `1 ${token1?.symbol} = ${price1} ${token2?.symbol}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />
    </Box>
  );
};


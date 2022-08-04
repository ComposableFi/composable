import React from "react";
import { Label, BaseAsset, PairAsset } from "@/components/Atoms";
import {
  Box,
  BoxProps,
} from "@mui/material";

import BigNumber from "bignumber.js";
import { MockedAsset } from "@/store/assets/assets.types";
import millify from "millify";

export type PreviewDetailsProps = {
  token1: MockedAsset | undefined,
  token2: MockedAsset | undefined,
  lpToRemove: BigNumber,
  expectedReceiveAmountToken1: BigNumber,
  expectedReceiveAmountToken2: BigNumber,
  price1: BigNumber,
  price2: BigNumber,
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
      <Label BalanceProps={{
        balance: `${lpToRemove.eq(0) ? '-' : millify(lpToRemove.toNumber())}`,
        BalanceTypographyProps: {
          variant: "body1",
        },
      }}
      >
        {token1 && token2 &&
          <PairAsset
            assets={[
              {
                icon: token1.icon,
                label: token1.symbol,
              },
              {
                icon: token2.icon,
                label: token2.symbol,
              },
            ]}
            separator="/"
          />

        }
      </Label>

      <Label mt={3}
        BalanceProps={{
          balance: `${millify(expectedReceiveAmountToken1.toNumber())}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={token1?.icon} label={`Expected ` + token1?.symbol} />
      </Label>

      <Label
        mt={3}
        BalanceProps={{
          balance: `${millify(expectedReceiveAmountToken2.toNumber())}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={token2?.icon} label={`Expected ` + token2?.symbol} />
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


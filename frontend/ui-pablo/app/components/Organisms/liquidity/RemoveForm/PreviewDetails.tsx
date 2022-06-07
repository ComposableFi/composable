import React from "react";
import { Label, BaseAsset, PairAsset } from "@/components/Atoms";
import {
  Box,
  BoxProps,
} from "@mui/material";

import BigNumber from "bignumber.js";
import { AssetMetadata } from "@/defi/polkadot/Assets";

export type PreviewDetailsProps = {
  tokenId1: AssetMetadata,
  tokenId2: AssetMetadata,
  lpToRemove: BigNumber,
  expectedRecieveAmountToken1: BigNumber,
  expectedRecieveAmountToken2: BigNumber,
  price1: BigNumber,
  price2: BigNumber,
} & BoxProps;

export const PreviewDetails: React.FC<PreviewDetailsProps> = ({
  tokenId1,
  tokenId2,
  lpToRemove,
  expectedRecieveAmountToken1,
  expectedRecieveAmountToken2,
  price1,
  price2,
  ...rest
}) => {

  return (
    <Box {...rest}>

      <Label BalanceProps={{
          balance: `${lpToRemove.eq(0) ? '-' : lpToRemove}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <PairAsset
            assets={[
              {
                icon: tokenId1?.icon,
                label: tokenId1?.symbol,
              },
              {
                icon: tokenId2?.icon,
                label: tokenId2?.symbol,
              },
            ]}
            separator="/"
        />
      </Label>

      <Label mt={3}
        BalanceProps={{
          balance: `${expectedRecieveAmountToken1.toFormat()}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={tokenId1.icon} label={`Expected ` + tokenId1?.symbol} />
      </Label>

      <Label
        mt={3}
        BalanceProps={{
          balance: `${expectedRecieveAmountToken2.toFormat()}`,
          BalanceTypographyProps: {
            variant: "body1",
          },
        }}
      >
        <BaseAsset icon={tokenId2?.icon} label={`Expected ` + tokenId2?.symbol} />
      </Label>

      <Label
        mt={4}
        label={`Price`}
        BalanceProps={{
          balance: `1 ${tokenId2?.symbol} = ${price2} ${tokenId1?.symbol}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />

      <Label
        mt={2}
        label=""
        BalanceProps={{
          balance: `1 ${tokenId1?.symbol} = ${price1} ${tokenId2?.symbol}`,
          BalanceTypographyProps: {
            variant: "body2",
          },
        }}
      />
    </Box>
  );
};


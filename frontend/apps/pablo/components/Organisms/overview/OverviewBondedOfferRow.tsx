import { TableCell, TableRow, Typography } from "@mui/material";
import {
  useBondedOfferVestingState,
  useBondOfferROI,
} from "@/store/bond/bond.slice";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import BondPrincipalAssetIcon from "../bonds/BondPrincipalAssetIcon";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { BondOffer } from "shared";
import { useBondedAsset, useAssetIdOraclePrice } from "@/defi/hooks";
import BigNumber from "bignumber.js";

export const OverviewBondedOfferRow = ({
  bondOffer,
  offerId,
}: {
  offerId: string;
  bondOffer: BondOffer;
}) => {
  const bondedAsset_s = useBondedAsset(bondOffer);
  const discount = useBondOfferROI(offerId);
  const vestingTime = useBondVestingTime(bondOffer);
  const rewardAssetPriceUSD = useAssetIdOraclePrice(bondOffer.getRewardAssetId() as string);

  const { claimable } = useBondedOfferVestingState(offerId);

  return (
    <TableRow>
      <TableCell align="left">
        <BondPrincipalAssetIcon bondedAsset={bondedAsset_s} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          {discount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          {(bondOffer.getRewardAssetAmount(true) as BigNumber)
            .div(bondOffer.getNumberOfBonds())
            .toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          $
          {claimable
            .times(rewardAssetPriceUSD)
            .toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{vestingTime}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          {claimable.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

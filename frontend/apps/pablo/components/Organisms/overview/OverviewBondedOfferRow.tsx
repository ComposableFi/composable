import { TableCell, TableRow, Typography } from "@mui/material";
import { BondOffer } from "@/defi/types";
import {
  useBondedOfferVestingState,
  useBondOfferROI,
} from "@/store/bond/bond.slice";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import BondPrincipalAssetIcon from "../bonds/BondPrincipalAssetIcon";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";

export const OverviewBondedOfferRow = ({
  bondOffer,
  offerId,
}: {
  offerId: string;
  bondOffer: BondOffer;
}) => {
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const discount = useBondOfferROI(offerId);
  const vestingTime = useBondVestingTime(bondOffer);
  const rewardAssetPriceUSD = useUSDPriceByAssetId(bondOffer.reward.asset);

  const { claimable } = useBondedOfferVestingState(offerId);

  return (
    <TableRow>
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          {discount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          {bondOffer.reward.amount
            .div(bondOffer.nbOfBonds)
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

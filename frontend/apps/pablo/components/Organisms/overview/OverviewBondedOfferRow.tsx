import {
  TableCell,
  TableRow,
  Typography,
} from "@mui/material";
import { BondOffer } from "@/defi/types";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import BondPrincipalAssetIcon from "../bonds/BondPrincipalAssetIcon";
import { useBondedOfferVestingSchedules, useBondedOfferVestingState, useBondOfferROI } from "@/store/bond/bond.slice";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import BigNumber from "bignumber.js";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";

export const OverviewBondedOfferRow = ({
  bondOffer,
  offerId
}: {
  offerId: string;
  bondOffer: BondOffer;
}) => {
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const discount = useBondOfferROI(offerId);
  const vestingTime = useBondVestingTime(bondOffer);
  const rewardAssetPriceUSD = useUSDPriceByAssetId(bondOffer.reward.asset);

  const {
    claimable
  } = useBondedOfferVestingState(offerId);

  return (
    <TableRow>
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{discount.toFixed(2)}%</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{claimable.toFixed(2)}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">${claimable.times(rewardAssetPriceUSD).toFixed()}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{vestingTime}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{0}</Typography>
      </TableCell>
    </TableRow>
  );
};

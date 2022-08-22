import {
  TableCell,
  TableRow,
  Typography,
} from "@mui/material";
import { BondOffer } from "@/defi/types";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import BondPrincipalAssetIcon from "../bonds/BondPrincipalAssetIcon";
import { useBondedOfferVestingSchedules, useBondOfferROI } from "@/store/bond/bond.slice";
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
  const vestingSchedules = useBondedOfferVestingSchedules(offerId);

  const amountToClaim = vestingSchedules.reduce((acc, c) => {
    return acc.plus(c.perPeriod.times(c.periodCount));
  }, new BigNumber(0));

  const rewardAssetPriceUSD = useUSDPriceByAssetId(bondOffer.reward.asset);

  return (
    <TableRow>
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{discount.toFixed(2)}%</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{amountToClaim.toFixed(2)}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">${amountToClaim.times(rewardAssetPriceUSD).toFixed()}</Typography>
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

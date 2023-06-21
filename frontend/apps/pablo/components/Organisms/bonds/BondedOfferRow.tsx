import { BondOffer } from "shared";
import { TableCell, TableRow, Typography } from "@mui/material";
import { useAsset, useBondedAsset } from "@/defi/hooks";
import { useBondedOfferVestingState } from "@/store/bond/bond.slice";
import BondPrincipalAssetIcon from "./BondPrincipalAssetIcon";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";

const BondedOfferRow = ({ bondOffer, handleBondedOfferRowClick }: { bondOffer: BondOffer, handleBondedOfferRowClick: () => void }) => {
  const rewardAsset = useAsset(bondOffer.getRewardAssetId() as string);
  const bondedAsset_s = useBondedAsset(bondOffer);
  const vestingTime = useBondVestingTime(bondOffer);
  const { claimable, pendingRewards } = useBondedOfferVestingState(bondOffer.getBondOfferId() as string);

  return (
    <TableRow sx={{ cursor: "pointer" }} onClick={handleBondedOfferRowClick}>
      <TableCell align="left">
      <BondPrincipalAssetIcon bondedAsset={bondedAsset_s} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {claimable.toFixed(2)} {rewardAsset?.getSymbol()}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {pendingRewards.toFixed(2)} {rewardAsset?.getSymbol()}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{vestingTime}</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondedOfferRow;

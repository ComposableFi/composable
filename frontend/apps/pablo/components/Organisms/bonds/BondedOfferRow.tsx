import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondOffer } from "@/defi/types";
import { TableCell, TableRow, Typography } from "@mui/material";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";
import { useAsset } from "@/defi/hooks";
import { useBondedOfferVestingState } from "@/store/bond/bond.slice";
import BondPrincipalAssetIcon from "./BondPrincipalAssetIcon";

const BondedOfferRow = ({ bondOffer, handleBondedOfferRowClick }: { bondOffer: BondOffer, handleBondedOfferRowClick: () => void }) => {
  const rewardAsset = useAsset(bondOffer.reward.asset);
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const vestingTime = useBondVestingTime(bondOffer);
  const { claimable, pendingRewards } = useBondedOfferVestingState(bondOffer.offerId.toString());

  return (
    <TableRow sx={{ cursor: "pointer" }} onClick={handleBondedOfferRowClick}>
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {claimable.toFixed(2)} {rewardAsset?.symbol}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {pendingRewards.toFixed(2)} {rewardAsset?.symbol.toUpperCase()}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{vestingTime}</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondedOfferRow;

import { TableCell, TableRow, Typography } from "@mui/material";

import { BondOffer } from "@/defi/types";
import useBondOfferROI from "@/defi/hooks/bonds/useBondOfferROI";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import useTotalPurchasedBondOffer from "@/defi/hooks/bonds/useTotalPurchased";
import useBondPrice from "@/defi/hooks/bonds/useBondPrice";
import BondOfferRowIcon from "./BondOfferRowIcon";

const BondOfferRow = ({
  bondOffer,
  handleBondClick,
}: {
  bondOffer: BondOffer;
  handleBondClick: (bondOfferId: string) => void;
}) => {
  const roi = useBondOfferROI(bondOffer);
  const totalPurchasedValue = useTotalPurchasedBondOffer(bondOffer);
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const bondPrice = useBondPrice(bondOffer);

  return (
    <TableRow
      key={bondOffer.offerId.toString()}
      onClick={() => handleBondClick(bondOffer.offerId.toString())}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        <BondOfferRowIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">${bondPrice.toFormat()}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2" color="featured.main">
          {roi.toFormat()}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          ${totalPurchasedValue.toFormat()}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondOfferRow;
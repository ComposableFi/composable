import { TableCell, TableRow, Typography } from "@mui/material";
import { BondOffer } from "shared";
import {
  useBondOfferPriceInAmountOfPrincipalTokens,
  useBondOfferROI,
  useBondOfferTotalPurchased,
} from "@/store/bond/bond.slice";
import BondPrincipalAssetIcon from "./BondPrincipalAssetIcon";
import { useBondedAsset } from "@/defi/hooks";
import { useAssetPrice } from "@/defi/hooks";

const BondOfferRow = ({
  bondOffer,
  handleBondClick,
  offerId,
}: {
  offerId: string;
  bondOffer: BondOffer;
  handleBondClick: (bondOfferId: string) => void;
}) => {
  const roi = useBondOfferROI(offerId);
  const totalPurchasedBonds = useBondOfferTotalPurchased(offerId);
  const bondedAsset_s = useBondedAsset(bondOffer);
  const bondedAssetPriceInUSD = useAssetPrice(bondedAsset_s);
  const principalAmountOfTokensRequiredToBuy = useBondOfferPriceInAmountOfPrincipalTokens(offerId);

  return (
    <TableRow
      key={bondOffer.getBondOfferId() as string}
      onClick={() => handleBondClick(bondOffer.getBondOfferId() as string)}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        <BondPrincipalAssetIcon bondedAsset={bondedAsset_s} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {principalAmountOfTokensRequiredToBuy
            .times(bondedAssetPriceInUSD)
            .toFormat(2)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2" color="featured.main">
          {roi.toFormat()}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {totalPurchasedBonds
            .times(principalAmountOfTokensRequiredToBuy)
            .times(bondedAssetPriceInUSD)
            .toFormat(2)}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondOfferRow;

import { TableCell, TableRow, Typography } from "@mui/material";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondOffer } from "@/defi/types";
import {
  useBondOfferPriceInAmountOfPrincipalTokens,
  useBondOfferROI,
  useBondOfferTotalPurchased,
} from "@/store/bond/bond.slice";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import BondPrincipalAssetIcon from "./BondPrincipalAssetIcon";

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
  const totalPuchasedBonds = useBondOfferTotalPurchased(offerId);
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const principalAmountOfTokensRequiredToBuy =
    useBondOfferPriceInAmountOfPrincipalTokens(offerId);
  const principalAssetPriceInUSD = useUSDPriceByAssetId(bondOffer.asset);

  return (
    <TableRow
      key={bondOffer.offerId.toString()}
      onClick={() => handleBondClick(bondOffer.offerId.toString())}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          $
          {principalAmountOfTokensRequiredToBuy
            .times(principalAssetPriceInUSD)
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
          {totalPuchasedBonds
            .times(principalAmountOfTokensRequiredToBuy)
            .times(principalAssetPriceInUSD)
            .toFormat(2)}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondOfferRow;

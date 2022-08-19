import { TableCell, TableRow, Typography } from "@mui/material";
import { PairAsset, BaseAsset } from "@/components/Atoms";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondPrincipalAsset, BondOffer } from "@/defi/types";
import {
  useBondOfferPriceInAmountOfPrincipalTokens,
  useBondOfferROI,
  useBondOfferTotalPurchased,
} from "@/store/bond/bond.slice";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";

const BondPrincipalAssetIcon = ({
  principalAsset,
}: {
  principalAsset: BondPrincipalAsset;
}) => {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  if (baseAsset && quoteAsset) {
    return (
      <PairAsset
        assets={[
          {
            icon: baseAsset.icon,
            label: baseAsset.symbol,
          },
          {
            icon: quoteAsset.icon,
            label: quoteAsset.symbol,
          },
        ]}
        separator="/"
      />
    );
  }

  if (simplePrincipalAsset) {
    return (
      <BaseAsset
        label={simplePrincipalAsset.symbol}
        icon={simplePrincipalAsset.icon}
      />
    );
  }

  return null;
};

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

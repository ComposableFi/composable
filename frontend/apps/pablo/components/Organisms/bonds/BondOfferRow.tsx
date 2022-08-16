import {
  TableCell,
  TableRow,
  Typography
} from "@mui/material";
import { PairAsset, BaseAsset } from "@/components/Atoms";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import useBondOfferROI from "@/defi/hooks/bonds/useBondOfferROI";
import useBondPrice from "@/defi/hooks/bonds/useBondPrice";
import useTotalPurchasedBondOffer from "@/defi/hooks/bonds/useTotalPurchased";
import { BondPrincipalAsset, BondOffer } from "@/defi/types";

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
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
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
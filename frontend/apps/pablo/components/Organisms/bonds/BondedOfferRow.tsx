import { PairAsset, BaseAsset } from "@/components/Atoms";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondOffer } from "@/defi/types";
import { TableCell, TableRow, Typography } from "@mui/material";

const BondedOfferRow = ({ bondOffer }: { bondOffer: BondOffer }) => {
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset

  return (
    <TableRow sx={{ cursor: "pointer" }}>
      <TableCell align="left">
        {baseAsset && quoteAsset ? (
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
        ) : simplePrincipalAsset ? (
          <BaseAsset label={simplePrincipalAsset.symbol} icon={simplePrincipalAsset.icon} />
        ) : null }
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {0} CHAOS
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          {0} CHAOS
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">{0}</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondedOfferRow;
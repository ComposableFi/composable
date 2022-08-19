import { PairAsset, BaseAsset } from "@/components/Atoms";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import { BondOffer } from "@/defi/types";
import { TableCell, TableRow, Typography } from "@mui/material";
import useBondVestingTime from "@/defi/hooks/bonds/useBondVestingTime";

const BondedOfferRow = ({ bondOffer, handleBondedOfferRowClick }: { bondOffer: BondOffer, handleBondedOfferRowClick: () => void }) => {
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  const vestingTime = useBondVestingTime(bondOffer);

  return (
    <TableRow sx={{ cursor: "pointer" }} onClick={handleBondedOfferRowClick}>
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
        <Typography variant="body2">{vestingTime}</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondedOfferRow;
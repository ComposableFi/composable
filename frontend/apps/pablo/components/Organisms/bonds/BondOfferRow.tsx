import { TableCell, TableRow, Typography } from "@mui/material";
import { Asset, BondOffer } from "shared";
import BondPrincipalAssetIcon from "./BondPrincipalAssetIcon";

const BondOfferRow = ({
  bondOffer,
  handleBondClick,
  offerId,
}: {
  offerId: string;
  bondOffer: BondOffer;
  handleBondClick?: (bondOfferId: string) => void;
}) => {
  return (
    <TableRow
      key={bondOffer.getBondOfferId() as string}
      onClick={() => handleBondClick?.(bondOffer.getBondOfferId() as string)}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        <BondPrincipalAssetIcon
          bondedAsset={
            new Asset("Chaos", "CHAOS", "/tokens/chaos.svg", "chaos")
          }
        />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">$12.00</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2" color="featured.main">
          8.01%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">$42.32</Typography>
      </TableCell>
    </TableRow>
  );
};

export default BondOfferRow;

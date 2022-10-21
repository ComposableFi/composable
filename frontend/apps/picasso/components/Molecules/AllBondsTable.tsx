import { TokenAsset } from "@/components";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableContainerProps,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import BigNumber from "bignumber.js";
import * as React from "react";
import { humanBalance } from "shared";
import { NoAssetsCover } from "./NoAssetsCover";

export type AllBondsTableProps = TableContainerProps & {
  bonds?: BondOffer[];
  onRowClick: (offerId: string) => void;
};

function getTotalPurchasedInFormat(
  currentBond: {
    totalPurchased: string;
  },
  bondPrice: BigNumber,
  price: BigNumber
) {
  let totalPurchased: number | string = currentBond?.totalPurchased || 0;
  return humanBalance(
    new BigNumber(totalPurchased)
      .multipliedBy(bondPrice)
      .multipliedBy(price)
      .toString()
  );
}

export const AllBondsTable: React.FC<AllBondsTableProps> = ({
  bonds,
  onRowClick = () => {},
  ...rest
}) => {
  if (bonds && bonds.length > 0) {
    return (
      <TableContainer {...rest}>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Asset</TableCell>
              <TableCell align="left">Price</TableCell>
              <TableCell align="left">ROI</TableCell>
              <TableCell align="left">Total Purchased</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {bonds.map((currentBond) => {
              const { bondPrice, asset, price, rewardPrice, bondOfferId } =
                currentBond;
              const roi = new BigNumber(1);

              let totalPurchased = new BigNumber(0);
              return (
                <TableRow
                  sx={{
                    "&:hover": {
                      cursor: "pointer",
                    },
                  }}
                  key={"pica"}
                  onClick={() => onRowClick(String(bondOfferId))}
                >
                  <TableCell align="left">
                    <TokenAsset tokenId="pica" />
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body2">$0</Typography>
                  </TableCell>
                  <TableCell align="left">
                    <Typography
                      variant="body2"
                      color={roi.lt(0) ? "error.main" : "featured.lemon"}
                    >
                      55%
                    </Typography>
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body2">$0</Typography>
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return <NoAssetsCover />;
};

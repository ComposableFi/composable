import * as React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  TableContainerProps,
} from "@mui/material";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenPairAsset } from "../Atom/TokenPairAsset";
import { AllBondsAsset } from "@/stores/defi/polkadot";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { getROI } from "@/stores/defi/polkadot/bonds/utils";

export type AllBondsTableProps = TableContainerProps & {
  bonds?: BondOffer[];
  onRowClick: (offerId: number) => void;
};

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
            {bonds.map(
              (
                {
                  bondPrice,
                  asset,
                  price,
                  rewardPrice,
                  reward: { amount, asset: rewardAsset },
                },
                index
              ) => {
                const roi = getROI(rewardPrice, amount, price, bondPrice);
                return (
                  <TableRow
                    sx={{
                      "&:hover": {
                        cursor: "pointer",
                      },
                    }}
                    key={asset.symbol}
                    onClick={() => onRowClick(index + 1)}
                  >
                    <TableCell align="left">
                      <TokenPairAsset tokenIds={[asset.id, rewardAsset.id]} />
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">
                        ${price.multipliedBy(bondPrice).toFormat(0)}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography
                        variant="body2"
                        color={roi.lt(0) ? "error.main" : "featured.lemon"}
                      >
                        {roi.gt(0) ? "+" : ""}
                        {roi.toFormat(2)}%
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      {/* Uncomment once totalPurchased is clear */}
                      {/*<Typography variant="body2">${totalPurchased}</Typography>*/}
                    </TableCell>
                  </TableRow>
                );
              }
            )}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return <NoAssetsCover />;
};

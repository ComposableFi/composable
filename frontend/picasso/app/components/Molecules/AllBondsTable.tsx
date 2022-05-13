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

export type AllBondsTableProps = TableContainerProps & {
  assets?: AllBondsAsset[];
};

export const AllBondsTable: React.FC<AllBondsTableProps> = ({
  assets,
  ...rest
}) => {
  if (assets && assets.length > 0) {
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
            {assets.map((row) => (
              <TableRow key={row.token.symbol}>
                <TableCell align="left">
                  <TokenPairAsset tokenIds={[row.token.id, row.toToken.id]} />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${row.price}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography
                    variant="body2"
                    color={row.roi < 0 ? "error.main" : "featured.lemon"}
                  >
                    {row.roi > 0 ? "+" : ""}
                    {row.roi}%
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${row.totalPurchased}</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return <NoAssetsCover />;
};

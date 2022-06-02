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
  onRowClick: (asset: AllBondsAsset) => void;
};

export const AllBondsTable: React.FC<AllBondsTableProps> = ({
  assets,
  onRowClick = () => {},
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
            {assets.map(({ token, toToken, price, roi, totalPurchased }) => (
              <TableRow
                sx={{
                  "&:hover": {
                    cursor: "pointer",
                  },
                }}
                key={token.symbol}
                onClick={() =>
                  onRowClick({
                    token,
                    toToken,
                    price,
                    roi,
                    totalPurchased,
                  })
                }
              >
                <TableCell align="left">
                  <TokenPairAsset tokenIds={[token.id, toToken.id]} />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${price}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography
                    variant="body2"
                    color={roi < 0 ? "error.main" : "featured.lemon"}
                  >
                    {roi > 0 ? "+" : ""}
                    {roi}%
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${totalPurchased}</Typography>
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

import * as React from "react";
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
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenAsset, TokenPairAsset } from "@/components";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { getROI } from "@/defi/polkadot/pallets/BondedFinance";
import { humanBalance } from "@/utils/formatters";

export type AllBondsTableProps = TableContainerProps & {
  bonds?: BondOffer[];
  onRowClick: (offerId: string) => void;
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
                  nbOfBonds,
                },
                index
              ) => {
                const roi = getROI(rewardPrice, price);
                return (
                  <TableRow
                    sx={{
                      "&:hover": {
                        cursor: "pointer",
                      },
                    }}
                    key={
                      Array.isArray(asset)
                        ? asset.map((a) => a.symbol).join("+")
                        : asset.symbol
                    }
                    onClick={() => onRowClick(String(index + 1))}
                  >
                    <TableCell align="left">
                      {Array.isArray(asset) && (
                        <TokenPairAsset tokenIds={asset.map(({ id }) => id)} />
                      )}
                      {!Array.isArray(asset) && (
                        <TokenAsset tokenId={asset.id} />
                      )}
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">
                        ${humanBalance(price)}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography
                        variant="body2"
                        color={roi.lt(0) ? "error.main" : "featured.lemon"}
                      >
                        {roi.gt(0) ? "+" : ""}
                        {humanBalance(roi)}%
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

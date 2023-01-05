import * as React from "react";
import {
  Box,
  Button,
  Skeleton,
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
import { humanBalance } from "shared";
import { useQuery } from "@apollo/client";
import { GET_BONDED_FINANCE } from "@/apollo/queries";
import BigNumber from "bignumber.js";

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
  const { loading, data, error } = useQuery(GET_BONDED_FINANCE);

  if (error) {
    console.error(error);
    return (
      <Box>
        <Typography textAlign="center">An error occurred.</Typography>
        <Button onClick={() => window.location.reload()}>Reload</Button>
      </Box>
    );
  }

  if (loading) {
    return (
      <>
        <Typography>...</Typography>
        <Skeleton width={200} height={50} />
      </>
    );
  }
  const bondedFinanceBondOffers = data.bondedFinanceBondOffers;
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
              ({ bondPrice, asset, price, rewardPrice, bondOfferId }) => {
                const roi = getROI(rewardPrice, price);
                let currentBond = bondedFinanceBondOffers.find(
                  (offer: any) => offer.offerId === bondOfferId.toString()
                );
                if (!currentBond) {
                  return null;
                }
                let totalPurchased = getTotalPurchasedInFormat(
                  currentBond,
                  bondPrice,
                  price
                );
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
                    onClick={() => onRowClick(String(bondOfferId))}
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
                      <Typography variant="body2">${totalPurchased}</Typography>
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

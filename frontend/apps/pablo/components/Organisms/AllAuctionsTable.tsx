import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  useTheme,
  TableContainerProps,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import { useRouter } from "next/router";
import React, { useState } from "react";
import { KeyboardArrowDown } from "@mui/icons-material";
import { KeyboardArrowUp } from "@mui/icons-material";
import { AuctionStatusIndicator } from "./auction/AuctionStatusIndicator";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { useAllAuctionVerifiedPools } from "@/defi/hooks/auctions";
import { setAuctionsSlice } from "@/store/auctions/auctions.slice";

export const AllAuctionsTable: React.FC<TableContainerProps> = ({
  ...rest
}) => {
  const { liquidityBootstrappingPools, tableLimit } =
  useAllAuctionVerifiedPools();
  const theme = useTheme();
  const [count, setCount] = useState(tableLimit);

  const router = useRouter();

  const goAuctionDetails = (auction: LiquidityBootstrappingPool) => {
    setAuctionsSlice({ activePool: auction });
    router.push("/auctions/" + auction.id);
  };

  const handleSeeMore = () => {
    setCount(count + tableLimit);
  };

  const handleSeeLess = () => {
    setCount(tableLimit);
  };

  return (
    <TableContainer {...rest}>
      <Table sx={{ minWidth: 420 }} aria-label="auctions table">
        <TableHead>
          <TableRow>
            <TableCell align="left" sx={{ paddingLeft: theme.spacing(4) }}>
              Token
            </TableCell>
            {/* <TableCell align="left">Network</TableCell> */}
            <TableCell align="center">Auction Status</TableCell>
            <TableCell align="right" sx={{ paddingRight: theme.spacing(4) }}>
              Price
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {liquidityBootstrappingPools.slice(0, count).map((lbPool) => (
            <TableRow
              onClick={() => {
                goAuctionDetails(lbPool);
              }}
              key={lbPool.poolId}
              sx={{ cursor: "pointer" }}
            >
              <TableCell align="left" sx={{ padding: theme.spacing(4) }}>
                {lbPool.baseAsset && (
                  <BaseAsset
                    icon={lbPool.baseAsset.icon}
                    label={lbPool.baseAsset.name}
                    LabelProps={{ variant: "body1" }}
                  />
                )}
              </TableCell>
              <TableCell align="center">
                <AuctionStatusIndicator
                  auction={lbPool}
                  justifyContent="center"
                />
              </TableCell>
              <TableCell align="right" sx={{ padding: theme.spacing(4) }}>
                <Typography variant="body1">${lbPool.spotPrice.toFixed(2)}</Typography>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {liquidityBootstrappingPools.length > count && (
        <Box
          onClick={handleSeeMore}
          mt={2}
          display="flex"
          gap={1}
          justifyContent="center"
          sx={{ cursor: "pointer" }}
        >
          <Typography textAlign="center" variant="body2">
            See more
          </Typography>
          <KeyboardArrowDown sx={{ color: theme.palette.primary.main }} />
        </Box>
      )}

      {liquidityBootstrappingPools.length <= count &&
        liquidityBootstrappingPools.length > tableLimit && (
          <Box
            onClick={handleSeeLess}
            mt={2}
            display="flex"
            gap={1}
            justifyContent="center"
            sx={{ cursor: "pointer" }}
          >
            <Typography textAlign="center" variant="body2">
              See Less
            </Typography>
            <KeyboardArrowUp sx={{ color: theme.palette.primary.main }} />
          </Box>
        )}
    </TableContainer>
  );
};

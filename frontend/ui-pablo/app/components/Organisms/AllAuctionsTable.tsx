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
import { useAppSelector } from "@/hooks/store";
import { useRouter } from "next/router";
import React, { useState } from "react";
import { KeyboardArrowDown } from "@mui/icons-material";
import { KeyboardArrowUp } from "@mui/icons-material";
import { AuctionStatusIndicator } from "./auction/AuctionStatusIndicator";
import { getAssetById } from "@/defi/polkadot/Assets";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import { useLiquidityBootstrappingPools } from "@/defi/hooks/auctions";

export const AllAuctionsTable: React.FC<TableContainerProps> = ({
  ...rest
}) => {
  const { liquidityBootstrappingPools, setActiveAuctionsPool } =
    useLiquidityBootstrappingPools();
  const theme = useTheme();
  const limit = useAppSelector((state) => state.auctions.auctionsTableLimit);
  const [count, setCount] = useState(limit);

  const router = useRouter();

  const goAuctionDetails = (auction: LiquidityBootstrappingPool) => {
    setActiveAuctionsPool(auction);
    router.push("/auctions/" + auction.id);
  };

  const handleSeeMore = () => {
    setCount(count + limit);
  };

  const handleSeeLess = () => {
    setCount(limit);
  };

  return (
    <TableContainer {...rest}>
      <Table sx={{ minWidth: 420 }} aria-label="autions table">
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
          {liquidityBootstrappingPools
            .slice(0, count)
            .map((lbPool: LiquidityBootstrappingPool) => (
              <TableRow
                onClick={() => {
                  goAuctionDetails(lbPool);
                }}
                key={lbPool.icon}
                sx={{ cursor: "pointer" }}
              >
                <TableCell align="left" sx={{ padding: theme.spacing(4) }}>
                  <BaseAsset
                    icon={lbPool.icon}
                    label={
                      getAssetById(lbPool.networkId, lbPool.pair.base)?.symbol
                    }
                    LabelProps={{ variant: "body1" }}
                  />
                </TableCell>
                {/* <TableCell align="left">
                  <BaseAsset
                    icon={lbPool.icon}
                    label={getParachainNetwork(lbPool.networkId).name}
                    LabelProps={{ variant: "body1" }}
                  />
                </TableCell> */}
                <TableCell align="center">
                  <AuctionStatusIndicator
                    auction={lbPool}
                    justifyContent="center"
                  />
                </TableCell>
                <TableCell align="right" sx={{ padding: theme.spacing(4) }}>
                  <Typography variant="body1">${lbPool.spotPrice}</Typography>
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
        liquidityBootstrappingPools.length > limit && (
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
function useVerifiedLiquidityBootstrappingPools(): { liquidityBootstrappingPools: any; setActiveAuctionsPool: any; } {
  throw new Error("Function not implemented.");
}


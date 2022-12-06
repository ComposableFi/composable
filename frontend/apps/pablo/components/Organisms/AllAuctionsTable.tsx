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
import { useAuctionSpotPrice } from "@/defi/hooks/auctions";
import { setAuctionsSlice } from "@/store/auctions/auctions.slice";
import { NoPositionsPlaceholder } from "./overview/NoPositionsPlaceholder";
import { PabloLiquidityBootstrappingPool } from "shared";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { useAsset } from "@/defi/hooks";
import BigNumber from "bignumber.js";

export const AuctionPoolRow = ({ pool, onClickAuction }: {
  pool: PabloLiquidityBootstrappingPool,
  onClickAuction: (auction: PabloLiquidityBootstrappingPool) => void
}) => {
  const baseAsset = useAsset(pool?.getPair().getBaseAsset().toString() ?? "-");
  const spotPrice = useAuctionSpotPrice((pool.getPoolId() as BigNumber).toNumber())
  const theme = useTheme();

  return (
    <TableRow
      onClick={() => {
        onClickAuction(pool);
      }}
      key={pool.getPoolId() as string}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left" sx={{ padding: theme.spacing(4) }}>
        {baseAsset && (
          <BaseAsset
            icon={baseAsset.getIconUrl()}
            label={baseAsset.getName()}
            LabelProps={{ variant: "body1" }}
          />
        )}
      </TableCell>
      <TableCell align="center">
        <AuctionStatusIndicator
          labelWithDuration={true}
          auction={pool}
          justifyContent="center"
        />
      </TableCell>
      <TableCell align="right" sx={{ padding: theme.spacing(4) }}>
        <Typography variant="body1">${spotPrice.toFixed(2)}</Typography>
      </TableCell>
    </TableRow>
  )
}

export const AllAuctionsTable: React.FC<TableContainerProps> = ({
  ...rest
}) => {
  const { liquidityBootstrappingPools } = usePoolsSlice();
  const theme = useTheme();
  const [tableLimit, _setTableLimit] = useState(4);
  const [count, setCount] = useState(tableLimit);

  const router = useRouter();

  const goAuctionDetails = (auction: PabloLiquidityBootstrappingPool) => {
    setAuctionsSlice({ activePool: auction });
    router.push("/auctions/" + auction.getPoolId().toString());
  };

  const handleSeeMore = () => {
    setCount(count + tableLimit);
  };

  const handleSeeLess = () => {
    setCount(tableLimit);
  };

  if (liquidityBootstrappingPools.length === 0) {
    return (
      <NoPositionsPlaceholder text="There are no active liquidity bootstrapping pools at the moment." />
    )
  } else {
    return (
      <TableContainer {...rest}>
        <Table sx={{ minWidth: 420 }} aria-label="auctions table">
          <TableHead>
            <TableRow>
              <TableCell align="left" sx={{ paddingLeft: theme.spacing(4) }}>
                Token
              </TableCell>
              <TableCell align="center">Auction Status</TableCell>
              <TableCell align="right" sx={{ paddingRight: theme.spacing(4) }}>
                Price
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {liquidityBootstrappingPools.slice(0, count).map((lbPool) => (
              <AuctionPoolRow onClickAuction={goAuctionDetails} key={lbPool.getPoolId() as string} pool={lbPool} />
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
  }
};

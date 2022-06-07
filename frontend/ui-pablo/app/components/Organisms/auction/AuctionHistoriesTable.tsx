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
  TableCellProps
} from "@mui/material";
import React, { useState } from "react";
import { KeyboardArrowDown } from "@mui/icons-material";
import { KeyboardArrowUp } from "@mui/icons-material";
import { useAppSelector } from "@/hooks/store";
import moment from "moment-timezone";
import { Link } from "@/components/Molecules";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import { getAssetById } from "@/defi/polkadot/Assets";
import { getShortAddress } from "@/utils/string";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export type AuctionHistoriesTableProps = {
  auction: LiquidityBootstrappingPool,
} & TableContainerProps;

export const AuctionHistoriesTable: React.FC<AuctionHistoriesTableProps> = ({
  auction,
  ...rest
}) => {
  const theme = useTheme();
  const limit = useAppSelector((state) => state.auctions.histiriesTableLimit);
  const [count, setCount] = useState(limit);
  const { auctions: { activeLBPHistory } } = useStore();

  const expandable = activeLBPHistory.length > count;
  const collapsable = !expandable && activeLBPHistory.length > limit;

  const handleSeeMoreOrLess = () => {
    expandable
      ? setCount(count + limit)
      : setCount(limit);
  };

  const seeMoreOrLessText = expandable ? "See More" : "See Less";

  const getHistoryLink = (address: string) => {
    return `${address}`;
  };

  return (
    <TableContainer {...rest}>
      <Table sx={{ minWidth: 420 }} aria-label="autions table">
        <TableHead>
          <TableRow>
            <TableCell align="left" sx={{paddingLeft: theme.spacing(4)}}>
              <Typography variant="body1">
                Time
              </Typography>  
            </TableCell>
            <TableCell align="right">
              <Typography variant="body1">
                Type
              </Typography>  
            </TableCell>
            <TableCell align="center">
              <Typography variant="body1">
                Input
              </Typography>  
            </TableCell>
            <TableCell align="center">
              <Typography variant="body1">
                Output
              </Typography>  
            </TableCell>
            <TableCell align="center">
              <Typography variant="body1">
                {`${getAssetById(auction.networkId, auction.pair.base)?.symbol} Price`}
              </Typography>  
            </TableCell>
            <TableCell align="right" sx={{paddingRight: theme.spacing(4)}}>
              <Typography variant="body1">
                Wallet
              </Typography>  
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {activeLBPHistory.slice(0, count).map((history, index) => (
            <TableRow
              key={index}
            >
              <TableCell align="left" sx={{p: 4}}>
                <Box display="flex" alignItems="center" gap={1.5}>
                  <Typography variant="body1">
                    {moment(history.receivedTimestamp).utc().format("MMM DD, YYYY, h:mm A")}
                  </Typography>
                  <Link href={getHistoryLink(history.walletAddress)} target="_blank">
                    <OpenInNewRoundedIcon />
                  </Link>
                </Box>
              </TableCell>
              <TableCell align="right">
                <Typography variant="body1">
                  {history.side}
                </Typography>  
              </TableCell>
              <TableCell align="center">
                <Typography variant="body1">
                  {`${history.quoteAssetAmount} ${getAssetById(auction.networkId, history.quoteAssetId)?.symbol}`}
                </Typography>  
              </TableCell>
              <TableCell align="center">
                <Typography variant="body1">
                  {`${history.baseAssetAmount} ${getAssetById(auction.networkId, history.baseAssetId)?.symbol}`}
                </Typography>  
              </TableCell>
              <TableCell align="center">
                <Typography variant="body1">
                  ${new BigNumber(history.spotPrice).toFixed(4)}
                </Typography>  
              </TableCell>
              <TableCell align="right" sx={{padding: theme.spacing(4)}}>
                <Typography variant="body1">
                  {getShortAddress(history.walletAddress)}
                </Typography>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      {(expandable || collapsable) && (
        <Box
          onClick={handleSeeMoreOrLess}
          mt={2}
          display="flex"
          gap={1}
          justifyContent="center"
          sx={{ cursor: "pointer" }}
        >
          <Typography textAlign="center" variant="body2">
            {seeMoreOrLessText}
          </Typography>
          {expandable ? (
            <KeyboardArrowDown sx={{ color: theme.palette.primary.main }} />
          ) : (
            <KeyboardArrowUp sx={{ color: theme.palette.primary.main }} />
          )}
        </Box>
      )}
    </TableContainer>
  );
};

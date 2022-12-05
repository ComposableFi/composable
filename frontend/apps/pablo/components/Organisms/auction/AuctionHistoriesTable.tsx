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
  TableContainerProps
} from "@mui/material";
import React, { useState } from "react";
import { KeyboardArrowDown } from "@mui/icons-material";
import { KeyboardArrowUp } from "@mui/icons-material";
import { Link } from "@/components/Molecules";
import { getShortAddress, PabloLiquidityBootstrappingPool } from "shared";
import { Asset } from "shared";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import moment from "moment-timezone";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";
import BigNumber from "bignumber.js";

export type AuctionHistoriesTableProps = {
  auction: PabloLiquidityBootstrappingPool,
  baseAsset: Asset,
  quoteAsset: Asset,
  history: PoolTradeHistory[],
  historiesTableLimit?: number,
} & TableContainerProps;

export const AuctionHistoriesTable: React.FC<AuctionHistoriesTableProps> = ({
  historiesTableLimit = 10,
  auction,
  history,
  baseAsset,
  quoteAsset,
  ...rest
}) => {
  const theme = useTheme();
  const limit = historiesTableLimit;
  const [count, setCount] = useState(limit);

  const expandable = history.length > count;
  const collapsible = !expandable && history.length > limit;

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
      <Table sx={{ minWidth: 420 }} aria-label="auctions table">
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
                {`${baseAsset?.getSymbol()} Price`}
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
          {history.slice(0, count).map((history, index) => {
            let historyBase = undefined, historyQuote = undefined;
            if (baseAsset && quoteAsset) {
              if(history.quoteAssetId.toString() === quoteAsset.getPicassoAssetId() as string) {
                historyBase = baseAsset;
                historyQuote = quoteAsset;
              } else {
                historyBase = quoteAsset;
                historyQuote = baseAsset;
              }
            }
            return (
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
                    {`${history.quoteAssetAmount} ${historyQuote?.getSymbol()}`}
                  </Typography>
                </TableCell>
                <TableCell align="center">
                  <Typography variant="body1">
                    {`${history.baseAssetAmount} ${historyBase?.getSymbol()}`}
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
            )
          })}
        </TableBody>
      </Table>

      {(expandable || collapsible) && (
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

import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow
} from "@mui/material";
import { TokenAsset } from "@/components/Atom";
import { formatNumberWithSymbol } from "shared";
import BigNumber from "bignumber.js";
import { APOLLO_ALLOWED_CURRENCIES } from "@/stores/defi/stats/apollo";
import { useApolloStats } from "@/defi/polkadot/hooks/useApolloStats";
import { FC } from "react";

const tableHeaderTitles = ["Asset", "Binance", "Apollo", "Change (24hr)"];

function formatDiff(diff: BigNumber) {
  if (diff) {
    return formatNumberWithSymbol(diff, diff.isGreaterThan(0) ? "+" : "", "%");
  }
  return "-";
}

export const ApolloTable: FC = () => {
  const { binanceAssets, oracleAssets } = useApolloStats();
  return (
    <TableContainer>
      <Table sx={{ minWidth: 420 }} aria-label="apollo table">
        <TableHead>
          <TableRow>
            {tableHeaderTitles.map(title => (
              <TableCell key={title} align="left">
                {title}
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {APOLLO_ALLOWED_CURRENCIES.map(symbol => {
            const binanceValue = binanceAssets[symbol];
            const oracleValue = oracleAssets[symbol];
            const diff = new BigNumber(0); // [todo: subsquid] Replace this with actual value once subsquid is done
            return (
              <TableRow key={symbol}>
                <TableCell align="left">
                  <TokenAsset tokenId={symbol.toLowerCase()} />
                </TableCell>
                <TableCell align="left">
                  <Box
                    display="flex"
                    alignItems={"center"}
                    justifyContent={"start"}
                  >
                    {binanceValue.close ? (
                      <>${binanceValue.close.toFormat().toString()}</>
                    ) : (
                      "-"
                    )}
                  </Box>
                </TableCell>
                <TableCell align="left">
                  {oracleValue.close ? (
                    <>${oracleValue.close.toFormat().toString()}</>
                  ) : (
                    "-"
                  )}
                </TableCell>
                <TableCell
                  align="left"
                  sx={{
                    color: () => {
                      if (!diff) {
                        return "primary";
                      } else if (diff.gt(0)) {
                        return "featured.lemon";
                      }
                      return "error.main";
                    }
                  }}
                >
                  {formatDiff(diff)}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

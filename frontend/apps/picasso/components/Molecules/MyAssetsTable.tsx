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
import { TokenAsset } from "../Atom";
import BigNumber from "bignumber.js";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { useStore } from "@/stores/root";
import { PriceHashMap } from "@/stores/defi/stats/apollo";

export type MyAssetsTableProps = TableContainerProps & {
  tokensToList: TokenId[]
};

function getPrice(map: PriceHashMap, token: TokenMetadata, key: "open" | "close"): BigNumber {
  return !!map[token.symbol] && !!map[token.symbol][key] ? map[token.symbol][key] as BigNumber : new BigNumber(0)
}

export const MyAssetsTable: React.FC<MyAssetsTableProps> = ({ tokensToList }) => {
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const tokenList = Object.values(tokens).filter((x) => (tokensToList.includes(x.id)));
  const balances = useStore(({ substrateBalances }) => substrateBalances.balances.picasso);
  // dont know whether to use binance or our oracle here
  // for now using oracle
  const { binanceAssets, oracleAssets } = useStore(({ statsApollo }) => statsApollo)
  
  if (tokenList && tokenList.length > 0) {
    return (
      <TableContainer>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Asset</TableCell>
              <TableCell align="left">Price</TableCell>
              <TableCell align="left">Balance</TableCell>
              <TableCell align="left">Value</TableCell>
              <TableCell align="left">Change (24hr)</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {tokenList.map((row: TokenMetadata) => {
              const openPrice = getPrice(oracleAssets, row, "open");
              const closePrice = getPrice(oracleAssets, row, "open");
              const change_24hr = openPrice.minus(closePrice).div(openPrice).toNumber()
              const balance = balances[row.id].balance;
              if (row.symbol) {
                return (
                  <TableRow key={row.symbol}>
                    <TableCell align="left">
                      <TokenAsset tokenId={row.id} />
                    </TableCell>
                    <TableCell align="left">
                      {/* Needs work */}
                      <Typography variant="body2">
                        $ {openPrice.toFormat(
                          row.decimalsToDisplay
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Box sx={{ display: "flex" }}>
                        <TokenAsset
                          tokenId={row.id}
                          iconOnly
                          sx={{ width: 36 }}
                        />
                        <Typography variant="body2">
                          {balance.toFormat(
                            row.decimalsToDisplay
                          )}
                          &nbsp;
                          {row.symbol}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">
                        $
                        {balance.times(openPrice).toFormat(
                          row.decimalsToDisplay
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      {/* Needs work */}
                      <Typography
                        variant="body2"
                        color={
                          change_24hr < 0 ? "error.main" : "featured.lemon"
                        }
                      >
                        {change_24hr > 0 ? "+" : ""}
                        {new BigNumber(change_24hr * 100).toFormat(
                          row.decimalsToDisplay
                        )}
                        %
                      </Typography>
                    </TableCell>
                  </TableRow>
                );
              }
            })}
          </TableBody>
        </Table>
      </TableContainer>
    );
  } else {
    return <NoAssetsCover />;
  }
};

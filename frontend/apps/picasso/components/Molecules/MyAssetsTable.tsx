import * as React from "react";
import { useEffect } from "react";
import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableContainerProps,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { useStore } from "@/stores/root";
import { subscribeCoingeckoPrices } from "@/stores/defi/coingecko";
import { useCoingecko } from "coingecko";
import { TokenAsset } from "../Atom/TokenAsset";

export type MyAssetsTableProps = TableContainerProps & {
  tokensToList: TokenId[];
};

export const MyAssetsTable: React.FC<MyAssetsTableProps> = ({
  tokensToList,
}) => {
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const tokenList = Object.values(tokens).filter((x) =>
    tokensToList.includes(x.id)
  );
  const balances = useStore(
    ({ substrateBalances }) => substrateBalances.balances.picasso
  );
  const prices = useCoingecko((state) => state.prices);

  useEffect(() => {
    return subscribeCoingeckoPrices();
  }, []);

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
              const price = prices[row.id].usd;
              const change_24hr = prices[row.id].usd_24h_change;
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
                        $ {price.toFormat(row.decimalsToDisplay)}
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
                          {balance.toFormat(row.decimalsToDisplay)}
                          &nbsp;
                          {row.symbol}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">
                        ${balance.times(price).toFormat(row.decimalsToDisplay)}
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
                        {new BigNumber(change_24hr).toFormat(
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

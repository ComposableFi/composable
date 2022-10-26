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

export type MyAssetsTableProps = TableContainerProps & {
  assets?: any;
};

export const MyAssetsTable: React.FC<MyAssetsTableProps> = ({ assets }) => {
  if (assets && assets.length > 0) {
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
            {assets.map((row: any) => {
              if (row.symbol) {
                return (
                  <TableRow key={row.symbol}>
                    <TableCell align="left">
                      <TokenAsset tokenId={row.tokenId} />
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">
                        $
                        {new BigNumber(row.price).toFormat(
                          row.decimalsToDisplay
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Box sx={{ display: "flex" }}>
                        <TokenAsset
                          tokenId={row.tokenId}
                          iconOnly
                          sx={{ width: 36 }}
                        />
                        <Typography variant="body2">
                          {new BigNumber(row.balance).toFormat(
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
                        {new BigNumber(row.value).toFormat(
                          row.decimalsToDisplay
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography
                        variant="body2"
                        color={
                          row.change_24hr < 0 ? "error.main" : "featured.lemon"
                        }
                      >
                        {row.change_24hr > 0 ? "+" : ""}
                        {new BigNumber(row.change_24hr * 100).toFormat(
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

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
import { TokenPairAsset } from "../Atom/TokenPairAsset";
import { StakingAsset } from "@/stores/defi/polkadot";

export type MyBondsTableProps = TableContainerProps & {
  assets?: StakingAsset[];
};

export const MyStakesTable: React.FC<MyBondsTableProps> = ({ assets }) => {
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
            {assets.map((row) => (
              <TableRow key={row.token.symbol}>
                <TableCell align="left">
                  <TokenPairAsset tokenIds={[row.token.id, row.toToken.id]} />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    ${new BigNumber(row.price).toFormat()}
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Box sx={{ display: "flex" }}>
                    <TokenAsset
                      tokenId={row.token.id}
                      iconOnly
                      sx={{ width: 36 }}
                    />
                    <Typography variant="body2">
                      {new BigNumber(row.balance).toFormat()}&nbsp;LP
                    </Typography>
                  </Box>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    ${new BigNumber(row.value).toFormat()}
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
                    {new BigNumber(row.change_24hr * 100).toFormat()}%
                  </Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  } else {
    return <NoAssetsCover />;
  }
};

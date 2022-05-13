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
import { TokenAsset, BaseAsset } from "../Atom";
import BigNumber from "bignumber.js";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenPairAsset } from "../Atom/TokenPairAsset";
import { BondingAsset } from "@/stores/defi/polkadot";

export type MyBondingsTableProps = TableContainerProps & {
  assets?: BondingAsset[];
};

export const MyBondingsTable: React.FC<MyBondingsTableProps> = ({
  assets,
  ...rest
}) => {
  if (assets && assets.length > 0) {
    return (
      <TableContainer {...rest}>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Asset</TableCell>
              <TableCell align="left">Claimable</TableCell>
              <TableCell align="left">Pending</TableCell>
              <TableCell align="left">Vesting Time</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {assets.map((row) => (
              <TableRow key={row.token.symbol}>
                <TableCell align="left">
                  <TokenPairAsset tokenIds={[row.token.id, row.toToken.id]} />
                </TableCell>
                <TableCell align="left">
                  <BaseAsset
                    icon="/tokens/chaos.svg"
                    label={`${new BigNumber(row.claimable).toFormat()} Chaos`}
                  />
                </TableCell>
                <TableCell align="left">
                  <BaseAsset
                    icon="/tokens/chaos.svg"
                    label={`${new BigNumber(row.pending).toFormat()} Chaos`}
                  />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{row.vesting_time}</Typography>
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

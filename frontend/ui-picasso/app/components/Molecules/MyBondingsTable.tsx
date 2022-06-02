import * as React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  TableContainerProps,
} from "@mui/material";
import { BaseAsset } from "../Atom";
import BigNumber from "bignumber.js";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenPairAsset } from "../Atom/TokenPairAsset";
import { BondingAsset } from "@/stores/defi/polkadot";

export type MyBondingsTableProps = TableContainerProps & {
  assets?: BondingAsset[];
  onRowClick?: (asset: BondingAsset) => void;
};

export const MyBondingsTable: React.FC<MyBondingsTableProps> = ({
  assets,
  onRowClick = () => {},
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
            {assets.map(
              ({ token, toToken, claimable, pending, vesting_time }) => (
                <TableRow
                  sx={{
                    "&:hover": {
                      cursor: "pointer",
                    },
                  }}
                  key={token.symbol}
                  onClick={() =>
                    onRowClick({
                      token,
                      toToken,
                      claimable,
                      pending,
                      vesting_time,
                    })
                  }
                >
                  <TableCell align="left">
                    <TokenPairAsset tokenIds={[token.id, toToken.id]} />
                  </TableCell>
                  <TableCell align="left">
                    <BaseAsset
                      icon="/tokens/chaos.svg"
                      label={`${new BigNumber(claimable).toFormat()} Chaos`}
                    />
                  </TableCell>
                  <TableCell align="left">
                    <BaseAsset
                      icon="/tokens/chaos.svg"
                      label={`${new BigNumber(pending).toFormat()} Chaos`}
                    />
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body2">{vesting_time}</Typography>
                  </TableCell>
                </TableRow>
              )
            )}
          </TableBody>
        </Table>
      </TableContainer>
    );
  } else {
    return <NoAssetsCover />;
  }
};

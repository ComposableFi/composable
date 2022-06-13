import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  BoxProps,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "../BoxWrapper";

import { useAssetsWithBalance } from "@/store/hooks/overview/useAssetsOverview";

const tableHeaders: TableHeader[] = [
  {
    header: "Assets",
  },
  {
    header: "Price",
  },
  {
    header: "Amount",
  },
  {
    header: "Value",
  },
];

export const WalletBreakdownBox: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const walletOverview = useAssetsWithBalance();

  return (
    <BoxWrapper
      title="Wallet Breakdown"
      {...boxProps}
    >
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              {
                tableHeaders.map((th) => (
                  <TableCell key={th.header} align="left">{th.header}</TableCell>
                ))
              }
            </TableRow>
          </TableHead>
          <TableBody>
            {walletOverview.map((asset) => {
              return (
                <TableRow key={asset.assetId}>
                  <TableCell align="left">
                    <BaseAsset
                      label={asset.symbol}
                      icon={asset.icon}
                    />
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">${asset.price.toFixed(2)}</Typography>
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">
                      {asset.balance.toFormat(2)}
                    </Typography>
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">
                      ${asset.balance.multipliedBy(asset.price).toFormat(2)}
                    </Typography>
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};

import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  BoxProps,
  Box,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "../BoxWrapper";
import { useAssetsOverview } from "@/store/hooks/overview/useAssetsOverview";
import { NoPositionsPlaceholder } from "./NoPositionsPlaceholder";
import { OVERVIEW_ERRORS } from "./errors";

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

export const WalletBreakdownBox: React.FC<BoxProps> = ({ ...boxProps }) => {
  const assetsOverview = useAssetsOverview();

  return (
    <BoxWrapper title="Wallet Breakdown" {...boxProps}>
      {assetsOverview.length === 0 && (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_ASSETS} />
      )}

      {assetsOverview.length > 0 && (
        <TableContainer>
          <Table>
            <TableHead>
              <TableRow>
                {tableHeaders.map((th) => (
                  <TableCell key={th.header} align="left">
                    {th.header}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {assetsOverview.map((asset) => {
                return (
                  <TableRow key={asset.name}>
                    <TableCell align="left">
                      <BaseAsset label={asset.symbol} icon={asset.icon} />
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">
                        ${asset.priceUsd.toFixed(2)}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">
                        {asset.balance.toFormat(2)}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">
                        $
                        {asset.balance.multipliedBy(asset.priceUsd).toFormat(2)}
                      </Typography>
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </TableContainer>
      )}
    </BoxWrapper>
  );
};

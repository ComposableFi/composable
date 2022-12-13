import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  BoxProps,
} from "@mui/material";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "../BoxWrapper";
import { usePoolsWithLpBalance } from "@/defi/hooks/overview/usePoolsWithLpBalance";
import { NoPositionsPlaceholder } from "./NoPositionsPlaceholder";
import { OVERVIEW_ERRORS } from "./errors";
import LiquidityProviderPositionRow from "./LiquidityProviderPositionRow";

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
  {
    header: "APR",
  },
];

export const LiquidityProvidersBox: React.FC<BoxProps> = ({ ...boxProps }) => {
  const pools = usePoolsWithLpBalance();

  return (
    <BoxWrapper title="Liquidity provider positions" {...boxProps}>
      {pools.length === 0 && (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_LP} />
      )}

      {pools.length > 0 && (
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
              {pools.map((pool, index) => (
                <LiquidityProviderPositionRow pool={pool} key={index} />
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      )}
    </BoxWrapper>
  );
};

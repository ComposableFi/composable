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
import { PairAsset } from "@/components/Atoms";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "../BoxWrapper";
import { usePoolsWithLpBalance } from "@/store/hooks/overview/usePoolsWithLpBalance";
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
  {
    header: "APR",
  },
];

export const LiquidityProvidersBox: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const liquidityProvided = usePoolsWithLpBalance();

  return (
    <BoxWrapper
      title="Liquidity provider positions"
      {...boxProps}
    >
      {liquidityProvided.length === 0 && (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_BOND} />
      )}

      {liquidityProvided.length > 0 && <TableContainer>
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
            {liquidityProvided.map(({baseAsset, quoteAsset, apr, lpPrice, totalVolume, lpBalance}) => (
              <TableRow key={`${baseAsset?.symbol}-${quoteAsset?.symbol}`}>
                <TableCell align="left">
                  {baseAsset && quoteAsset && 
                  <PairAsset
                    assets={[
                      {icon: baseAsset.icon, label: baseAsset.symbol},
                      {icon: quoteAsset.icon, label: quoteAsset.symbol},
                    ]}
                    separator="/"
                  />
                  }
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">${lpPrice ? lpPrice.toFormat(2) : " - "}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{lpBalance.toFormat(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">
                    ${lpPrice.times(lpBalance).toFormat(2)}
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{apr.toFormat(2)}%</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>}
    </BoxWrapper>
  );
};

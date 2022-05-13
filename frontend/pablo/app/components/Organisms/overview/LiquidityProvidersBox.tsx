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
import { useAppSelector } from "@/hooks/store";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "./BoxWrapper";

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
  const providers = useAppSelector((state) => state.polkadot.yourLiquidityPools);

  return (
    <BoxWrapper
      title="Liquidity provider positions"
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
            {providers.map(({token1, token2, apr, volume, price}) => (
              <TableRow key={`${token1.id}-${token2.id}`}>
                <TableCell align="left">
                  <PairAsset
                    assets={[
                      {icon: token1.icon, label: token1.symbol},
                      {icon: token2.icon, label: token2.symbol},
                    ]}
                    separator="/"
                  />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">${price ? price.toFormat(2) : " - "}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{volume.toFormat(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">
                    ${price ? volume.multipliedBy(price).toFormat(2) : " - "}
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{apr.toFormat(2)}%</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};

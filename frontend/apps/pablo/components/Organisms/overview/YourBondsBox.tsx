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
import { BoxWrapper } from "../BoxWrapper";

const tableHeaders: TableHeader[] = [
  {
    header: "Assets",
  },
  {
    header: "Discount",
  },
  {
    header: "Amount",
  },
  {
    header: "Value",
  },
  {
    header: "Vesting",
  },
  {
    header: "Claimable",
  },
];

export const YourBondsBox: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const bonds = useAppSelector((state) => state.polkadot.yourBondPools);

  return (
    <BoxWrapper
      title="Your Bonds"
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
            {bonds.map(({token1, token2, discount, volume, tvl, vesting_term, claimable}: any) => (
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
                  <Typography variant="body1">${discount.toFormat(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{volume.toFormat(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">
                    ${volume.multipliedBy(discount).toFormat(2)}
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{vesting_term} days</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{claimable.toFormat(2)}</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};

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
import { useAppSelector } from "@/hooks/store";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "./BoxWrapper";
import { getToken } from "@/defi/Tokens";
import moment from "moment-timezone";

const tableHeaders: TableHeader[] = [
  {
    header: "fNFT ID",
  },
  {
    header: "PBLO locked",
  },
  {
    header: "Expiry",
  },
  {
    header: "Multiplier",
  },
  {
    header: "xPBLO",
  },
];

export const XPablosBox: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const xPablos = useAppSelector((state) => state.polkadot.yourXPablos);

  return (
    <BoxWrapper
      title="Your xPBLO"
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
            {xPablos.map(({tokenId, locked, expiry, muliplier, amount}) => (
              <TableRow key={tokenId}>
                <TableCell align="left">
                  <BaseAsset
                    icon={getToken(tokenId).icon}
                    label={getToken(tokenId).symbol}
                  />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{locked.toFormat(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{moment(expiry).utc().format("DD MMM YYYY")}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{muliplier.toFixed(2)}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{amount.toFormat(2)}</Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};

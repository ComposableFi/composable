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
import { TableHeader, XPablo } from "@/defi/types";
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

export type XPablosBoxProps = {
  title?: string,
  header?: TableHeader[],
} & BoxProps;

export const XPablosBox: React.FC<XPablosBoxProps> = ({
  title,
  header,
  ...boxProps
}) => {
  const xPablos = useAppSelector((state) => state.polkadot.yourXPablos);
  const expired = (expiry: number) => expiry < new Date().getTime();

  return (
    <BoxWrapper
      title={title || "Your xPBLO"}
      {...boxProps}
    >
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              {
                (header || tableHeaders).map((th) => (
                  <TableCell key={th.header} align="left">{th.header}</TableCell>
                ))
              }
            </TableRow>
          </TableHead>
          <TableBody>
            {xPablos.map(({tokenId, locked, expiry, multiplier, amount}: XPablo) => (
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
                  <Typography
                    variant="body1"
                    color={expired(expiry) ? "error" : undefined}
                  >
                    {expired(expiry)
                      ? "Expired"
                      : moment(expiry).utc().format("DD MMM YYYY")
                    }
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body1">{multiplier.toFixed(2)}</Typography>
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

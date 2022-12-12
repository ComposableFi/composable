import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Tooltip,
} from "@mui/material";
import React from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import BondOfferRow from "./bonds/BondOfferRow";
import { BondOffer, BondOfferReward } from "shared";
import BigNumber from "bignumber.js";

const tableHeaders: TableHeader[] = [
  {
    header: "Asset",
  },
  {
    header: "Price",
    tooltip: "Price",
  },
  {
    header: "ROI",
    tooltip: "ROI",
  },
  {
    header: "Total purchased",
    tooltip: "Total purchased",
  },
];

const BOND_LIMIT_TO_SHOW = 4;

export const AllBondTable: React.FC = () => {
  const mockedOffer = new BondOffer(
    new BigNumber(1),
    new BigNumber(4),
    "",
    new BigNumber(0),
    new BigNumber(1),
    { finite: { returnIn: new BigNumber(1) } },
    new BondOfferReward(new BigNumber(1), new BigNumber(0), new BigNumber(1))
  );
  return (
    <TableContainer>
      <Table>
        <TableHead>
          <TableRow>
            {tableHeaders.map((th) => (
              <TableCell align="left" key={th.header}>
                <Box display="flex" alignItems="center" gap={1}>
                  {th.header}
                  {th.tooltip && (
                    <Tooltip arrow title={th.tooltip}>
                      <InfoOutlined color="primary" fontSize="small" />
                    </Tooltip>
                  )}
                </Box>
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {[mockedOffer].map((bondOffer) => (
            <BondOfferRow
              offerId={"fNFT 42"}
              key={"fNFT 42"}
              bondOffer={bondOffer}
            />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

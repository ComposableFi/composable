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
import { useBondOffersSlice } from "@/store/bond/bond.slice";
import { OverviewBondedOfferRow } from "./OverviewBondedOfferRow";

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
  const { bondOffers } = useBondOffersSlice();

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
            {bondOffers.map((offer) => (
              <OverviewBondedOfferRow offerId={offer.offerId.toString()} bondOffer={offer} />
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};

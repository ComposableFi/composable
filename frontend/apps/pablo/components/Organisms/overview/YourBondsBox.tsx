import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  BoxProps,
} from "@mui/material";
import React, { useMemo } from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "../BoxWrapper";
import { useBondOffersSlice } from "@/store/bond/bond.slice";
import { OverviewBondedOfferRow } from "./OverviewBondedOfferRow";
import { NoPositionsPlaceholder } from "./NoPositionsPlaceholder";
import { OVERVIEW_ERRORS } from "./errors";

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

export const YourBondsBox: React.FC<BoxProps> = ({ ...boxProps }) => {
  const { bondOffers, bondedOfferVestingSchedules } = useBondOffersSlice();

  const bondedOffers = useMemo(() => {
    return bondOffers.filter((offer) => {
      const offerId = offer.getBondOfferId() as string;
      return (
        offerId in bondedOfferVestingSchedules &&
        bondedOfferVestingSchedules[offerId].length > 0
      );
    });
  }, [bondOffers, bondedOfferVestingSchedules]);

  return (
    <BoxWrapper title="Your Bonds" {...boxProps}>
      {bondedOffers.length === 0 && (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_BOND} />
      )}

      {bondedOffers.length > 0 && (
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
              {bondedOffers.map((offer) => (
                <OverviewBondedOfferRow
                  offerId={offer.getBondOfferId() as string}
                  bondOffer={offer}
                />
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      )}
    </BoxWrapper>
  );
};

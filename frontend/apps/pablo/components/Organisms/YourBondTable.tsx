import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Tooltip,
} from "@mui/material";
import React, { useMemo } from "react";
import { useRouter } from "next/router";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useBondOffersSlice } from "@/store/bond/bond.slice";
import { NoPositionsPlaceholder } from "./overview/NoPositionsPlaceholder";
import BondedOfferRow from "./bonds/BondedOfferRow";
import BigNumber from "bignumber.js";

const tableHeaders: TableHeader[] = [
  {
    header: "Asset",
  },
  {
    header: "Claimable",
    tooltip: "Claimable",
  },
  {
    header: "Pending",
    tooltip: "Pending",
  },
  {
    header: "Vesting time",
    tooltip: "Vesting time",
  },
];

export const YourBondTable: React.FC = () => {
  const { bondOffers, bondedOfferVestingSchedules } = useBondOffersSlice();
  const router = useRouter();

  const myOffers = useMemo(() => {
    return bondOffers.filter((bondOffer) => {
      const offerId = bondOffer.getBondOfferId() as string;
      return (
        offerId in bondedOfferVestingSchedules &&
        bondedOfferVestingSchedules[offerId].length > 0
      );
    });
  }, [bondOffers, bondedOfferVestingSchedules]);
  
  const handleRowClick = (offerId: number) => {
    router.push(`/bond/select/${offerId}`);
  };

  if (myOffers.length == 0) {
    return (
      <NoPositionsPlaceholder text="You currently do not have any active bonds." />
    );
  } else {
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
            {myOffers.map((bond) => (
              <BondedOfferRow
                key={bond.getBondOfferId() as string}
                bondOffer={bond}
                handleBondedOfferRowClick={() =>
                  handleRowClick((bond.getBondOfferId() as BigNumber).toNumber())
                }
              />
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
};

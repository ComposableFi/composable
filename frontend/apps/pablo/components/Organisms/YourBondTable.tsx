import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  Tooltip,
} from "@mui/material";
import Image from "next/image";
import { useRouter } from "next/router";
import React, { useMemo } from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import BondedOfferRow from "./bonds/BondedOfferRow";
import { useBondOffersSlice } from "@/store/bond/bond.slice";

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
      const offerId = bondOffer.offerId.toString();
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
      <Box textAlign="center" mt={3}>
        <Image
          src="/static/lemonade.png"
          css={{ mixBlendMode: "luminosity" }}
          width="96"
          height="96"
          alt="lemonade"
        />
        <Typography variant="body2" paddingTop={4} color="text.secondary">
          You currently do not have any active bonds.
        </Typography>
      </Box>
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
                key={bond.offerId.toString()}
                bondOffer={bond}
                handleBondedOfferRowClick={() =>
                  handleRowClick(bond.offerId.toNumber())
                }
              />
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return null;
};

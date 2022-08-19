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
import React, { useEffect, useMemo, useState } from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import {
  createBondOfferIdVestingScheduleIdMap,
  DEFAULT_NETWORK_ID,
} from "@/defi/utils";
import { fetchVestingSchedulesAdded } from "@/defi/subsquid/bonds/helpers";
import useStore from "@/store/useStore";
import BondedOfferRow from "./bonds/BondedOfferRow";

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
  const { bondOffers, putBondedOffers } = useStore();
  const { list, bondedOffers } = bondOffers;
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const router = useRouter();

  useEffect(() => {
    if (selectedAccount && parachainApi) {
      fetchVestingSchedulesAdded(selectedAccount.address).then(
        (addedEvents) => {
          putBondedOffers(
            createBondOfferIdVestingScheduleIdMap(parachainApi, addedEvents)
          );
        }
      );
    }
  }, [selectedAccount, parachainApi, putBondedOffers]);

  const handleRowClick = (offerId: number) => {
    router.push(`/bond/select/${offerId}`);
  };

  const myOffers = useMemo(() => {
    return list.filter((bondOffer) => {
      const offerId = bondOffer.offerId.toString();
      return offerId in bondedOffers;
    });
  }, [list, bondedOffers]);

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

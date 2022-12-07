import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  useTheme,
  Tooltip,
} from "@mui/material";
import React, { useEffect, useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useRouter } from "next/router";
import {
  DEFAULT_NETWORK_ID,
  fetchVestingSchedulesByBondOffers,
} from "@/defi/utils";
import {
  extractUserBondedFinanceVestingScheduleAddedEvents,
  fetchTotalPurchasedBondsByOfferIds,
} from "@/defi/subsquid/bonds/helpers";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import BondOfferRow from "./bonds/BondOfferRow";
import {
  putBondedOfferBondedVestingScheduleIds,
  putBondOffersTotalPurchasedCount,
  putBondedOfferVestingSchedules,
  useBondOffersSlice,
  putBondOffers,
} from "@/store/bond/bond.slice";
import { NoPositionsPlaceholder } from "./overview/NoPositionsPlaceholder";
import { BondOffer } from "shared";

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
  const theme = useTheme();
  const router = useRouter();

  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { bondOffers, bondedOfferVestingScheduleIds } = useBondOffersSlice();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (!parachainApi) return;
    BondOffer.fetchBondOffers(parachainApi).then(putBondOffers);
  }, [parachainApi]);

  useEffect(() => {
    fetchTotalPurchasedBondsByOfferIds().then(
      putBondOffersTotalPurchasedCount
    );
  }, []);

  useEffect(() => {
    if (selectedAccount && parachainApi) {
      extractUserBondedFinanceVestingScheduleAddedEvents(
        parachainApi,
        selectedAccount.address
      ).then(putBondedOfferBondedVestingScheduleIds);
    }
  }, [selectedAccount, parachainApi]);

  useEffect(() => {
    if (selectedAccount && parachainApi) {
      fetchVestingSchedulesByBondOffers(
        parachainApi,
        selectedAccount.address,
        bondOffers,
        bondedOfferVestingScheduleIds
      ).then(putBondedOfferVestingSchedules)
    }
  }, [selectedAccount, parachainApi, bondOffers, bondedOfferVestingScheduleIds]);

  const [count, setCount] = useState(BOND_LIMIT_TO_SHOW);
  const handleSeeMore = () => {
    setCount(count + BOND_LIMIT_TO_SHOW);
  };

  const handleBondClick = (offerId: string) => {
    router.push(`bond/select/${offerId}`);
  };

  if (bondOffers.length === 0) {
    return (
      <NoPositionsPlaceholder text="There no bond offers active at the moment." />
    )
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
            {bondOffers.slice(0, count).map((bondOffer) => (
              <BondOfferRow
                offerId={bondOffer.getBondOfferId() as string}
                key={bondOffer.getBondOfferId() as string}
                bondOffer={bondOffer}
                handleBondClick={handleBondClick}
              />
            ))}
          </TableBody>
        </Table>
        {bondOffers.length > count && (
          <Box
            onClick={handleSeeMore}
            mt={4}
            display="flex"
            gap={1}
            justifyContent="center"
            sx={{ cursor: "pointer" }}
          >
            <Typography textAlign="center" variant="body2">
              See more
            </Typography>
            <KeyboardArrowDown sx={{ color: theme.palette.primary.main }} />
          </Box>
        )}
      </TableContainer>
    );
  }
};

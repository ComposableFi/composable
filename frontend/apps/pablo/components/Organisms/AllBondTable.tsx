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
import { BaseAsset, PairAsset } from "../Atoms";
import React, { useCallback, useEffect, useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { BondOffer, BondPrincipalAsset, TableHeader } from "@/defi/types";
import { useRouter } from "next/router";
import useBondOfferROI from "@/defi/hooks/bonds/useBondOfferROI";
import useBondOfferPrincipalAsset from "@/defi/hooks/bonds/useBondOfferPrincipalAsset";
import useTotalPurchasedBondOffer from "@/defi/hooks/bonds/useTotalPurchased";
import useBondPrice from "@/defi/hooks/bonds/useBondPrice";
import useStore from "@/store/useStore";
import { DEFAULT_NETWORK_ID, fetchBondOffers } from "@/defi/utils";
import { fetchTotalPurchasedBondsByOfferIds } from "@/defi/subsquid/bonds/helpers";
import { useParachainApi } from "substrate-react";

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

const BondPrincipalAssetIcon = ({
  principalAsset,
}: {
  principalAsset: BondPrincipalAsset;
}) => {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  if (baseAsset && quoteAsset) {
    return (
      <PairAsset
        assets={[
          {
            icon: baseAsset.icon,
            label: baseAsset.symbol,
          },
          {
            icon: quoteAsset.icon,
            label: quoteAsset.symbol,
          },
        ]}
        separator="/"
      />
    );
  }

  if (simplePrincipalAsset) {
    return (
      <BaseAsset
        label={simplePrincipalAsset.symbol}
        icon={simplePrincipalAsset.icon}
      />
    );
  }

  return null;
};

const BondOfferRow = ({
  bondOffer,
  handleBondClick,
}: {
  bondOffer: BondOffer;
  handleBondClick: (bondOfferId: string) => void;
}) => {
  const roi = useBondOfferROI(bondOffer);
  const totalPurchasedValue = useTotalPurchasedBondOffer(bondOffer);
  const principalAsset = useBondOfferPrincipalAsset(bondOffer);
  const bondPrice = useBondPrice(bondOffer);

  return (
    <TableRow
      key={bondOffer.offerId.toString()}
      onClick={() => handleBondClick(bondOffer.offerId.toString())}
      sx={{ cursor: "pointer" }}
    >
      <TableCell align="left">
        <BondPrincipalAssetIcon principalAsset={principalAsset} />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">${bondPrice.toFormat()}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2" color="featured.main">
          {roi.toFormat()}%
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body2">
          ${totalPurchasedValue.toFormat()}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export const AllBondTable: React.FC = () => {
  const theme = useTheme();
  const router = useRouter();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const {
    bondOffers: { list },
    setBondOffers,
    setBondOfferTotalPurchased,
  } = useStore();

  useEffect(() => {
    if (parachainApi) {
      fetchBondOffers(parachainApi).then(setBondOffers);
    }
  }, [parachainApi, setBondOffers]);

  useEffect(() => {
    fetchTotalPurchasedBondsByOfferIds().then(
      setBondOfferTotalPurchased
    );
  }, [setBondOfferTotalPurchased]);

  const [count, setCount] = useState(BOND_LIMIT_TO_SHOW);
  const handleSeeMore = () => {
    setCount(count + BOND_LIMIT_TO_SHOW);
  };

  const handleBondClick = (offerId: string) => {
    router.push(`bond/select/${offerId}`);
  };

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
          {list.slice(0, count).map((bondOffer, index) => (
            <BondOfferRow
              key={bondOffer.offerId.toString()}
              bondOffer={bondOffer}
              handleBondClick={handleBondClick}
            />
          ))}
        </TableBody>
      </Table>
      {list.length > count && (
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
};
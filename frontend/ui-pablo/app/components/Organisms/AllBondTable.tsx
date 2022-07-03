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
import React, { useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useRouter } from "next/router";
import useBondOffers from "@/defi/hooks/bonds/useBondOffers";

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
  const bondOffers = useBondOffers();
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
          {bondOffers.slice(0, count).map((bond, index) => (
            <TableRow
              key={index}
              onClick={() => handleBondClick(bond.offerId.toString())}
              sx={{ cursor: "pointer" }}
            >
              <TableCell align="left">
                {bond.principalAsset?.lpPrincipalAsset?.baseAsset && bond.principalAsset?.lpPrincipalAsset?.quoteAsset ? (
                  <PairAsset
                    assets={[
                      {
                        icon: bond.principalAsset.lpPrincipalAsset.baseAsset.icon,
                        label: bond.principalAsset.lpPrincipalAsset.baseAsset.symbol,
                      },
                      {
                        icon: bond.principalAsset.lpPrincipalAsset.quoteAsset.icon,
                        label: bond.principalAsset.lpPrincipalAsset.quoteAsset.symbol,
                      },
                    ]}
                    separator="/"
                  />
                ) : bond.principalAsset.simplePrincipalAsset ? (
                    <BaseAsset label={bond.principalAsset.simplePrincipalAsset.symbol} icon={bond.principalAsset.simplePrincipalAsset.icon} />
                ) : null}
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2">
                  ${bond.bondPrice.toFormat()}
                </Typography>
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2" color="featured.main">
                  {bond.roi.toFormat()}%
                </Typography>
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2">
                  ${bond.totalPurchased.toFormat()}
                </Typography>
              </TableCell>
            </TableRow>
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
};

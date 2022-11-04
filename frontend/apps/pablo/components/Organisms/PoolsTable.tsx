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
  BoxProps,
} from "@mui/material";

import { useRouter } from "next/router";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { NoPositionsPlaceholder } from "./overview/NoPositionsPlaceholder";
import { PabloConstantProductPool } from "shared";
import LiquidityPoolRow from "./pool/LiquidityPoolRow";
import React, { useState } from "react";

enum EMPTY_INFO_MESSAGES {
  USER_NO_POOL = "You currently do not have any active liquidity pool.",
  NO_POOL_EXISTS = "Liquidity pools are not available at the moment."
}

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "TVL",
    tooltip: "Total value locked",
  },
  {
    header: "APY",
    tooltip: "APY",
  },
  {
    header: "Daily Rewards",
    tooltip: "Daily Rewards",
  },
  {
    header: "Volume",
    tooltip: "Volume",
  },
];

export type PoolsTableProps = {
  liquidityPools: Array<PabloConstantProductPool>;
  source: "user" | "pallet"
};

const SEE_MORE_OFFSET = 5;

export const PoolsTable: React.FC<PoolsTableProps> = ({
  liquidityPools,
  source
}) => {
  const theme = useTheme();
  const [startIndex, setStartIndex] = useState(0);
  const router = useRouter();

  const handleRowClick = (e: React.MouseEvent, poolId: string) => {
    e.preventDefault();
    router.push(`/pool/select/${poolId}`);
  };

  const handleSeeMore = () => {
    setStartIndex(startIndex + SEE_MORE_OFFSET);
  };

  if (liquidityPools.length === 0) {
    return (
      <NoPositionsPlaceholder text={
        source === "user" ? EMPTY_INFO_MESSAGES.USER_NO_POOL : EMPTY_INFO_MESSAGES.NO_POOL_EXISTS
      } />
    );
  } else if (liquidityPools.length > 0) {
    return (
      <TableContainer>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              {tableHeaders.map((th) => (
                <TableCell key={th.header} align="left">
                  <Box display="flex" alignItems="center" gap={1.75}>
                    <Typography variant="body1">{th.header}</Typography>
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
            {liquidityPools.map((row, index) => (
              <LiquidityPoolRow
                liquidityPool={row}
                key={index}
                handleRowClick={handleRowClick}
              />
            ))}
          </TableBody>
        </Table>

        {liquidityPools.length > startIndex + SEE_MORE_OFFSET && <Box
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
        </Box>}
      </TableContainer>
    );
  }
  return null;
};

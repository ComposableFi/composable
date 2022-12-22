import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";

import { useRouter } from "next/router";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import React, { FC, useState } from "react";
import { OwnedLiquidityTokens, PoolId } from "@/store/pools/types";
import BigNumber from "bignumber.js";
import { Asset } from "shared";
import { PairAsset } from "@/components";
import { NoPositionsPlaceholder } from "../overview/NoPositionsPlaceholder";
import { fromChainUnits } from "@/defi/utils";

const USER_NO_POOL = "You currently do not have any active liquidity pool.";

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "",
  },
  {
    header: "",
  },
  {
    header: "",
  },
  {
    header: "Balance",
  },
];

export type YourLiquidityTableProps = {
  tokens: OwnedLiquidityTokens;
};

const SEE_MORE_OFFSET = 5;

export const YourLiquidityTable: FC<YourLiquidityTableProps> = ({ tokens }) => {
  const theme = useTheme();
  const [startIndex, setStartIndex] = useState(0);
  const router = useRouter();
  const noLiquidityToken = Object.keys(tokens).length === 0;

  const handleRowClick = (poolId: PoolId) => {
    router.push(`/pool/select/${poolId.toString()}`);
  };

  const handleSeeMore = () => {
    setStartIndex(startIndex + SEE_MORE_OFFSET);
  };

  if (noLiquidityToken) {
    return <NoPositionsPlaceholder text={USER_NO_POOL} />;
  }

  return (
    <TableContainer>
      <Table sx={{ minWidth: 420 }} aria-label="All Liquidity table">
        <TableHead>
          <TableRow>
            {tableHeaders.map((th, index) => (
              <TableCell key={index} align="left">
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
          {Object.values(tokens)
            .filter((token) =>
              token.balance.free.isGreaterThanOrEqualTo(fromChainUnits(1))
            )
            .map(({ balance, poolId, pair }) => (
              <OwnedLiquidityPoolRow
                poolId={poolId}
                key={`owned_liquidity_${poolId.toString()}`}
                pair={pair}
                onClick={handleRowClick}
                balance={balance}
              />
            ))}
        </TableBody>
      </Table>

      {Object.keys(tokens).length > startIndex + SEE_MORE_OFFSET && (
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
type OwnedLiquidityRowProps = {
  onClick: (poolId: PoolId) => void;
  balance: {
    free: BigNumber;
    locked: BigNumber;
  };
  pair: [Asset, Asset];
  poolId: PoolId;
};

const OwnedLiquidityPoolRow: FC<OwnedLiquidityRowProps> = ({
  pair,
  balance,
  poolId,
  onClick,
}) => {
  return (
    <TableRow onClick={() => onClick(poolId)} sx={{ cursor: "pointer" }}>
      <TableCell align="left">
        <PairAsset assets={pair} separator="/" />
      </TableCell>
      <TableCell align="left"></TableCell>
      <TableCell align="left"></TableCell>
      <TableCell align="left"></TableCell>
      <TableCell align="left">
        <Typography variant="body2">{balance.free.toFormat(4)}</Typography>
      </TableCell>
    </TableRow>
  );
};

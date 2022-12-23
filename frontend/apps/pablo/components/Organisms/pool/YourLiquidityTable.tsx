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
import React, { FC, useEffect, useState } from "react";
import { OwnedLiquidityTokens, PoolConfig, PoolId } from "@/store/pools/types";
import { PairAsset } from "@/components";
import { NoPositionsPlaceholder } from "../overview/NoPositionsPlaceholder";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import useStore from "@/store/useStore";
import { subscribeOwnedLiquidity } from "@/store/pools/subscribeOwnedLiquidity";
import {
  useDotSamaContext,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import BigNumber from "bignumber.js";

const USER_NO_POOL = "You currently do not have any active liquidity pool.";

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "Total value locked",
  },
  {
    header: "Volume",
  },
  {
    header: "Balance",
  },
];

export type YourLiquidityTableProps = {
  pools: PoolConfig[];
};

const SEE_MORE_OFFSET = 5;

export const YourLiquidityTable: FC<YourLiquidityTableProps> = ({ pools }) => {
  const theme = useTheme();
  const [startIndex, setStartIndex] = useState(0);
  const router = useRouter();
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const noLiquidityToken = Object.keys(userOwnedLiquidity).length === 0;
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { extensionStatus } = useDotSamaContext();

  useEffect(() => {
    let unsub: any = undefined;
    if (parachainApi && selectedAccount && extensionStatus === "connected") {
      unsub = subscribeOwnedLiquidity(parachainApi, selectedAccount.address);
    }

    return () => {
      unsub?.();
    };
  }, [extensionStatus, selectedAccount, parachainApi]);

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
          {pools.map((pool) => (
            <OwnedLiquidityPoolRow
              pool={pool}
              key={`owned_liquidity_${pool.poolId.toString()}`}
              onClick={handleRowClick}
              userLiquidity={userOwnedLiquidity}
            />
          ))}
        </TableBody>
      </Table>
      {Object.keys(userOwnedLiquidity).length >
        startIndex + SEE_MORE_OFFSET && (
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
  pool: PoolConfig;
  userLiquidity: OwnedLiquidityTokens;
};

const OwnedLiquidityPoolRow: FC<OwnedLiquidityRowProps> = ({
  pool,
  onClick,
  userLiquidity,
}) => {
  const pair = pool.config.assets;
  const balance = userLiquidity[pool.config.lpToken]?.balance ?? {
    free: new BigNumber(0),
    locked: new BigNumber(0),
  };

  const { userVolume, userTVL } = usePoolRatio(pool);
  if (!userLiquidity[pool.config.lpToken]) {
    return <TableRow></TableRow>;
  }
  return (
    <TableRow onClick={() => onClick(pool.poolId)} sx={{ cursor: "pointer" }}>
      <TableCell align="left">
        <PairAsset assets={pair} separator="/" />
      </TableCell>
      <TableCell align="left">${userTVL.toFormat(0)}</TableCell>
      <TableCell align="left">${userVolume.toFormat(0)}</TableCell>
      <TableCell align="left">
        <Typography variant="body2">{balance.free.toFormat(4)}</Typography>
      </TableCell>
    </TableRow>
  );
};

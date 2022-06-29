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
import Image from "next/image";
import { PairAsset } from "../Atoms";
import { useAppDispatch } from "@/hooks/store";
import { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useLiquidityPoolsListWithOpenPositions } from "@/store/hooks/useLiquidityPoolsListWithOpenPositions";
import { useLiquidityPoolsList } from "@/store/hooks/useLiquidityPoolsList";

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "TVL",
    tooltip: "TVL",
  },
  {
    header: "ROI",
    tooltip: "ROI",
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

export type AllLiquidityTableProps = {
  flow: "all" | "user";
};

export const AllLiquidityTable: React.FC<AllLiquidityTableProps> = ({
  flow,
}) => {
  const userPools = useLiquidityPoolsListWithOpenPositions();
  const list = useLiquidityPoolsList();
  let pools: ReturnType<typeof useLiquidityPoolsList>;
  const dispatch = useAppDispatch();
  const theme = useTheme();
  const [startIndex, setStartIndex] = useState(0);

  const [showNoPools, setShowNoPools] = useState(true);

  if (flow === "all") {
    pools = list;
  } else {
    pools = userPools;
  }

  const router = useRouter();

  const handleRowClick = (e: React.MouseEvent, poolId: number) => {
    e.preventDefault();
    router.push(`/pool/select/${poolId}`);
  };

  const handleSeeMore = () => {
    setStartIndex(startIndex + 4);
  };

  useEffect(() => {
    if (!userPools.length) {
      setShowNoPools(true);
    } else {
      setShowNoPools(false);
    }
  }, [userPools]);

  if (flow === "user" && showNoPools) {
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
          You currently do not have any active liquidity pool.
        </Typography>
      </Box>
    );
  } else if (Array.isArray(pools) && pools.length > 0) {
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
            {pools.map((row, index) => (
              <TableRow
                onClick={(e) => {
                  handleRowClick(e, row.poolId);
                }}
                key={index}
                sx={{ cursor: "pointer" }}
              >
                <TableCell align="left">
                  {row.baseAsset && row.quoteAsset && (
                    <PairAsset
                      assets={[
                        {
                          icon: row.baseAsset.icon,
                          label: row.baseAsset.symbol,
                        },
                        {
                          icon: row.quoteAsset.icon,
                          label: row.quoteAsset.symbol,
                        },
                      ]}
                      separator="/"
                    />
                  )}
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    ${row.totalValueLocked.toFixed(2)}
                  </Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{row.apr.toFixed(2)}%</Typography>
                </TableCell>
                <TableCell align="left">
                  {row.dailyRewards.map((item) => {
                    return (
                      <Box key={item.assetId} display="flex">
                        <PairAsset
                          assets={[
                            {
                              icon: item.icon,
                              label: item.symbol,
                            },
                          ]}
                          label={item.rewardAmount}
                        />
                      </Box>
                    );
                  })}
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    ${row.totalVolume.toFixed(2)}
                  </Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        {flow === "all" && (
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
  return null;
};

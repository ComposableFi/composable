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
import BigNumber from "bignumber.js";
import { PairAsset } from "../Atoms";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import {
  addNextDataLiquidityPools,
  LiquidityPoolRow,
} from "@/stores/defi/polkadot";
import { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";

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
  let pools: LiquidityPoolRow[];
  const dispatch = useAppDispatch();
  const theme = useTheme();
  const [startIndex, setStartIndex] = useState(0);
  const polkaDotState = useAppSelector((state) => state.polkadot);
  const [showNoPools, setShowNoPools] = useState(true);

  if (flow === "all") {
    pools = polkaDotState.allLiquidityPools;
  } else {
    pools = polkaDotState.yourLiquidityPools;
  }

  const router = useRouter();

  const handleRowClick = (e: React.MouseEvent) => {
    e.preventDefault();
    if (flow === "user") {
      router.push("/pool-select");
    }
  };

  const handleSeeMore = () => {
    dispatch(addNextDataLiquidityPools({ startIndex: startIndex + 4 }));
    setStartIndex(startIndex + 4);
  };

  useEffect(() => {
    dispatch(addNextDataLiquidityPools({ startIndex }));
  }, []);

  useEffect(() => {
    if (flow === "user") {
      setTimeout(() => {
        setShowNoPools(false);
      }, 5000);
    }
  }, []);

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
                        <InfoOutlined color="primary" fontSize="small"/>
                      </Tooltip>
                    )}
                  </Box>
                </TableCell>
                )
              )}
            </TableRow>
          </TableHead>
          <TableBody>
            {pools.map((row, index) => (
              <TableRow onClick={handleRowClick} key={index} sx={{cursor: "pointer"}}>
                <TableCell align="left">
                  <PairAsset
                    assets={[
                      { icon: row.token1.icon, label: row.token1.symbol },
                      { icon: row.token2.icon, label: row.token2.symbol },
                    ]}
                    separator="/"
                  />
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${row.tvl.toFormat()}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{row.apr.toFormat()}%</Typography>
                </TableCell>
                <TableCell align="left">
                  {row.rewardsLeft.map((item) => {
                    return (
                      <Box key={item.token.id} display="flex">
                        <PairAsset
                          assets={[
                            {
                              icon: item.token.icon,
                              label: item.token.symbol,
                            },
                          ]}
                          label={item.value.toFormat(2)}
                        />
                      </Box>
                    );
                  })}
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    ${row.volume.toFormat()}
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

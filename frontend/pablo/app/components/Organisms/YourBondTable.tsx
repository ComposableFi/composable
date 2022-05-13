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
  useTheme,
} from "@mui/material";
import Image from "next/image";
import { BaseAsset, PairAsset } from "../Atoms";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { addNextDataBondPools, YourBondPoolRow } from "@/stores/defi/polkadot";
import { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "TVL",
    tooltip: "",
  },
  {
    header: "APR",
    tooltip: "",
  },
  {
    header: "Bond",
    tooltip: "",
  },
  { header: "Volume", tooltip: "" },
];

export const YourBondTable: React.FC = () => {
  let pools: YourBondPoolRow[];
  const theme = useTheme();
  const dispatch = useAppDispatch();
  const [startIndex, setStartIndex] = useState(0);
  const polkaDotState = useAppSelector((state) => state.polkadot);
  const [showNoPools, setShowNoPools] = useState(true);

  const router = useRouter();

  pools = polkaDotState.yourBondPools;

  const handleRowClick = (e: React.MouseEvent) => {
    router.push("/bond/select");
  };

  const handleSeeMore = () => {
    dispatch(addNextDataBondPools({ startIndex: startIndex + 4 }));
    setStartIndex(startIndex + 4);
  };

  useEffect(() => {
    dispatch(addNextDataBondPools({ startIndex }));
  }, []);

  useEffect(() => {
    setTimeout(() => {
      setShowNoPools(false);
    }, 5000);
  }, []);

  if (showNoPools) {
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
  } else if (Array.isArray(pools) && pools.length > 0) {
    return (
      <TableContainer>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              {tableHeaders.map((th) => {
                return typeof th.tooltip === "string" ? (
                  <TableCell align="left">
                    <Box display="flex" alignItems="center" gap={1}>
                      {th.header}
                      <Tooltip arrow title={th.tooltip}>
                        <InfoOutlined
                          sx={{
                            height: 20,
                            width: 20,
                            color: theme.palette.primary.main,
                          }}
                        />
                      </Tooltip>
                    </Box>
                  </TableCell>
                ) : (
                  <TableCell align="left">{th.header}</TableCell>
                );
              })}
            </TableRow>
          </TableHead>
          <TableBody>
            {pools.map((row) => (
              <TableRow onClick={handleRowClick} key={row.token1.symbol}>
                <TableCell align="left">
                  {row.token2 ? (
                    <PairAsset
                      assets={[
                        { icon: row.token1.icon, label: row.token1.symbol },
                        { icon: row.token2.icon, label: row.token2.symbol },
                      ]}
                      separator="/"
                    />
                  ) : (
                    <BaseAsset
                      label={row.token1.symbol}
                      icon={row.token1.icon}
                    />
                  )}
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">${row.tvl.toFormat()}</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{row.apr.toFormat()}%</Typography>
                </TableCell>
                <TableCell align="left">
                  {row.bond.map((item) => {
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
      </TableContainer>
    );
  }
  return null;
};
